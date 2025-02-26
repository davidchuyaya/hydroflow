use std::cell::RefCell;
use std::collections::HashMap;

use grb::prelude::*;

use crate::ir::*;
use crate::location::LocationId;

struct ModelMetadata {
    cluster_to_decouple: LocationId,
    decoupling_send_overhead: f64, /* CPU usage per cardinality to send, assuming all messages serialize/deserialize similarly */
    decoupling_recv_overhead: f64,
    // Model variables to construct final cost function
    model: Model,
    stmt_id_to_metadata: HashMap<usize, HydroIrMetadata>,
    ops_with_same_tick: HashMap<usize, Vec<usize>>, // tick_id: vec of op_id
    tees_with_same_inner: HashMap<usize, (Vec<usize>, usize)>, // inner_id: (vec of Tee op_id, cardinality)
    potential_decoupling_network_cardinalities: Vec<(usize, usize, usize)>, /* (operator ID, input ID, cardinality) */
}

fn var_for_op_id(model: &Model, op_id: usize) -> Var {
    model.get_var_by_name(&op_id.to_string()).unwrap().unwrap()
}

fn add_var_and_metadata(metadata: &HydroIrMetadata, model_metadata: &RefCell<ModelMetadata>) {
    let ModelMetadata {
        model,
        stmt_id_to_metadata,
        ..
    } = &mut *model_metadata.borrow_mut();

    // Create var
    let id = metadata.id.unwrap();
    let name = id.to_string();
    add_binvar!(model, name: &name, bounds: ..).unwrap();

    // Store metadata
    stmt_id_to_metadata.insert(id, metadata.clone());
}

// Store how much data we would need to send if we decoupled above this
fn add_decoupling_overhead(input_metadatas: Vec<&mut HydroIrMetadata>, model_metadata: &RefCell<ModelMetadata>) {
    let ModelMetadata {
        cluster_to_decouple,
        potential_decoupling_network_cardinalities,
        ..
    } = &mut *model_metadata.borrow_mut();

    for input_metadata in input_metadatas {
        if cluster_to_decouple != input_metadata.location_kind.root() {
            continue;
        }
        if let Some(input_id) = input_metadata.id {
            if let Some(input_cardinality) = input_metadata.cardinality {
                potential_decoupling_network_cardinalities.push((
                    input_metadata.id.unwrap(),
                    input_id,
                    input_cardinality,
                ));
            }
        }
    }
}

// Store the tick that an op is constrained to
fn add_tick_constraint(metadata: &HydroIrMetadata, model_metadata: &RefCell<ModelMetadata>) {
    let ModelMetadata {
        ops_with_same_tick,
        ..
    } = &mut *model_metadata.borrow_mut();
    if let LocationId::Tick(tick_id, _) = metadata.location_kind {
        ops_with_same_tick
            .entry(tick_id)
            .or_insert_with(Vec::new)
            .push(metadata.id.unwrap());
    }
}

fn decouple_analysis_leaf(
    leaf: &mut HydroLeaf,
    _next_stmt_id: &mut usize,
    model_metadata: &RefCell<ModelMetadata>,
) {
    let ModelMetadata {
        cluster_to_decouple,
        ..
    } = &mut *model_metadata.borrow_mut();

    // Ignore nodes that are not in the cluster to decouple
    if cluster_to_decouple != leaf.metadata().location_kind.root() {
        return;
    }

    add_var_and_metadata(&leaf.metadata(), model_metadata);
    add_decoupling_overhead(leaf.input_metadata_mut(), model_metadata);
    add_tick_constraint(&leaf.metadata(), model_metadata);
}

