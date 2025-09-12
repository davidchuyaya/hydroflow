use std::cell::RefCell;
use std::collections::HashMap;

use hydro_lang::builder::deploy::DeployResult;
use hydro_lang::deploy::HydroDeploy;
use hydro_lang::deploy::deploy_graph::DeployCrateWrapper;
use hydro_lang::ir::{HydroIrMetadata, HydroLeaf, HydroNode, traverse_dfir};
use hydro_lang::location::LocationId;
use regex::Regex;
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Default)]
pub struct RunMetadata {
    pub send_overhead: HashMap<LocationId, f64>,
    pub recv_overhead: HashMap<LocationId, f64>,
    unaccounted_perf: HashMap<LocationId, f64>, // % of perf samples not mapped to any operator
    total_usage: HashMap<LocationId, f64>,      // 100% CPU = 1.0
    op_id_to_metadata: HashMap<usize, HydroIrMetadata>,
    orig_id_to_metadata: HashMap<usize, HydroIrMetadata>,
    op_id_to_input_op_id: HashMap<usize, Vec<usize>>,
}

pub type MultiRunMetadata = Vec<RunMetadata>;

pub fn parse_cpu_usage(measurement: String) -> f64 {
    let regex = Regex::new(r"Total (\d+\.\d+)%").unwrap();
    regex
        .captures_iter(&measurement)
        .last()
        .map(|cap| cap[1].parse::<f64>().unwrap())
        .unwrap_or(0f64)
}

/// Returns a map from (operator ID, is network receiver) to percentage of total samples, and the percentage of samples that are unaccounted
fn parse_perf(file: String) -> (HashMap<(usize, bool), f64>, f64) {
    let mut total_samples = 0f64;
    let mut unidentified_samples = 0f64;
    let mut samples_per_id = HashMap::new();
    let operator_regex = Regex::new(r"op_\d+v\d+__(.*?)__(send)?(recv)?(\d+)").unwrap();

    for line in file.lines() {
        let n_samples_index = line.rfind(' ').unwrap() + 1;
        let n_samples = &line[n_samples_index..].parse::<f64>().unwrap();

        if let Some(cap) = operator_regex.captures_iter(line).last() {
            let id = cap[4].parse::<usize>().unwrap();
            let is_network_recv = cap
                .get(3)
                .is_some_and(|direction| direction.as_str() == "recv");

            let dfir_operator_and_samples =
                samples_per_id.entry((id, is_network_recv)).or_insert(0.0);
            *dfir_operator_and_samples += n_samples;
        } else {
            unidentified_samples += n_samples;
        }
        total_samples += n_samples;
    }

    let percent_unidentified = unidentified_samples / total_samples;
    println!(
        "Out of {} samples, {} were unidentified, {}%",
        total_samples,
        unidentified_samples,
        percent_unidentified * 100.0
    );

    samples_per_id
        .iter_mut()
        .for_each(|(_, samples)| *samples /= total_samples);
    (samples_per_id, percent_unidentified)
}

fn inject_perf_leaf(
    leaf: &mut HydroLeaf,
    id_to_usage: &HashMap<(usize, bool), f64>,
    next_stmt_id: &mut usize,
) {
    if let Some(cpu_usage) = id_to_usage.get(&(*next_stmt_id, false)) {
        leaf.metadata_mut().cpu_usage = Some(*cpu_usage);
    }
}

fn inject_perf_node(
    node: &mut HydroNode,
    id_to_usage: &HashMap<(usize, bool), f64>,
    next_stmt_id: &mut usize,
) {
    if let Some(cpu_usage) = id_to_usage.get(&(*next_stmt_id, false)) {
        node.metadata_mut().cpu_usage = Some(*cpu_usage);
    }
    // If this is a Network node, separately get receiver CPU usage
    if let HydroNode::Network { metadata, .. } = node
        && let Some(cpu_usage) = id_to_usage.get(&(*next_stmt_id, true))
    {
        metadata.network_recv_cpu_usage = Some(*cpu_usage);
    }
}

