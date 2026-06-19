use std::sync::Arc;

use clap::{ArgAction, Parser};
use hydro_deploy::gcp::GcpNetwork;
use hydro_deploy::rust_crate::tracing_options::{
    AL2_PERF_SETUP_COMMAND, DEBIAN_PERF_SETUP_COMMAND, TracingOptions,
};
use hydro_deploy::{AwsNetwork, Deployment, Host};
use hydro_lang::deploy::TrybuildHost;
use hydro_lang::location::Location;
use hydro_lang::viz::config::GraphConfig;
use hydro_test::cluster::paxos::{CorePaxos, PaxosConfig};
use stageleft::q;

type HostCreator = Box<dyn Fn(&mut Deployment, &str) -> Arc<dyn Host>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, group(
    clap::ArgGroup::new("cloud")
        .args(&["gcp", "aws"])
        .multiple(false)
))]
struct Args {
    #[command(flatten)]
    graph: GraphConfig,

    /// Include CPU tracing profiling (takes a long time to download)
    #[arg(long, action = ArgAction::SetTrue)]
    tracing: bool,

    /// Use GCP for deployment (provide project name)
    #[arg(long)]
    gcp: Option<String>,

    /// Use AWS, make sure credentials are set up
    #[arg(long, action = ArgAction::SetTrue)]
    aws: bool,
}

const AWS_REGION: &str = "us-east-1";
const AWS_INSTANCE_AMI: &str = "ami-0521cb2d60cfbb1a6"; // Amazon Linux 2023
const AWS_INSTANCE_TYPE: &str = "m5.2xlarge"; // 8 vCPU, 32 GB RAM
const NUM_CORES: usize = 8;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut deployment = Deployment::new();

    let create_host: HostCreator = if let Some(project) = &args.gcp {
        let network = GcpNetwork::new(project, None);
        let project = project.clone();

        Box::new(move |deployment, _name| -> Arc<dyn Host> {
            deployment
                .GcpComputeEngineHost()
                .project(&project)
                .machine_type("n2-standard-4")
                .image("debian-cloud/debian-11")
                .region("us-central1-c")
                .network(network.clone())
                .add()
        })
    } else if args.aws {
        let network = AwsNetwork::new(AWS_REGION, None);

        Box::new(move |deployment, name| -> Arc<dyn Host> {
            deployment
                .AwsEc2Host()
                .region(AWS_REGION)
                .instance_type(AWS_INSTANCE_TYPE)
                .ami(AWS_INSTANCE_AMI)
                .network(network.clone())
                .display_name(name.to_string())
                .add()
        })
    } else {
        let localhost = deployment.Localhost();
        Box::new(move |_, _name| -> Arc<dyn Host> { localhost.clone() })
    };

    let mut builder = hydro_lang::compile::builder::FlowBuilder::new();
    let f = 1;
    let num_clients = 2;
    let num_clients_per_node = 50; // Change based on experiment between 1, 50, 100.
    let checkpoint_frequency = 1000; // Num log entries
    let i_am_leader_send_timeout = 5; // Sec
    let i_am_leader_check_timeout = 10; // Sec
    let i_am_leader_check_timeout_delay_multiplier = 15;
    let print_result_frequency = 1000; // Millis

    let proposers = builder.cluster();
    let acceptors = builder.cluster();
    let clients = builder.cluster();
    let client_aggregator = builder.process();
    let replicas = builder.cluster();

    hydro_test::cluster::paxos_bench::paxos_bench(
        checkpoint_frequency,
        f,
        f + 1,
        CorePaxos {
            proposers: proposers.clone(),
            acceptors: acceptors.clone(),
            paxos_config: PaxosConfig {
                f,
                i_am_leader_send_timeout,
                i_am_leader_check_timeout,
                i_am_leader_check_timeout_delay_multiplier,
            },
        },
        &clients,
        clients.singleton(q!(std::env::var("NUM_CLIENTS_PER_NODE")
            .unwrap()
            .parse::<usize>()
            .unwrap())),
        &client_aggregator,
        &replicas,
        print_result_frequency / 10,
        print_result_frequency,
        hydro_std::bench_client::pretty_print_bench_results,
    );

    // Extract the IR for graph visualization
    let built = builder.finalize();

    match built.generate_graph(&args.graph) {
        Ok(Some(_)) => return,
        Ok(None) => {}
        Err(err) => {
            eprintln!("Failed to generate graph: {err}");
            std::process::exit(1);
        }
    }

    // Optimize the flow before deployment to remove marker nodes
    let optimized = built.with_default_optimize();

    let rustflags = if args.gcp.is_some() || args.aws {
        "-C opt-level=3 -C codegen-units=1 -C strip=none -C debuginfo=2 -C lto=off -C link-args=--no-rosegment"
    } else {
        "-C opt-level=3 -C codegen-units=1 -C strip=none -C debuginfo=2 -C lto=off"
    };

    let frequency = 128;
    let setup_command = if args.gcp.is_some() {
        DEBIAN_PERF_SETUP_COMMAND
    } else if args.aws {
        AL2_PERF_SETUP_COMMAND
    } else {
        ""
    };
    let create_trybuild_host = |host: Arc<dyn Host + 'static>, name: &str, i: usize| {
        let mut tbh = TrybuildHost::new(host)
            .rustflags(rustflags)
            .display_name(format!("{name}{i}"));
        // Pin main thread to core 0, networking to core 1 on remote machines
        if args.gcp.is_some() || args.aws {
            tbh = tbh.networking_cores(NUM_CORES - 1);
        }
        if args.tracing {
            tbh = tbh.tracing(
                TracingOptions::builder()
                    .perf_raw_outfile(format!("{name}{i}.perf.data"))
                    .samply_outfile(format!("{name}{i}.profile"))
                    .fold_outfile(format!("{name}{i}.data.folded"))
                    .flamegraph_outfile(format!("{name}{i}.svg"))
                    .frequency(frequency)
                    .setup_command(setup_command)
                    .build(),
            );
        }
        tbh
    };

    let clients_key = clients.id().key();
    let _nodes = optimized
        .with_cluster(
            &proposers,
            (0..f + 1).map(|i| {
                create_trybuild_host(
                    create_host(&mut deployment, &format!("proposers{i}")),
                    "proposers",
                    i,
                )
            }),
        )
        .with_cluster(
            &acceptors,
            (0..2 * f + 1).map(|i| {
                create_trybuild_host(
                    create_host(&mut deployment, &format!("acceptors{i}")),
                    "acceptors",
                    i,
                )
            }),
        )
        .with_cluster(
            &clients,
            (0..num_clients).map(|i| {
                create_trybuild_host(
                    create_host(&mut deployment, &format!("clients{i}")),
                    "clients",
                    i,
                )
                .env("NUM_CLIENTS_PER_NODE", num_clients_per_node.to_string())
            }),
        )
        .with_process(
            &client_aggregator,
            create_trybuild_host(
                create_host(&mut deployment, "client_aggregator0"),
                "client_aggregator",
                0,
            ),
        )
        .with_cluster(
            &replicas,
            (0..f + 1).map(|i| {
                create_trybuild_host(
                    create_host(&mut deployment, &format!("replicas{i}")),
                    "replicas",
                    i,
                )
            }),
        )
        .set_batch_limit_from(clients_key, 1)
        .deploy(&mut deployment);

    deployment.deploy().await.unwrap();

    deployment
        .start_until(async {
            std::io::stdin().read_line(&mut String::new()).unwrap();
        })
        .await
        .unwrap();
}
