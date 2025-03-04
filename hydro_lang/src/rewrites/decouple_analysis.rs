use std::cell::RefCell;
use std::collections::HashMap;

use grb::prelude::*;

use crate::ir::*;
use crate::location::LocationId;

use super::analyze_send_recv_overheads::{get_network_type, NetworkType};

/// Each operator is assigned either 0 or 1
/// 0 means that its output will go to the original node, 1 means that it will go to the decoupled node
/// If there are two operators, map() -> filter(), and they are assigned variables 0 and 1, then that means filter's result ends up on a different machine.
/// The original machine still executes filter(), but pays a serializaton cost for the output of filter.
/// The decoupled machine executes the following operator and pays a deserialization cost for the output of filter.
/// Each operator is executed on the machine indicated by its INPUT's variable.
///
/// Constraints:
/// 1. For binary operators, both inputs must be assigned the same var (output to the same location)
/// 2. For Tee, the serialization/deserialization cost is paid only once if multiple branches are assigned different vars. (instead of sending each branch of the Tee over to a different node, we'll send it once and have the receiver Tee)
///
/// HydroNode::Network:
/// 1. If we are the receiver, then create a var, we pay deserialization
/// 2. If we are the sender (and not the receiver), don't create a var (because the destination is another machine and can't change), but we still pay serialization
/// 3. If we are the sender and receiver, then create a var, pay for both

struct ModelMetadata {
    // Const fields
    cluster_to_decouple: LocationId,
    decoupling_send_overhead: f64, /* CPU usage per cardinality to send, assuming all messages serialize/deserialize similarly */
    decoupling_recv_overhead: f64,
    // Model variables to construct final cost function
    model: Model,
    orig_node_cpu_usage: Expr,
    decoupled_node_cpu_usage: Expr,
    op_id_to_var: HashMap<usize, Var>,
    op_id_to_inputs: HashMap<usize, Vec<usize>>,
    prev_op_input_with_tick: HashMap<usize, usize>, // tick_id: last op_id with that tick_id
    tee_inner_to_decoupled_vars: HashMap<usize, (Var, Var)>, /* inner_id: (orig_to_decoupled, decoupled_to_orig) */
    orig_to_decoupled_vars: Vec<(usize, Var)>, // Vars representing whether the output of op is a network edge to the decouple node (op, var)
    decoupled_to_orig_vars: Vec<(usize, Var)>, // Vars representing whether the output of op is a network edge to the original node (op, var)
}

// Penalty for decoupling regardless of cardinality (to prevent decoupling low cardinality operators)
const DECOUPLING_PENALTY: f64 = 0.0001;

// Lazily creates the var
fn var_from_op_id(op_id: usize, op_id_to_var: &mut HashMap<usize, Var>, model: &mut Model) -> Var {
    *op_id_to_var.entry(op_id).or_insert_with(|| {
        let name = op_id.to_string();
        add_binvar!(model, name: &name, bounds: ..).unwrap()
    })
}

fn add_equality_constr(
    ops: &Vec<usize>,
    op_id_to_var: &mut HashMap<usize, Var>,
    model: &mut Model,
) {
    if let Some(mut prev_op) = ops.first() {
        for op in ops.iter().skip(1) {
            let prev_op_var = var_from_op_id(*prev_op, op_id_to_var, model);
            let op_var = var_from_op_id(*op, op_id_to_var, model);
            model
                .add_constr(&format!("eq_{}_{}", prev_op, op), c!(prev_op_var == op_var))
                .unwrap();
            prev_op = op;
        }
    }
}

fn relevant_inputs(input_metadatas: Vec<&HydroIrMetadata>, cluster_to_decouple: &LocationId) -> Vec<usize> {
    input_metadatas.iter()
        .filter_map(|input_metadata| {
            if cluster_to_decouple == input_metadata.location_kind.root() {
                Some(input_metadata.id.unwrap())
            } else {
                None
            }
        })
        .collect()
}

fn add_input_constraints(
    op_id: usize,
    input_ids: Vec<usize>,
    model_metadata: &RefCell<ModelMetadata>,
) {
    let ModelMetadata {
        model,
        op_id_to_var,
        op_id_to_inputs,
        ..
    } = &mut *model_metadata.borrow_mut();

    // Add input constraints. All inputs of an op must output to the same machine (be assigned the same var)
    add_equality_constr(&input_ids, op_id_to_var, model);
    op_id_to_inputs.insert(op_id, input_ids);
}

