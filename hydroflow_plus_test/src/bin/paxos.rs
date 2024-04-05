// cannot use hydroflow::main because connect_local_blocking causes a deadlock
#[tokio::main]
async fn main() {
    let f = 1;
    let num_clients_per_node = 1; // TODO: Need to change based on experiment between 1, 50, 100.
    let kv_num_keys = 1;
    let kv_value_size = 16;
    
    hydroflow_plus::launch!(|ports| hydroflow_plus_test::paxos::paxos_runtime!(
        ports,
        &f,
        &num_clients_per_node,
        &kv_num_keys,
        &kv_value_size,
    ))
    .await;
}
