use hydro_lang::*;
use hydro_std::bench_client::{bench_client, print_bench_results};

use crate::cluster::paxos_bench::inc_u32_workload_generator;
use crate::cluster::simple_graphs::{Client, GraphFunction, Server};
pub struct Aggregator;

pub fn simple_graphs_bench<'a>(
    num_clients_per_node: usize,
    server: &Cluster<'a, Server>,
    clients: &Cluster<'a, Client>,
    client_aggregator: &Process<'a, Aggregator>,
    graph: impl GraphFunction<'a>,
) {
    let bench_results = bench_client(
        clients,
        inc_u32_workload_generator,
        |payloads| {
            graph(
                server,
                payloads
                    .broadcast_bincode(server, nondet!(/** Test */))
                    .into(),
            )
            .demux_bincode(clients)
            .values()
        },
        num_clients_per_node,
        nondet!(/** bench */),
    );

    print_bench_results(bench_results, client_aggregator, clients);
}