// Store the tick that an op is constrained to
fn add_tick_constraint(metadata: &HydroIrMetadata, model_metadata: &RefCell<ModelMetadata>) {
    let ModelMetadata {
        model,
        op_id_to_var,
        op_id_to_inputs,
        prev_op_input_with_tick,
        ..
    } = &mut *model_metadata.borrow_mut();

    if let LocationId::Tick(tick_id, _) = metadata.location_kind {
        // Set each input = to the last input
        let mut inputs = op_id_to_inputs.get(&metadata.id.unwrap()).unwrap().clone();
        if let Some(prev_input) = prev_op_input_with_tick.get(&tick_id) {
            inputs.push(*prev_input);
        }
        add_equality_constr(&inputs, op_id_to_var, model);

        // Set this op's last input as the last op's input that had this tick
        if let Some(last_input) = inputs.last() {
            prev_op_input_with_tick.insert(tick_id, *last_input);
        }
    }
}

fn add_cpu_usage(
    metadata: &HydroIrMetadata,
    network_type: &Option<NetworkType>,
    model_metadata: &RefCell<ModelMetadata>,
) {
    let ModelMetadata {
        model,
        orig_node_cpu_usage,
        decoupled_node_cpu_usage,
        op_id_to_inputs,
        op_id_to_var,
        ..
    } = &mut *model_metadata.borrow_mut();

    let op_id = metadata.id.unwrap();

    // Calculate total CPU usage on each node (before overheads). Operators are run on the machine that their inputs send to.
    match network_type {
        Some(NetworkType::Send) | Some(NetworkType::SendRecv) | None => {
            if let Some(inputs) = op_id_to_inputs.get(&op_id) {
                // All inputs must be assigned the same var (by constraints above), so it suffices to check one
                if let Some(first_input) = inputs.first() {
                    let input_var = var_from_op_id(*first_input, op_id_to_var, model);
                    if let Some(cpu_usage) = metadata.cpu_usage {
                        let og_usage_temp = std::mem::replace(orig_node_cpu_usage, Expr::default());
                        *orig_node_cpu_usage = og_usage_temp + cpu_usage * (1 - input_var);
                        let decoupled_usage_temp =
                            std::mem::replace(decoupled_node_cpu_usage, Expr::default());
                        *decoupled_node_cpu_usage = decoupled_usage_temp + cpu_usage * input_var;
                    }
                }
            }
        }
        _ => {}
    }
    // Special case for network receives: their cpu usage (deserialization) is paid by the receiver, aka the machine they send to.
    match network_type {
        Some(NetworkType::Recv) | Some(NetworkType::SendRecv) => {
            let op_var = var_from_op_id(op_id, op_id_to_var, model);
            if let Some(recv_cpu_usage) = metadata.network_recv_cpu_usage {
                let og_usage_temp = std::mem::replace(orig_node_cpu_usage, Expr::default());
                *orig_node_cpu_usage = og_usage_temp + recv_cpu_usage * (1 - op_var);
                let decoupled_usage_temp =
                    std::mem::replace(decoupled_node_cpu_usage, Expr::default());
                *decoupled_node_cpu_usage = decoupled_usage_temp + recv_cpu_usage * op_var;
            }
        }
        _ => {}
    }
}

// Return the variables:
// orig_to_decoupled_var: 1 if (op1 = 0, op2 = 1), 0 otherwise
// decoupled_to_orig_var: 1 if (op1 = 1, op2 = 0), 0 otherwise
fn add_decouple_vars(
    model: &mut Model,
    op1: usize,
    op2: usize,
    op_id_to_var: &mut HashMap<usize, Var>,
    orig_to_decoupled_vars: &mut Vec<(usize, Var)>,
    decoupled_to_orig_vars: &mut Vec<(usize, Var)>,
) -> (Var, Var) {
    let op1_var = var_from_op_id(op1, op_id_to_var, model);
    let op2_var = var_from_op_id(op2, op_id_to_var, model);
    // 1 if (op1 = 0, op2 = 1), 0 otherwise
    let orig_to_decoupled_var = add_binvar!(model, name: &format!("{}_{}_orig_to_decoupled", op1, op2), bounds: ..).unwrap();
    orig_to_decoupled_vars.push((op2, orig_to_decoupled_var));
    // Technically unnecessary since we're using binvar, but future proofing
    model
        .add_constr(
            &format!("{}_{}_orig_to_decoupled_pos", op1, op2),
            c!(orig_to_decoupled_var >= 0),
        )
        .unwrap();
    model
        .add_constr(
            &format!("{}_{}_orig_to_decoupled_constr", op1, op2),
            c!(orig_to_decoupled_var >= op2_var - op1_var),
        )
        .unwrap();
    // 1 if (op1 = 1, op2 = 0), 0 otherwise
    let decoupled_to_orig_var = add_binvar!(model, name: &format!("{}_{}_decoupled_to_orig", op1, op2), bounds: ..).unwrap();
    decoupled_to_orig_vars.push((op2, decoupled_to_orig_var));
    model
        .add_constr(
            &format!("{}_{}_decoupled_to_orig_pos", op1, op2),
            c!(decoupled_to_orig_var >= 0),
        )
        .unwrap();
    model
        .add_constr(
            &format!("{}_{}_decoupled_to_orig_constr", op1, op2),
            c!(decoupled_to_orig_var >= op1_var - op2_var),
        )
        .unwrap();
    (orig_to_decoupled_var, decoupled_to_orig_var)
}