fn decouple_analysis_node(
    node: &mut HydroNode,
    next_stmt_id: &mut usize,
    model_metadata: &RefCell<ModelMetadata>,
) {
    let ModelMetadata {
        cluster_to_decouple,
        tees_with_same_inner,
        ..
    } = &mut *model_metadata.borrow_mut();

    // Ignore nodes that are not in the cluster to decouple
    if cluster_to_decouple != node.metadata().location_kind.root() {
        return;
    }

    add_var_and_metadata(&node.metadata(), model_metadata);
    // Add metadata for calculating decoupling overhead, special cases for Tee and CycleSource
    match node {
        HydroNode::Tee { inner, .. } => {
            let inner_node = inner.0.borrow();
            let inner_metadata = inner_node.metadata();
            if cluster_to_decouple == inner_metadata.location_kind.root() {
                if let Some(inner_cardinality) = inner_metadata.cardinality {
                    let entry = tees_with_same_inner
                        .entry(inner_metadata.id.unwrap())
                        .or_insert_with(|| (vec![], inner_cardinality));
                    entry.0.push(*next_stmt_id);
                }
            }
        }
        HydroNode::CycleSource { .. } => {
            // Do nothing, will be handled later
        }
        _ => {
            add_decoupling_overhead(node.input_metadata_mut(), model_metadata);
        }
    }
    add_tick_constraint(&node.metadata(), model_metadata);
}

fn construct_objective_fn(model_metadata: &RefCell<ModelMetadata>, cycle_sink_to_sources: &HashMap<usize, usize>) {
    let ModelMetadata {
        decoupling_send_overhead,
        decoupling_recv_overhead,
        model,
        stmt_id_to_metadata,
        ops_with_same_tick,
        tees_with_same_inner,
        potential_decoupling_network_cardinalities,
        ..
    } = &mut *model_metadata.borrow_mut();

    // Add tick constraints
    for (_, ops) in ops_with_same_tick {
        let mut prev_op: Option<usize> = None;
        for op_id in ops {
            if let Some(prev_op_id) = prev_op {
                let prev_op_var = var_for_op_id(model, prev_op_id);
                let op_var = var_for_op_id(model, *op_id);
                model.add_constr(
                        &format!("tick_constraint_{}_{}", prev_op_id, op_id),
                        c!(prev_op_var == op_var),
                    )
                    .unwrap();
            }
            prev_op = Some(*op_id);
        }
    }

    let mut orig_node_cpu_expr = Expr::default();
    let mut decoupled_node_cpu_expr = Expr::default();

    // Calculate total CPU usage on each node (before overheads)
    for (stmt_id, metadata) in stmt_id_to_metadata.iter() {
        if let Some(cpu_usage) = metadata.cpu_usage {
            let var = var_for_op_id(model, *stmt_id);

            orig_node_cpu_expr = orig_node_cpu_expr + cpu_usage * var;
            decoupled_node_cpu_expr = decoupled_node_cpu_expr + cpu_usage * (1 - var);
        }
    }

    // Calculate overheads
    for (op, input, cardinality) in potential_decoupling_network_cardinalities {
        let op_var = var_for_op_id(model, *op);
        let input_var = var_for_op_id(model, *input);

        // Variable that is 1 if the op and its input are on different nodes
        let op_or_input_var = add_binvar!(model, bounds: ..).unwrap();
        model
            .add_genconstr_or(&format!("op{}_or_input{}", op, input), op_or_input_var, [op_var, input_var])
            .unwrap();

        orig_node_cpu_expr =
            orig_node_cpu_expr + *decoupling_send_overhead * *cardinality as f64 * op_or_input_var;
        decoupled_node_cpu_expr = decoupled_node_cpu_expr
            + *decoupling_recv_overhead * *cardinality as f64 * op_or_input_var;

        if let Some(source_id) = cycle_sink_to_sources.get(op) {
            // If the op is a CycleSink, then decoupling above the sink = decoupling above the source as well, so factor that overhead in too
            let source_var = var_for_op_id(model, *source_id);

            let source_or_input_var = add_binvar!(model, bounds: ..).unwrap();
            model
                .add_genconstr_or(&format!("op{}_or_input{}", source_id, input), source_or_input_var, [source_var, input_var])
                .unwrap();

            orig_node_cpu_expr =
                orig_node_cpu_expr + *decoupling_send_overhead * *cardinality as f64 * source_or_input_var;
            decoupled_node_cpu_expr = decoupled_node_cpu_expr
                + *decoupling_recv_overhead * *cardinality as f64 * source_or_input_var;
        }
    }
    // Calculate overhead of decoupling any set of Tees from its inner. Only penalize decoupling once for decoupling any number of Tees.
    for (tee_inner, (ops, cardinality)) in tees_with_same_inner {
        let tee_inner_var = var_for_op_id(model, *tee_inner);
        let mut op_vars = ops
            .iter()
            .map(|op| var_for_op_id(model, *op))
            .collect::<Vec<_>>();
        op_vars.push(tee_inner_var);

        // Variable that is 0 if the inner and any Tees are on the decoupled node
        let min_tee_or_inner_var = add_binvar!(model, bounds: ..).unwrap();
        model
            .add_genconstr_min(&format!("tee_inner_min{}", tee_inner), min_tee_or_inner_var, op_vars.clone(), None)
            .unwrap();
        // Variable that is 1 if the inner and any Tees are on the original node
        let max_tee_or_inner_var = add_binvar!(model, bounds: ..).unwrap();
        model
            .add_genconstr_max(&format!("tee_inner_max{}", tee_inner), max_tee_or_inner_var, op_vars, None)
            .unwrap();
        // Variable that is 1 or -1 if the inner and any Tees are on different nodes
        let tee_diff_var = add_intvar!(model, bounds: ..).unwrap();
        model
            .add_constr(
                &format!("tee_inner_diff_{}", tee_inner),
                c!(tee_diff_var == max_tee_or_inner_var - min_tee_or_inner_var),
            )
            .unwrap();
        // Variable that is 1 if the inner and any Tees are on different nodes, 0 otherwise
        let decoupled_tee_var = add_binvar!(model, bounds: ..).unwrap();
        model.add_genconstr_abs(
            &format!("tee_inner_decoupled_{}", tee_inner),
            decoupled_tee_var,
            tee_diff_var,
        );

        orig_node_cpu_expr =
            orig_node_cpu_expr + *decoupling_send_overhead * *cardinality as f64 * decoupled_tee_var;
        decoupled_node_cpu_expr = decoupled_node_cpu_expr
            + *decoupling_recv_overhead * *cardinality as f64 * decoupled_tee_var;
    }

    // Create vars that store the CPU usage of each node
    let orig_node_cpu_var = add_ctsvar!(model, bounds: ..).unwrap();
    let decoupled_node_cpu_var = add_ctsvar!(model, bounds: ..).unwrap();
    model
        .add_constr("orig_node_cpu", c!(orig_node_cpu_var == orig_node_cpu_expr))
        .unwrap();
    model
        .add_constr(
            "decoupled_node_cpu",
            c!(decoupled_node_cpu_var == decoupled_node_cpu_expr),
        )
        .unwrap();

    // Which node has the highest CPU usage?
    let highest_cpu = add_ctsvar!(model, bounds: ..).unwrap();
    model
        .add_genconstr_max(
            "highest_cpu",
            highest_cpu,
            [orig_node_cpu_var, decoupled_node_cpu_var],
            None,
        )
        .unwrap();

    // Minimize the CPU usage of that node
    model.set_objective(highest_cpu, Minimize).unwrap();
}

