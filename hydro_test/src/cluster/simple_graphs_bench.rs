use hydro_lang::*;
use hydro_std::bench_client::{bench_client, print_bench_results};

use crate::cluster::simple_graphs::{Client, GraphFunction, Server};
pub struct Aggregator;

pub fn simple_graphs_bench<'a>(
    num_clients_per_node: usize,
    server: &Cluster<'a, Server>,
    clients: &Cluster<'a, Client>,
    client_aggregator: &Process<'a, Aggregator>,
    graph: impl GraphFunction<'a>,
) {
    let bench_results = unsafe {
        bench_client(
            clients,
            |payloads| {
                graph(server, payloads.broadcast_bincode(server)).send_bincode_anonymous(clients)
            },
            num_clients_per_node,
        )
    };

    print_bench_results(bench_results, client_aggregator, clients);
}