fn add_decoupling_overhead(node: &HydroNode, network_type: &Option<NetworkType>, model_metadata: &RefCell<ModelMetadata>) {
    let ModelMetadata {
        decoupling_send_overhead,
        decoupling_recv_overhead,
        model,
        orig_node_cpu_usage,
        decoupled_node_cpu_usage,
        op_id_to_inputs,
        op_id_to_var,
        orig_to_decoupled_vars,
        decoupled_to_orig_vars,
        ..
    } = &mut *model_metadata.borrow_mut();

    let metadata = node.metadata();
    let cardinality = match network_type {
        Some(NetworkType::Send) | Some(NetworkType::SendRecv) => node.input_metadata().first().unwrap().cardinality.unwrap_or_default(),
        _ => metadata.cardinality.unwrap_or_default(),
    };

    let op_id = metadata.id.unwrap();
    if let Some(inputs) = op_id_to_inputs.get(&op_id) {
        // All inputs must be assigned the same var (by constraints above), so it suffices to check one
        if let Some(input) = inputs.first() {
            let (orig_to_decoupled_var, decoupled_to_orig_var) =
                add_decouple_vars(model, *input, op_id, op_id_to_var, orig_to_decoupled_vars, decoupled_to_orig_vars);
            let og_usage_temp = std::mem::replace(orig_node_cpu_usage, Expr::default());
            *orig_node_cpu_usage = og_usage_temp
                + (*decoupling_send_overhead * cardinality as f64 + DECOUPLING_PENALTY) * orig_to_decoupled_var
                + (*decoupling_recv_overhead * cardinality as f64 + DECOUPLING_PENALTY) * decoupled_to_orig_var;
            let decoupled_usage_temp =
                std::mem::replace(decoupled_node_cpu_usage, Expr::default());
            *decoupled_node_cpu_usage = decoupled_usage_temp
                + (*decoupling_recv_overhead * cardinality as f64 + DECOUPLING_PENALTY) * orig_to_decoupled_var
                + (*decoupling_send_overhead * cardinality as f64 + DECOUPLING_PENALTY) * decoupled_to_orig_var;
        }
    }
}