pub fn inject_perf(ir: &mut [HydroLeaf], folded_data: Vec<u8>) -> f64 {
    let (id_to_usage, unidentified_usage) = parse_perf(String::from_utf8(folded_data).unwrap());
    traverse_dfir(
        ir,
        |leaf, next_stmt_id| {
            inject_perf_leaf(leaf, &id_to_usage, next_stmt_id);
        },
        |node, next_stmt_id| {
            inject_perf_node(node, &id_to_usage, next_stmt_id);
        },
    );
    unidentified_usage
}

/// Returns (op_id, count)
pub fn parse_counter_usage(measurement: String) -> (usize, usize) {
    let regex = Regex::new(r"\((\d+)\): (\d+)").unwrap();
    let matches = regex.captures_iter(&measurement).last().unwrap();
    let op_id = matches[1].parse::<usize>().unwrap();
    let count = matches[2].parse::<usize>().unwrap();
    (op_id, count)
}

fn inject_count_node(
    node: &mut HydroNode,
    next_stmt_id: &mut usize,
    op_to_count: &HashMap<usize, usize>,
) {
    if let Some(count) = op_to_count.get(next_stmt_id) {
        let metadata = node.metadata_mut();
        metadata.cardinality = Some(*count);
    } else {
        match node {
            HydroNode::Tee { inner ,metadata, .. } => {
                metadata.cardinality = inner.0.borrow().metadata().cardinality;
            }
            | HydroNode::Map { input, metadata, .. } // Equal to parent cardinality
            | HydroNode::DeferTick { input, metadata, .. } // Equal to parent cardinality
            | HydroNode::Enumerate { input, metadata, .. }
            | HydroNode::Inspect { input, metadata, .. }
            | HydroNode::Sort { input, metadata, .. }
            | HydroNode::Counter { input, metadata, .. }
            => {
                metadata.cardinality = input.metadata().cardinality;
            }
            _ => {}
        }
    }
}

pub fn inject_count(ir: &mut [HydroLeaf], op_to_count: &HashMap<usize, usize>) {
    traverse_dfir(
        ir,
        |_, _| {},
        |node, next_stmt_id| {
            inject_count_node(node, next_stmt_id, op_to_count);
        },
    );
}

pub async fn analyze_process_results(
    process: &impl DeployCrateWrapper,
    ir: &mut [HydroLeaf],
    node_cardinality: &mut UnboundedReceiver<String>,
) -> f64 {
    let perf_results = process.tracing_results().await.unwrap();

    // Inject perf usages into metadata
    let unidentified_usage = inject_perf(ir, perf_results.folded_data);

    // Get cardinality data. Allow later values to overwrite earlier ones
    let mut op_to_counter = HashMap::new();
    while let Some(measurement) = node_cardinality.recv().await {
        let (op_id, count) = parse_counter_usage(measurement);
        op_to_counter.insert(op_id, count);
    }
    inject_count(ir, &op_to_counter);

    unidentified_usage
}

pub async fn analyze_cluster_results(
    nodes: &DeployResult<'_, HydroDeploy>,
    ir: &mut [HydroLeaf],
    usage_out: &mut HashMap<(LocationId, String, usize), UnboundedReceiver<String>>,
    cardinality_out: &mut HashMap<(LocationId, String, usize), UnboundedReceiver<String>>,
    run_metadata: &mut RunMetadata,
    exclude_from_decoupling: Vec<String>,
) -> (LocationId, String, usize) {
    let mut max_usage_cluster_id = None;
    let mut max_usage_cluster_size = 0;
    let mut max_usage_cluster_name = String::new();
    let mut max_usage_overall = 0f64;

    for (id, name, cluster) in nodes.get_all_clusters() {
        println!("Analyzing cluster {:?}: {}", id, name);

        // Iterate through nodes' usages and keep the max usage one
        let mut max_usage = None;
        for (idx, _) in cluster.members().iter().enumerate() {
            let usage =
                get_usage(usage_out.get_mut(&(id.clone(), name.clone(), idx)).unwrap()).await;
            println!("Node {} usage: {}", idx, usage);
            if let Some((prev_usage, _)) = max_usage {
                if usage > prev_usage {
                    max_usage = Some((usage, idx));
                }
            } else {
                max_usage = Some((usage, idx));
            }
        }

        if let Some((usage, idx)) = max_usage {
            // Modify IR with perf & cardinality numbers
            let node_cardinality = cardinality_out
                .get_mut(&(id.clone(), name.clone(), idx))
                .unwrap();
            let unidentified_perf =
                analyze_process_results(cluster.members().get(idx).unwrap(), ir, node_cardinality)
                    .await;

            run_metadata.total_usage.insert(id.clone(), usage);
            run_metadata
                .unaccounted_perf
                .insert(id.clone(), unidentified_perf);

            // Update cluster with max usage
            if max_usage_overall < usage && !exclude_from_decoupling.contains(&name) {
                max_usage_cluster_id = Some(id);
                max_usage_cluster_name = name.clone();
                max_usage_cluster_size = cluster.members().len();
                max_usage_overall = usage;
                println!("The bottleneck is {}", name);
            }
        }
    }

    (
        max_usage_cluster_id.unwrap(),
        max_usage_cluster_name,
        max_usage_cluster_size,
    )
}

