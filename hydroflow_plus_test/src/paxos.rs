use std::collections::HashMap;

use hydroflow_plus::*;
use hydroflow_plus::util::cli::HydroCLI;
use hydroflow_plus_cli_integration::*;
use stageleft::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
struct ClientPayload {
    key: u32,
    value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct ReplicaPayload { // Note: Important that seq is the first member of the struct for sorting
    seq: i32,
    key: u32,
    value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Ballot { // Note: Important that num comes before id, since Ord is defined lexicographically
    num: u32,
    id: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct P1a {
    ballot: Ballot,
}

#[derive(Serialize, Deserialize, Clone)]
struct LogValue {
    ballot: Ballot,
    slot: i32,
    value: ClientPayload,
}

#[derive(Serialize, Deserialize, Clone)]
struct P1b {
    ballot: Ballot,
    max_ballot: Ballot,
    accepted: Vec<LogValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
struct P2a {
    ballot: Ballot,
    slot: i32,
    value: ClientPayload,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
struct P2b {
    ballot: Ballot,
    max_ballot: Ballot,
    slot: i32,
    value: ClientPayload,
}


pub fn paxos<'a, D: Deploy<'a, ClusterId = u32>>(
    flow: &FlowBuilder<'a, D>,
    proposers_spec: &impl ClusterSpec<'a, D>,
    acceptors_spec: &impl ClusterSpec<'a, D>,
    clients_spec: &impl ClusterSpec<'a, D>,
    replicas_spec: &impl ClusterSpec<'a, D>,
    f: RuntimeData<&'a usize>,
    num_clients_per_node: RuntimeData<&'a usize>,
    kv_num_keys: RuntimeData<&'a usize>, // How many unique keys are expected by the state machine kv store?
    kv_value_size: RuntimeData<&'a usize>, // Number of bytes in each value in the kv store
) -> (D::Cluster, D::Cluster, D::Cluster, D::Cluster) {
    let proposers = flow.cluster(proposers_spec);
    let acceptors = flow.cluster(acceptors_spec);
    let clients = flow.cluster(clients_spec);
    let replicas = flow.cluster(replicas_spec);

    /* Proposers */
    flow.source_iter(&proposers, q!(["Proposers say hello"]))
        .for_each(q!(|s| println!("{}", s)));

    /* Acceptors */
    flow.source_iter(&acceptors, q!(["Acceptors say hello"]))
        .for_each(q!(|s| println!("{}", s)));

    /* Clients */
    let c_to_replicas_tick_1 = flow
        .source_iter(&clients, q!([
            // ReplicaPayload { seq: 1, key: 10, value: "Hello again, Berkeley!".to_string() },
            // ReplicaPayload { seq: 2, key: 10, value: "Goodbye, Berkeley".to_string() },
            // ReplicaPayload { seq: 0, key: 10, value: "Hello, Berkeley!".to_string() },
            // ReplicaPayload { seq: 3, key: 20, value: "Hello, SF".to_string() },
            // ReplicaPayload { seq: 5, key: 20, value: "Goodbye, SF".to_string() }
            (1, 10, "Hello again, Berkeley!".to_string()),
            (2, 10, "Goodbye, Berkeley".to_string()),
            (0, 10, "Hello, Berkeley!".to_string()),
            (3, 20, "Hello, SF".to_string()),
            (5, 20, "Goodbye, SF".to_string())
        ]));
        
    let c_to_replicas_tick_2 = flow
        .source_iter(&clients, q!([(4, 10, "No more Berkeley".to_string())]))
        .defer_tick();
    let c_to_replicas = c_to_replicas_tick_1
        .union(c_to_replicas_tick_2)
        .broadcast_bincode_interleaved(&replicas);

    /* Replicas. All relations for replicas will be prefixed with r. */
    let (r_buffered_payloads_complete_cycle, r_buffered_payloads) = flow.cycle(&replicas);
    let r_sorted_payloads = c_to_replicas
        .tick_batch()
        .union(r_buffered_payloads) // Combine with all payloads that we've received and not processed yet
        .sort();
    // Create a cycle since we'll use this seq before we define it
    let (r_highest_seq_complete_cycle, r_highest_seq) = flow.cycle(&replicas);
    let empty_slot = flow.source_iter(&replicas, q!([-1]));
    // Either the max sequence number executed so far or -1. Need to union otherwise r_highest_seq is empty and joins with it will fail
    let r_highest_seq_with_default = r_highest_seq
        .union(empty_slot);
    // Find highest the sequence number of any payload that can be processed in this tick. This is the payload right before a hole.
    let r_highest_seq_processable_payload = r_sorted_payloads
        .clone()
        .cross_product(r_highest_seq_with_default)
        .fold(q!(|| -1), q!(| filled_slot, (sorted_payload, highest_seq)| { // Note: This function only works if the input is sorted on seq.
            let next_slot = std::cmp::max(*filled_slot, highest_seq);

            *filled_slot = if sorted_payload.0 == next_slot + 1 {
                sorted_payload.0
            } else {
                *filled_slot
            };
        }));
    // Find all payloads that can and cannot be processed in this tick.
    let r_processable_payloads = r_sorted_payloads
        .clone()
        .cross_product(r_highest_seq_processable_payload.clone())
        .filter(q!(|(sorted_payload, highest_seq)| sorted_payload.0 <= *highest_seq))
        .map(q!(|(sorted_payload, _)| sorted_payload));
    let r_new_non_processable_payloads = r_sorted_payloads
        .clone()
        .cross_product(r_highest_seq_processable_payload.clone())
        .filter(q!(|(sorted_payload, highest_seq)| sorted_payload.0 > *highest_seq))
        .map(q!(|(sorted_payload, _)| sorted_payload))
        .defer_tick(); // Save these, we can process them once the hole has been filled
    r_buffered_payloads_complete_cycle.complete(r_new_non_processable_payloads);

    let r_kv_store = r_processable_payloads
        .all_ticks() // Optimization: all_ticks() + fold() = fold<static>, where the state of the previous fold is saved and persisted values are deleted.
        .fold(q!(|| (HashMap::<u32, String>::new(), -1)), q!(|state, payload| {
            let ref mut kv_store = state.0;
            let ref mut last_seq = state.1;
            kv_store.insert(payload.1, payload.2);
            debug_assert!(payload.0 == *last_seq + 1, "Hole in log between seq {} and {}", *last_seq, payload.0);
            *last_seq = payload.0;
            println!("Replica kv store: {:?}", kv_store);
        }));
    // Update the highest seq for the next tick
    let r_new_highest_seq = r_kv_store
        .clone()
        .map(q!(|(kv_store, highest_seq)| highest_seq))
        .defer_tick();
    r_highest_seq_complete_cycle.complete(r_new_highest_seq);

    (proposers, acceptors, clients, replicas)
}

#[stageleft::entry]
pub fn paxos_runtime<'a>(
    flow: FlowBuilder<'a, CLIRuntime>,
    cli: RuntimeData<&'a HydroCLI<HydroflowPlusMeta>>,
    f: RuntimeData<&'a usize>,
    num_clients_per_node: RuntimeData<&'a usize>,
    kv_num_keys: RuntimeData<&'a usize>,
    kv_value_size: RuntimeData<&'a usize>,
) -> impl Quoted<'a, Hydroflow<'a>> {
    let _ = paxos(&flow, &cli, &cli, &cli, &cli, f, num_clients_per_node, kv_num_keys, kv_value_size);
    flow.extract()
        .optimize_default()
        .with_dynamic_id(q!(cli.meta.subgraph_id))
}

#[stageleft::runtime]
#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use hydro_deploy::{Deployment, HydroflowCrate};
    use hydroflow_plus_cli_integration::{
        DeployClusterSpec, DeployCrateWrapper, DeployProcessSpec,
    };
    use stageleft::RuntimeData;

    #[tokio::test]
    async fn paxos() {
        let deployment = RefCell::new(Deployment::new());
        let localhost = deployment.borrow_mut().Localhost();

        let builder = hydroflow_plus::FlowBuilder::new();
        let f = 1;
        let num_clients = 1;
        let num_replicas = 1;
        let (proposers, acceptors, clients, replicas) = super::paxos(
            &builder,
            &DeployClusterSpec::new(|| {
                (0..(f+1))
                    .map(|_| {
                        deployment.borrow_mut().add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("paxos")
                                .profile("dev"),
                        )
                    })
                    .collect()
            }),
            &DeployClusterSpec::new(|| {
                (0..(2*f+1))
                    .map(|_| {
                        deployment.borrow_mut().add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("paxos")
                                .profile("dev"),
                        )
                    })
                    .collect()
            }),
            &DeployClusterSpec::new(|| {
                (0..num_clients)
                    .map(|_| {
                        deployment.borrow_mut().add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("paxos")
                                .profile("dev"),
                        )
                    })
                    .collect()
            }),
            &DeployClusterSpec::new(|| {
                (0..num_replicas)
                    .map(|_| {
                        deployment.borrow_mut().add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("paxos")
                                .profile("dev"),
                        )
                    })
                    .collect()
            }),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
        );

        // insta::assert_debug_snapshot!(builder.extract().ir());

        let mut deployment = deployment.into_inner();

        deployment.deploy().await.unwrap();

        deployment.start().await.unwrap();

        tokio::signal::ctrl_c().await.unwrap();

        // Expected output:
        /*
        [service/6] Replica kv store: {10: "Hello, Berkeley!"}
        [service/6] Replica kv store: {10: "Hello again, Berkeley!"}
        [service/6] Replica kv store: {10: "Goodbye, Berkeley"}
        [service/6] Replica kv store: {20: "Hello, SF", 10: "Goodbye, Berkeley"}
        [service/6] Replica kv store: {20: "Hello, SF", 10: "No more Berkeley"}
        [service/6] Replica kv store: {20: "Goodbye, SF", 10: "No more Berkeley"}
         */
    }
}