fn add_tee_decoupling_overhead(
    inner_id: usize,
    metadata: &HydroIrMetadata,
    model_metadata: &RefCell<ModelMetadata>,
) {
    let ModelMetadata {
        decoupling_send_overhead,
        decoupling_recv_overhead,
        model,
        orig_node_cpu_usage,
        decoupled_node_cpu_usage,
        op_id_to_var,
        tee_inner_to_decoupled_vars,
        orig_to_decoupled_vars,
        decoupled_to_orig_vars,
        ..
    } = &mut *model_metadata.borrow_mut();

    let cardinality = metadata.cardinality.unwrap_or_default();
    let op_id = metadata.id.unwrap();

    println!("Tee {} has inner {}", op_id, inner_id);

    // 1 if any of the Tees are decoupled from the inner, 0 otherwise, and vice versa
    let (any_orig_to_decoupled_var, any_decoupled_to_orig_var) = tee_inner_to_decoupled_vars
        .entry(inner_id)
        .or_insert_with(|| {
            let any_orig_to_decoupled_var = add_binvar!(model, name: &format!("tee{}_any_orig_to_decoupled", inner_id), bounds: ..).unwrap();
            let any_decoupled_to_orig_var = add_binvar!(model, name: &format!("tee{}_any_decoupled_to_orig", inner_id), bounds: ..).unwrap();
            let og_usage_temp = std::mem::replace(orig_node_cpu_usage, Expr::default());
            *orig_node_cpu_usage = og_usage_temp
                + (*decoupling_send_overhead * cardinality as f64 + DECOUPLING_PENALTY) * any_orig_to_decoupled_var
                + (*decoupling_recv_overhead * cardinality as f64 + DECOUPLING_PENALTY) * any_decoupled_to_orig_var;
            let decoupled_usage_temp =
                std::mem::replace(decoupled_node_cpu_usage, Expr::default());
            *decoupled_node_cpu_usage = decoupled_usage_temp
                + (*decoupling_recv_overhead * cardinality as f64 + DECOUPLING_PENALTY) * any_orig_to_decoupled_var
                + (*decoupling_send_overhead * cardinality as f64 + DECOUPLING_PENALTY) * any_decoupled_to_orig_var;
            (any_orig_to_decoupled_var, any_decoupled_to_orig_var)
        });

    let (orig_to_decoupled_var, decoupled_to_orig_var) =
        add_decouple_vars(model, inner_id, op_id, op_id_to_var, orig_to_decoupled_vars, decoupled_to_orig_vars);
    // If any Tee has orig_to_decoupled, then set any_orig_to_decoupled to 1, vice versa
    model
        .add_constr(
            &format!("tee{}_inner{}_any_orig_to_decoupled", op_id, inner_id),
            c!(*any_orig_to_decoupled_var >= orig_to_decoupled_var),
        )
        .unwrap();
    model
        .add_constr(
            &format!("tee{}_inner{}_any_decoupled_to_orig", op_id, inner_id),
            c!(*any_decoupled_to_orig_var >= decoupled_to_orig_var),
        )
        .unwrap();
}

fn decouple_analysis_leaf(
    leaf: &mut HydroLeaf,
    op_id: &mut usize,
    model_metadata: &RefCell<ModelMetadata>,
) {
    // Ignore nodes that are not in the cluster to decouple
    if model_metadata.borrow().cluster_to_decouple != *leaf.metadata().location_kind.root() {
        return;
    }

    let input_ids = relevant_inputs(leaf.input_metadata(), &model_metadata.borrow().cluster_to_decouple);
    add_input_constraints(*op_id, input_ids, model_metadata);
    add_tick_constraint(leaf.metadata(), model_metadata);
}

fn decouple_analysis_node(
    node: &mut HydroNode,
    op_id: &mut usize,
    model_metadata: &RefCell<ModelMetadata>,
    cycle_source_to_sink_input: &HashMap<usize, usize>,
) {
    let network_type = get_network_type(node, &model_metadata.borrow().cluster_to_decouple);
    if let HydroNode::Network { .. } = node {
        // If this is a network and we're not involved, ignore
        if network_type.is_none() {
            return;
        }
    } else if model_metadata.borrow().cluster_to_decouple != *node.metadata().location_kind.root() {
        // If it's not a network and the operator isn't on the cluster, ignore
        return;
    }

    // Add inputs
    let input_ids = match node {
        HydroNode::CycleSource { .. } => {
            // For CycleSource, its input is its CycleSink's input. Note: assume the CycleSink is on the same cluster
            vec![*cycle_source_to_sink_input.get(op_id).unwrap()]
        }
        HydroNode::Tee { inner, .. } => {
            vec![inner.0.borrow().metadata().id.unwrap()]
        }
        _ => {
            relevant_inputs(node.input_metadata(), &model_metadata.borrow().cluster_to_decouple)
        }
    };
    add_input_constraints(*op_id, input_ids, model_metadata);

    // Add decoupling overhead. For Tees of the same inner, even if multiple are decoupled, only penalize decoupling once
    if let HydroNode::Tee {
        inner, metadata, ..
    } = node
    {
        add_tee_decoupling_overhead(
            inner.0.borrow().metadata().id.unwrap(),
            metadata,
            model_metadata,
        );
    } else {
        add_decoupling_overhead(node, &network_type, model_metadata);
    }

    add_cpu_usage(node.metadata(), &network_type, model_metadata);
    add_tick_constraint(node.metadata(), model_metadata);
}