pub async fn get_usage(usage_out: &mut UnboundedReceiver<String>) -> f64 {
    let measurement = usage_out.recv().await.unwrap();
    parse_cpu_usage(measurement)
}

// Track the max of each so we decouple conservatively
pub fn analyze_send_recv_overheads(ir: &mut [HydroLeaf], run_metadata: &mut RunMetadata) {
    traverse_dfir(
        ir,
        |_, _| {},
        |node, _| {
            if let HydroNode::Network {
                input, metadata, ..
            } = node
            {
                let sender = input.metadata().location_kind.root();
                let receiver = metadata.location_kind.root();

                // Use cardinality from the network's input, not the network itself.
                // Reason: Cardinality is measured at ONE recipient, but the sender may be sending to MANY machines.
                if let Some(cpu_usage) = metadata.cpu_usage
                    && let Some(cardinality) = input.metadata().cardinality
                {
                    let overhead = cpu_usage / cardinality as f64;
                    run_metadata
                        .send_overhead
                        .entry(sender.clone())
                        .and_modify(|max_send_overhead| {
                            if overhead > *max_send_overhead {
                                *max_send_overhead = overhead;
                            }
                        })
                        .or_insert(overhead);
                }

                if let Some(cardinality) = metadata.cardinality
                    && let Some(cpu_usage) = metadata.network_recv_cpu_usage
                {
                    let overhead = cpu_usage / cardinality as f64;

                    run_metadata
                        .recv_overhead
                        .entry(receiver.clone())
                        .and_modify(|max_recv_overhead| {
                            if overhead > *max_recv_overhead {
                                *max_recv_overhead = overhead;
                            }
                        })
                        .or_insert(overhead);
                }
            }
        },
    );

    // Print
    for (location, overhead) in &run_metadata.send_overhead {
        println!("Max send overhead at {:?}: {}", location, overhead);
    }
    for (location, overhead) in &run_metadata.recv_overhead {
        println!("Max recv overhead at {:?}: {}", location, overhead);
    }
}

fn record_metadata(
    metadata: &HydroIrMetadata,
    input_metadata: Vec<&HydroIrMetadata>,
    run_metadata: &mut RunMetadata,
) {
    run_metadata
        .op_id_to_metadata
        .insert(metadata.id.unwrap(), metadata.clone());
    if let Some(orig_id) = metadata.orig_id {
        run_metadata
            .orig_id_to_metadata
            .insert(orig_id, metadata.clone());
    }

    let input_ids = input_metadata.iter().filter_map(|input| input.id).collect();
    run_metadata
        .op_id_to_input_op_id
        .insert(metadata.id.unwrap(), input_ids);
}

fn compare_expected_values(
    opt_new_value: Option<f64>,
    opt_old_value: Option<f64>,
    orig_id: usize,
    value_name: &str,
) {
    match (opt_new_value, opt_old_value) {
        (Some(new_value), Some(old_value)) => {
            if old_value > 0.0 {
                let scale_up = new_value / old_value;
                println!(
                    "Operator {} {} scaled {}x ({} -> {})",
                    orig_id, value_name, scale_up, old_value, new_value
                );
            } else {
                println!(
                    "Operator {} had no previous {}, new {}: {}",
                    orig_id, value_name, value_name, new_value
                );
            }
        }
        (Some(new_value), None) => {
            println!(
                "Operator {} had no previous {}, new {}: {}",
                orig_id, value_name, value_name, new_value
            );
        }
        (None, Some(old_value)) => {
            println!(
                "Operator {} has no new {}, old {}: {}",
                orig_id, value_name, value_name, old_value
            );
        }
        _ => {}
    }
}

