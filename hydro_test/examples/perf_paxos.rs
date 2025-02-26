use hydro_deploy::Deployment;
use hydro_lang::deploy::DeployCrateWrapper;
use hydro_lang::ir::deep_clone;
use hydro_lang::q;
use hydro_lang::rewrites::analyze_perf_and_counters::{analyze_cluster_results, track_cluster_usage_cardinality, perf_cluster_specs};
use hydro_lang::rewrites::{insert_counter, persist_pullup};
use hydro_test::cluster::paxos::{CorePaxos, PaxosConfig};

#[tokio::main]
async fn main() {
    let mut deployment = Deployment::new();
    let host_arg = std::env::args().nth(1).unwrap_or_default();

    let builder = hydro_lang::FlowBuilder::new();
    let f = 1;
    let num_clients = 1;
    let num_clients_per_node = 100; // Change based on experiment between 1, 50, 100.
    let median_latency_window_size = 1000;
    let checkpoint_frequency = 1000; // Num log entries
    let i_am_leader_send_timeout = 5; // Sec
    let i_am_leader_check_timeout = 10; // Sec
    let i_am_leader_check_timeout_delay_multiplier = 15;

    let proposers = builder.cluster();
    let acceptors = builder.cluster();

    let (clients, replicas) = hydro_test::cluster::paxos_bench::paxos_bench(
        &builder,
        num_clients_per_node,
        median_latency_window_size,
        checkpoint_frequency,
        f,
        f + 1,
        |replica_checkpoint| CorePaxos {
            proposers: proposers.clone(),
            acceptors: acceptors.clone(),
            replica_checkpoint: replica_checkpoint.broadcast_bincode(&acceptors),
            paxos_config: PaxosConfig {
                f,
                i_am_leader_send_timeout,
                i_am_leader_check_timeout,
                i_am_leader_check_timeout_delay_multiplier,
            },
        },
    );

    let counter_output_duration = q!(std::time::Duration::from_secs(1));

    let optimized = builder
        .optimize_with(persist_pullup::persist_pullup)
        .optimize_with(|leaf| {
            insert_counter::insert_counter(leaf, counter_output_duration);
        });
    let mut ir = deep_clone(optimized.ir());
    let nodes = optimized
        .with_cluster(
            &proposers,
            perf_cluster_specs(&host_arg, &mut deployment, "proposer", f + 1),
        )
        .with_cluster(
            &acceptors,
            perf_cluster_specs(&host_arg, &mut deployment, "acceptor", 2 * f + 1),
        )
        .with_cluster(
            &clients,
            perf_cluster_specs(&host_arg, &mut deployment, "client", num_clients),
        )
        .with_cluster(
            &replicas,
            perf_cluster_specs(&host_arg, &mut deployment, "replica", f + 1),
        )
        .deploy(&mut deployment);

    deployment.deploy().await.unwrap();

    let (mut usage_out, mut cardinality_out) = track_cluster_usage_cardinality(&nodes).await;

    deployment
        .start_until(async {
            std::io::stdin().read_line(&mut String::new()).unwrap();
        })
        .await
        .unwrap();

    analyze_cluster_results(&nodes, &mut ir, &mut usage_out, &mut cardinality_out).await;
    hydro_lang::ir::dbg_dedup_tee(|| {
        println!("{:#?}", ir);
    });
}
