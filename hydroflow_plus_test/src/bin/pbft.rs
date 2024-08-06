#[tokio::main]
async fn main() {
    let f: u32 = 1;
    // let ports = hydroflow_plus::util::cli::init().await;

    hydroflow_plus::launch!(|ports| hydroflow_plus_test::cluster::pbft::pbft_runtime!(ports, &f))
        .await;
}