pub fn decouple_analysis(ir: &mut [HydroLeaf], modelname: &str, cluster_to_decouple: &LocationId, cycle_sink_to_sources: &HashMap<usize, usize>) {
    let model_metadata = RefCell::new(ModelMetadata {
        cluster_to_decouple: cluster_to_decouple.clone(),
        decoupling_send_overhead: 0.001, // TODO: Calculate
        decoupling_recv_overhead: 0.001,
        model: Model::new(modelname).unwrap(),
        stmt_id_to_metadata: HashMap::new(),
        ops_with_same_tick: HashMap::new(),
        tees_with_same_inner: HashMap::new(),
        potential_decoupling_network_cardinalities: vec![],
    });

    traverse_dfir(
        ir,
        |leaf, next_stmt_id| {
            decouple_analysis_leaf(leaf, next_stmt_id, &model_metadata);
        },
        |node, next_stmt_id| {
            decouple_analysis_node(node, next_stmt_id, &model_metadata);
        },
    );

    construct_objective_fn(&model_metadata, cycle_sink_to_sources);
    let model = &mut model_metadata.borrow_mut().model;
    model.optimize().unwrap();

    println!("We're decoupling the following operators:");
    for (stmt_id, _) in model_metadata.borrow().stmt_id_to_metadata.iter() {
        if model.get_obj_attr(attr::X, &var_for_op_id(model, *stmt_id)).unwrap() == 0.0 {
            println!("{}", stmt_id);
        }
    }
}
