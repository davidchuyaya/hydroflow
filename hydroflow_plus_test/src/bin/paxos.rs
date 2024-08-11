// cannot use hydroflow::main because connect_local_blocking causes a deadlock
use std::time::Duration;

#[tokio::main]
async fn main() {
    let f = 1;
    let num_clients_per_node = 1; // TODO: Need to change based on experiment between 1, 50, 100.
    let median_latency_window_size = 1000;
    let i_am_leader_send_timeout = Duration::from_secs(5);
    let i_am_leader_check_timeout = Duration::from_secs(10);
    let i_am_leader_check_timeout_delay_multiplier = 15;

    hydroflow_plus::launch!(|ports| hydroflow_plus_test::cluster::paxos::paxos_runtime!(
        ports,
        &f,
        &num_clients_per_node,
        &median_latency_window_size,
        &i_am_leader_send_timeout,
        &i_am_leader_check_timeout,
        &i_am_leader_check_timeout_delay_multiplier,
    ))
    .await;
}
