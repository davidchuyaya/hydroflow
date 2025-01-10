use std::sync::Arc;

use hydro_deploy::gcp::GcpNetwork;
use hydro_deploy::{Deployment, Host};
use hydro_lang::deploy::TrybuildHost;
use hydro_lang::rewrites::decoupler::Decoupler;
use hydro_lang::rewrites::persist_pullup::persist_pullup;
use hydro_lang::{ClusterId, Location};
use stageleft::q;
use tokio::sync::RwLock;

type HostCreator = Box<dyn Fn(&mut Deployment) -> Arc<dyn Host>>;

struct Sender {}
struct Receiver {}
struct NewNodeType {}

#[tokio::main]
async fn main() {
    let mut deployment = Deployment::new();
    let host_arg = std::env::args().nth(1).unwrap_or_default();

    let create_host: HostCreator = if host_arg == *"gcp" {
        let project = std::env::args().nth(2).unwrap();
        let network = Arc::new(RwLock::new(GcpNetwork::new(&project, None)));

        Box::new(move |deployment| -> Arc<dyn Host> {
            deployment
                .GcpComputeEngineHost()
                .project(&project)
                .machine_type("n2-highcpu-2")
                .image("debian-cloud/debian-11")
                .region("us-west1-a")
                .network(network.clone())
                .add()
        })
    } else {
        let localhost = deployment.Localhost();
        Box::new(move |_| -> Arc<dyn Host> { localhost.clone() })
    };
    let rustflags = "-C opt-level=3 -C codegen-units=1 -C strip=none -C debuginfo=2 -C lto=off";

    let flow = hydro_lang::FlowBuilder::new();
    let send_cluster = flow.cluster::<Sender>();
    let recv_cluster = flow.cluster::<Receiver>();
    let num_senders = 5;
    let num_receivers = 5;

    send_cluster
        .source_iter(q!(0..num_receivers))
        .map(q!(|id| (ClusterId::from_raw(id as u32), format!("Hello, {:?}", id))))
        .filter(q!(|(id, _msg)| id.raw_id % 2 == 0)) // Randomly remove some messages
        .send_bincode_interleaved(&recv_cluster)
        .for_each(q!(|msg| println!("Receiver received: {:?}", msg)));

    let new_location = flow.cluster::<NewNodeType>();
    let mut decoupler = Decoupler {
        nodes_to_decouple: vec![3],
        curr_node_id: 0,
        num_machines: num_senders,
        parent_location: send_cluster.id(),
        location: new_location.id(),
    };

    let _optimized = flow
        .optimize_with(persist_pullup)
        .optimize_with(|leaves| decoupler.decouple(leaves))
        .with_cluster(
            &send_cluster,
            (0..num_senders)
                .map(|_| TrybuildHost::new(create_host(&mut deployment)).rustflags(rustflags)),
        )
        .with_cluster(
            &recv_cluster,
            (0..num_receivers)
                .map(|_| TrybuildHost::new(create_host(&mut deployment)).rustflags(rustflags)),
        )
        .with_cluster(
            &new_location,
            (0..num_senders)
                .map(|_| TrybuildHost::new(create_host(&mut deployment)).rustflags(rustflags)),
        )
        .deploy(&mut deployment);

    // insta::assert_debug_snapshot!(built.ir());

    deployment.deploy().await.unwrap();

    deployment.start().await.unwrap();

    tokio::signal::ctrl_c().await.unwrap();
}
