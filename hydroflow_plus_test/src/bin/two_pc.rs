#[tokio::main]
async fn main() {
    // let ports = hydroflow_plus::util::cli::init().await;

    hydroflow_plus::launch!(|ports| hydroflow_plus_test::cluster::two_pc::two_pc_runtime!(ports))
        // hydroflow_plus::launch!(|ports| hydroflow_plus_test::cluster::pbft::pbft_runtime!(ports, &f, &time_duration))
        .await;
}