/// Records CPU usage based on a node's orig_id and compares it to the previous iteration.
/// Prints the scale-up factor for each operator.
/// `recorded_usages` is indexed by the iteration, maps from orig_id to the metadata of that run
/// Note: This won't be able to capture the CPU usage of nodes added by decoupling (mostly networking). Could be added later?
pub fn compare_expected_performance(
    ir: &mut [HydroLeaf],
    multi_run_metadata: &RefCell<MultiRunMetadata>,
    iteration: usize,
) {
    // Record run_metadata
    traverse_dfir(
        ir,
        |leaf, _| {
            record_metadata(
                leaf.metadata(),
                leaf.input_metadata(),
                multi_run_metadata.borrow_mut().get_mut(iteration).unwrap(),
            );
        },
        |node, _| {
            record_metadata(
                node.metadata(),
                node.input_metadata(),
                multi_run_metadata.borrow_mut().get_mut(iteration).unwrap(),
            );
        },
    );

    // Nothing to compare against for the 1st run, return
    if iteration == 0 {
        return;
    }

    // Compare against previous runs
    let borrowed_multi_run_metadata = multi_run_metadata.borrow();
    let run_metadata = borrowed_multi_run_metadata.get(iteration).unwrap();
    let prev_run_metadata = borrowed_multi_run_metadata.get(iteration - 1).unwrap();

    // 1. Compare operators with an orig_id
    for (orig_id, metadata) in run_metadata.orig_id_to_metadata.iter() {
        if let Some(old_metadata) = prev_run_metadata.orig_id_to_metadata.get(orig_id) {
            // Compare CPU usages
            compare_expected_values(
                metadata.cpu_usage,
                old_metadata.cpu_usage,
                *orig_id,
                "CPU usage",
            );
            compare_expected_values(
                metadata.network_recv_cpu_usage,
                old_metadata.network_recv_cpu_usage,
                *orig_id,
                "recv CPU usage",
            );
            // Compare cardinalities
            compare_expected_values(
                metadata.cardinality.map(|c| c as f64),
                old_metadata.cardinality.map(|c| c as f64),
                *orig_id,
                "cardinality",
            );
        } else {
            println!("Warning: Operator {} not found in previous run", orig_id);
        }
    }

    // 2. Compare operators without orig_id (added by decoupling)
    // let mut orig_id_to_send_usage = HashMap::new(); // orig_id -> CPU usage of output nodes that send to another node
    // let mut orig_id_to_recv_usage = HashMap::new();
    // for (id, metadata) in run_metadata.op_id_to_metadata {
    //     if metadata.orig_id.is_none() {
    //         let mut inputs = run_metadata.op_id_to_input_op_id.get(&id).unwrap();

    //     }
    // }

    // 3. Compare changes in unexplained CPU usage
    for (location, unexplained) in &run_metadata.unaccounted_perf {
        if let Some(old_unexplained) = prev_run_metadata.unaccounted_perf.get(location) {
            println!(
                "Location {:?}'s unexplained CPU usage changed from {} to {}",
                location, old_unexplained, unexplained
            );
        }
    }

    // 4. Compare changes in send/recv overheads
    for (location, send_overhead) in &run_metadata.send_overhead {
        if let Some(old_send_overhead) = prev_run_metadata.send_overhead.get(location) {
            println!(
                "Location {:?}'s send overhead changed from {} to {}",
                location, old_send_overhead, send_overhead
            );
        }
    }
    for (location, recv_overhead) in &run_metadata.recv_overhead {
        if let Some(old_recv_overhead) = prev_run_metadata.recv_overhead.get(location) {
            println!(
                "Location {:?}'s recv overhead changed from {} to {}",
                location, old_recv_overhead, recv_overhead
            );
        }
    }
}