fn construct_objective_fn(model_metadata: &RefCell<ModelMetadata>) {
    let ModelMetadata {
        model,
        orig_node_cpu_usage,
        decoupled_node_cpu_usage,
        ..
    } = &mut *model_metadata.borrow_mut();

    // Create vars that store the CPU usage of each node
    let orig_node_cpu_var = add_ctsvar!(model, bounds: ..).unwrap();
    let decoupled_node_cpu_var = add_ctsvar!(model, bounds: ..).unwrap();
    let og_usage_temp = std::mem::replace(orig_node_cpu_usage, Expr::default());
    let decoupled_usage_temp = std::mem::replace(decoupled_node_cpu_usage, Expr::default());
    model
        .add_constr("orig_node_cpu", c!(orig_node_cpu_var == og_usage_temp))
        .unwrap();
    model
        .add_constr(
            "decoupled_node_cpu",
            c!(decoupled_node_cpu_var == decoupled_usage_temp),
        )
        .unwrap();

    // Which node has the highest CPU usage?
    let highest_cpu = add_ctsvar!(model, bounds: ..).unwrap();
    model
        .add_constr("higher_than_orig_cpu", c!(highest_cpu >= orig_node_cpu_var))
        .unwrap();
    model
        .add_constr(
            "higher_than_decoupled_cpu",
            c!(highest_cpu >= decoupled_node_cpu_var),
        )
        .unwrap();

    // Minimize the CPU usage of that node
    model.set_objective(highest_cpu, Minimize).unwrap();
}

pub fn decouple_analysis(
    ir: &mut [HydroLeaf],
    modelname: &str,
    cluster_to_decouple: &LocationId,
    send_overhead: f64,
    recv_overhead: f64,
    cycle_source_to_sink_input: &HashMap<usize, usize>,
) {
    let model_metadata = RefCell::new(ModelMetadata {
        cluster_to_decouple: cluster_to_decouple.clone(),
        decoupling_send_overhead: send_overhead,
        decoupling_recv_overhead: recv_overhead,
        model: Model::new(modelname).unwrap(),
        orig_node_cpu_usage: Expr::default(),
        decoupled_node_cpu_usage: Expr::default(),
        op_id_to_var: HashMap::new(),
        op_id_to_inputs: HashMap::new(),
        prev_op_input_with_tick: HashMap::new(),
        tee_inner_to_decoupled_vars: HashMap::new(),
        orig_to_decoupled_vars: vec![],
        decoupled_to_orig_vars: vec![],
    });

    traverse_dfir(
        ir,
        |leaf, next_op_id| {
            decouple_analysis_leaf(leaf, next_op_id, &model_metadata);
        },
        |node, next_op_id| {
            decouple_analysis_node(node, next_op_id, &model_metadata, cycle_source_to_sink_input);
        },
    );

    construct_objective_fn(&model_metadata);
    let ModelMetadata {
        model,
        op_id_to_var,
        op_id_to_inputs,
        ..
    } = &mut *model_metadata.borrow_mut();
    model.write("decouple.lp").unwrap();
    model.optimize().unwrap();
    model.write("decouple.sol").unwrap();

    let mut orig_machine = vec![];
    let mut decoupled_machine = vec![];
    let mut orig_to_decoupled = vec![];
    let mut decoupled_to_orig = vec![];

    for (op_id, inputs) in op_id_to_inputs {
        if let Some(input) = inputs.first() {
            if let Some(op_var) = op_id_to_var.get(op_id) {
                if let Some(input_var) = op_id_to_var.get(input) {
                    let op_value = model.get_obj_attr(attr::X, op_var).unwrap();
                    let input_value = model.get_obj_attr(attr::X, input_var).unwrap();
                    match (input_value, op_value) {
                        (0.0, 1.0) => {
                            orig_to_decoupled.push(*op_id);
                        }
                        (1.0, 0.0) => {
                            decoupled_to_orig.push(*op_id);
                        }
                        _ => {}
                    }

                    if input_value == 0.0 {
                        orig_machine.push(*op_id);
                    } else {
                        decoupled_machine.push(*op_id);
                    }
                }
            }
        }
    }

    orig_machine.sort();
    decoupled_machine.sort();
    println!("Original: {:?}", orig_machine);
    println!("Decoupling: {:?}", decoupled_machine);
    orig_to_decoupled.sort();
    decoupled_to_orig.sort();
    println!("Original outputting to decoupled after: {:?}", orig_to_decoupled);
    println!("Decoupled outputting to original after: {:?}", decoupled_to_orig);
}
