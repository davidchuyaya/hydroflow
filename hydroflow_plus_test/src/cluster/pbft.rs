// cargo test -p hydroflow_plus_test pbft -- --nocapture
use std::collections::HashSet;
use hydroflow_plus::util::cli::HydroCLI;
use hydroflow_plus::*;
use hydroflow_plus_cli_integration::{CLIRuntime, HydroflowPlusMeta};
use serde::{Deserialize, Serialize};
use stageleft::*;
use std::time::{Duration, SystemTime};
use stream::Windowed;

// use hydroflow_plus_cli_integration::*;
// use std::collections::HashSet;
// use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct ViewNumber {
    view_number: u32,
    id: u32,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
struct SeqNumber {
    sequence_number: u32,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
struct PrePrepare {
    view_number: u32,
    sequence_number: u32,
    request: String,
    signature: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct Prepare {
    view_number: u32,
    sequence_number: u32,
    request: String,
    signature: u32,
    id: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct Prepared {
    view_number: u32,
    sequence_number: u32,
    request: String,
    signature: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct Commit {
    view_number: u32,
    sequence_number: u32,
    request: String,
    signature: u32,
    id: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct Committed {
    view_number: u32,
    sequence_number: u32,
    request: String,
    signature: u32,
}

pub fn pbft<'a, D: Deploy<'a, ClusterId = u32>>(
    flow: &FlowBuilder<'a, D>,
    client_spec: &impl ProcessSpec<'a, D>,
    replicas_spec: &impl ClusterSpec<'a, D>,
    f: RuntimeData<&'a u32>,
    time_duration: RuntimeData<&'a Duration>, 
) -> (D::Process, D::Cluster) {
    // Assume single client.
    let client = flow.process(client_spec);
    // Assume 4 replicas. f = 1. If want more or less participant, fix line 27 of examples/pbft.rs
    let replicas = flow.cluster(replicas_spec);
    // each replica have it's own id stored in r_id.
    let r_id = replicas.self_id();


    /* very simple primary election, assume primary has the largest id in cluster */
    let (have_primary_complete_cycle, have_primary) = flow.cycle(&replicas);
    // let (is_primary_complete_cycle, is_primary) = flow.cycle(&replicas);

    // broadcast view number to all replicas assume view number is the id of the replica.
    let r_view_number = flow
    .source_iter(&replicas, q!([r_id]))
    .broadcast_bincode(&replicas);

    // replica receive the id from replicas, calculate the largest id in cluster, also count how many it received.
    let r_largest_id = r_view_number
    .all_ticks()
    .sample_every(q!(*time_duration))
    .fold(q!(|| (0, 0, 0)), q!(|(count, largest_view, pid), (new_pid, view)| {
        *count += 1;
        if view > *largest_view {
            *largest_view = view;
            *pid = new_pid;
        }
    }));

    // replica make conclusion if it received all ids from all replicas.
    let r_primary_id = r_largest_id
    .filter_map(q!(move |(count, view, pid)| {
        if count == 4 {
            Some(ViewNumber{view_number: view, id: pid})
        } else {
            None
        }
    }));

    have_primary_complete_cycle.complete(r_primary_id);    
    // let is_primary = have_primary
    // .clone()
    // .filter(q!(move |view: &ViewNumber| view.id == r_id))
    // .all_ticks();
    /* End simple primary election */


    

    // assume the primary node is 0. could be fixed by sample_every.
    // is_primary is a single boolean persisted stream that indicate whether the node is primary or not.
    let is_primary = flow
        .source_iter(&replicas, q!([0u32]))
        .filter(q!(move |id: &u32| *id == r_id))
        .all_ticks();
    // have_primary is a single persisted stream that stores the current view number and the primary node.
    let have_primary = flow
        .source_iter(
            &replicas,
            q!([ViewNumber {
                view_number: 4,
                id: 0
            }]),
        )
        .all_ticks();


    // phase pre-prepare

    // some request send by client, send to primary.
    let client_request = flow.source_iter(
        &client,
        q!([
            String::from("request1"),
            String::from("request2"),
            String::from("request3")
        ]),
    );

    let p_receive_request = client_request.broadcast_bincode(&replicas);
    // replicas ignore the request from client if it is not the leader.
    let primary_request = p_receive_request
        .tick_batch()
        .continue_if(is_primary.clone());
    primary_request.clone().for_each(q!(|request: String| println!("primary receive request: {}", request)));


    // have a sequence number that record the current sequnce number.
    let (primary_sequence_number_complete_cycle, primary_sequence_number) = flow.cycle(&replicas);

    let first_sequence_number = flow
        .source_iter(&replicas, q!([0u32]))
        .continue_if(is_primary.clone())
    ;
    let next_num = primary_sequence_number.union(first_sequence_number);

    // primary send pre-prepare message to all replicas.
    // first enumerate it and crossproduct with the current primary node and sequence number.
    let primary_pre_prepare_message: Stream<PrePrepare, Windowed, <D as Deploy<'a>>::Cluster>
    = primary_request
    .enumerate()
    .cross_product(have_primary.clone())
    .cross_product(next_num.clone())
    .map(q!(move |(((index, request), view), seq_num): (((usize, String), ViewNumber), u32)| {
        PrePrepare{view_number: view.view_number, sequence_number: seq_num + index as u32, request: request, signature: view.id.clone()}
    }
    ));

    let primary_pre_prepare_count = primary_pre_prepare_message.clone().count();
    // increase the sequence number by the size of this batch, but increament it by next tick.
    let primary_next_sequence_number = primary_pre_prepare_count
        .cross_product(next_num)
        .map(q!(
            move |(count, seq_num): (usize, u32)| seq_num + count as u32
        ))
        .defer_tick();
    primary_sequence_number_complete_cycle.complete(primary_next_sequence_number);
    
    // other replicas receive the pre-prepare message from primary.
    let r_receive_pre_prepare = primary_pre_prepare_message
        .broadcast_bincode_interleaved(&replicas)
        .tick_batch();


    // // have a cycle represent the preprepare message sended by the replicas.
    let (r_sended_pre_prepares_complete_cycle, r_sended_pre_prepares) = flow.cycle(&replicas);


    // other replicas verify the pre-prepare message is send by the current primary and the view number is the same.
    let r_valid_pre_prepare = r_receive_pre_prepare
        .cross_product(have_primary.clone())
        .anti_join(r_sended_pre_prepares) //remove anything already sended
        .filter_map(q!(move |(pre_prepare, view): (PrePrepare, ViewNumber)| {
            if pre_prepare.view_number == view.view_number && pre_prepare.signature == view.id {
                Some(pre_prepare)
            } else {
                None
            }
        }));

    // replicas edit the current valid pre-prepare message to prepare message.
    let r_prepare_message = r_valid_pre_prepare
        .clone()
        .map(q!(move |pre_prepare: PrePrepare| Prepare {
            view_number: pre_prepare.view_number,
            sequence_number: pre_prepare.sequence_number,
            request: pre_prepare.request.clone(),
            signature: pre_prepare.signature.clone(),
            id: r_id.clone()
        }));

    // have a persisted pre prepare stores all the pre-prepare messages.
    let r_persisted_pre_prepares = r_valid_pre_prepare.clone().all_ticks();
    r_valid_pre_prepare.clone().for_each(q!(|pre_prepare| println!(
        "replica receive valid pre-prepare message: {:?}",
        pre_prepare
    )));
    // question 1: I think I need a all_tick here?
    // question 2: I think I can replace r_persisted_pre_prepares with what is inside cycle? but it have defer tick. 
    // what if : when it receive the valid prepare message, but it haven't save the pre-prepare message because of the defer?
    r_sended_pre_prepares_complete_cycle.complete(r_persisted_pre_prepares.clone().defer_tick());
    // end phase pre-prepare


    /* phase prepare */
    // replicas broadcast prepare message to all other replicas.
    let r_receive_prepare = r_prepare_message.broadcast_bincode_interleaved(&replicas).tick_batch().unique();
    
    // have a cycle represent the prepare message sended by the replicas.
    let (r_sended_prepares_complete_cycle, r_sended_prepares) = flow.cycle(&replicas);

    // check if the prepare message is valid by checking the view number and sequence number, and also if it is matched the previous pre-prepare request.
    let r_receive_valid_prepare = r_receive_prepare
    .map(q!(move |prepare: Prepare| (Prepared{view_number: prepare.view_number, sequence_number: prepare.sequence_number, request: prepare.request.clone(), signature: prepare.signature}, prepare.id))) // seperate out the id
    .anti_join(r_sended_prepares) //remove anything already sended
    .map(q!(move |(prepared, id): (Prepared, u32)| Prepare{view_number: prepared.view_number, sequence_number: prepared.sequence_number, request: prepared.request.clone(), signature: prepared.signature, id: id}))// add back the id
    .cross_product(r_persisted_pre_prepares)
    .filter_map(q!(move |(prepare, pre_prepare): (Prepare, PrePrepare)| {
        if prepare.view_number == pre_prepare.view_number && prepare.sequence_number == pre_prepare.sequence_number && prepare.request == pre_prepare.request && prepare.signature == pre_prepare.signature {
            Some(prepare)
        } else {
            None
        }
    })
    );
    // count the valid prepare message received by the replica, do not count the prepare message sent by itself.
    let r_received_valid_prepare_count = r_receive_valid_prepare
    .clone()
    .map(q!(move |prepare: Prepare| (prepare.request.clone(), prepare)))
    .all_ticks()
    .fold_keyed(q!(|| HashSet::new()), q!(move |count, prepare: Prepare| {
        count.insert(prepare.id);
    }));

    // save the valid prepare message only if the replica received 2f (f = 1) valid prepare message. (does not count itself)
    let r_valid_prepare = r_receive_valid_prepare
    .cross_product(r_received_valid_prepare_count)
    .filter_map(q!(move |(prepare, (request, count)): (Prepare, (String, HashSet<u32>))| {
        if count.len() >= (2*f+1).try_into().unwrap() && request == prepare.request {
            Some(Prepared{view_number: prepare.view_number, sequence_number: prepare.sequence_number, request: prepare.request.clone(), signature: prepare.signature})
        } else {
            None
        }
    })
    );
    let r_valid_prepare_persisted = r_valid_prepare.clone().all_ticks();
    r_sended_prepares_complete_cycle.complete(r_valid_prepare_persisted.clone().defer_tick());
    r_valid_prepare.clone().unique().for_each(q!(|prepare| println!("replica receive valid prepare message: {:?}", prepare)));

    /* end phase prepare */


    /* phase commit */
    // replicas broadcast commit message to all other replicas.
    let r_commit_message = r_valid_prepare
    .map(q!(move |prepared: Prepared| Commit{view_number: prepared.view_number, sequence_number: prepared.sequence_number.clone(), request: prepared.request.clone(), signature: prepared.signature.clone(), id: r_id.clone()}));
    let r_receive_commit = r_commit_message.broadcast_bincode_interleaved(&replicas).tick_batch().unique();

    // have a cycle represent the commit message sended by the replicas.
    let (r_sended_commits_complete_cycle, r_sended_commits) = flow.cycle(&replicas);

    // r_receive_commit.clone().for_each(q!(|commit: Commit| println!("replica receive commit message: {:?}", commit)));
    // check if the commit message is valid by checking the view number and sequence number, and also if it is matched the previous prepare request.
    let r_receive_valid_commit = r_receive_commit
    .map(q!(move |commit: Commit| (Committed{view_number: commit.view_number, sequence_number: commit.sequence_number, request: commit.request.clone(), signature: commit.signature}, commit.id))) // seperate out the id
    .anti_join(r_sended_commits) //remove anything already sended
    .map(q!(move |(committed, id): (Committed, u32)| Commit{view_number: committed.view_number, sequence_number: committed.sequence_number, request: committed.request.clone(), signature: committed.signature, id: id}))// add back the id
    .cross_product(r_valid_prepare_persisted.clone())
    .filter_map(q!(move |(commit, prepared): (Commit, Prepared)| {
        if commit.view_number == prepared.view_number && commit.sequence_number == prepared.sequence_number && commit.request == prepared.request && commit.signature == prepared.signature {
            Some(commit)
        } else {
            None
        }
    })
    );

    // count the valid commit message received by the replica, do not count the commit message sent by itself.
    let r_received_valid_commit_count = r_receive_valid_commit
    .clone()
    .map(q!(move |commit: Commit| (commit.request.clone(), commit)))
    .all_ticks()
    .fold_keyed(q!(|| HashSet::new()), q!(move |count, commit: Commit| {
        count.insert(commit.id);
    }));

    // save the valid commit message only if the replica received 2 (f = 1) valid commit message. (total 2f + 1 replicas accepted the request)
    let r_valid_commit = r_receive_valid_commit
    .cross_product(r_received_valid_commit_count)
    .filter_map(q!(move |(commit, (request, count)): (Commit, (String, HashSet<u32>))| {
        if count.len() >= (2*f+1).try_into().unwrap() && request == commit.request {
            Some(Committed{view_number: commit.view_number, sequence_number: commit.sequence_number, request: commit.request.clone(), signature: commit.signature})
        } else {
            None
        }
    })
    );
    r_sended_commits_complete_cycle.complete(r_valid_commit.clone().all_ticks().defer_tick());
    r_valid_commit.clone().unique().for_each(q!(|commit: Committed| println!("replica receive 2f+1 valid commit message, request commit: {:?}", commit)));
    /* end phase commit */
    

    // client saved commited messages.
    let (c_committed_complete_cycle, c_committed) = flow.cycle(&client);

    // replicas send back the commited message to the client.
    let c_receive_commit = r_valid_commit.send_bincode(&client).tick_batch().unique();
    // client receive reply and count it 
    let c_received_commit_count = c_receive_commit
    .map(q!(move |(id, commit): (u32, Committed)| (commit, id))) // switch to (commit, id) pair // remove anything already committed
    .all_ticks()
    .fold_keyed(q!(|| HashSet::new()), q!(move |count, id: u32| {
        count.insert(id);
    }));
    
    // client valid the commit message only if it received f+1 commit message.
    let c_valid_commit = c_received_commit_count
    .anti_join(c_committed)
    .filter_map(q!(move |(commit, count): (Committed, HashSet<u32>)| {
        if count.len() >= (f+1).try_into().unwrap() {
            Some(commit)
        } else {
            None
        }
    }));
    c_committed_complete_cycle.complete(c_valid_commit.clone().all_ticks().defer_tick());
    c_valid_commit.unique().for_each(q!(|commit| println!("client receive f+1 valid commit message, request commit: {:?}", commit)));


    (client, replicas)
}

#[stageleft::entry]
pub fn pbft_runtime<'a>(
    flow: FlowBuilder<'a, CLIRuntime>,
    cli: RuntimeData<&'a HydroCLI<HydroflowPlusMeta>>,
    f: RuntimeData<&'a u32>,
    time_duration: RuntimeData<&'a Duration>,
) -> impl Quoted<'a, Hydroflow<'a>> {
    let _ = pbft(&flow, &cli, &cli, f, time_duration);
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
    async fn pbft() {
        let deployment = RefCell::new(Deployment::new());
        let localhost = deployment.borrow_mut().Localhost();
        let builder = hydroflow_plus::FlowBuilder::new();
        let profile = "profile";
        let f: u32 = 1;

        let (client, replicas) = super::pbft(
            &builder,
            &DeployProcessSpec::new(|| {
                let mut deployment = deployment.borrow_mut();
                deployment.add_service(
                    HydroflowCrate::new(".", localhost.clone())
                        .bin("pbft")
                        .profile(profile)
                        .display_name("client"),
                )
            }),
            &DeployClusterSpec::new(|| {
                let mut deployment = deployment.borrow_mut();
                (0..4)
                    .map(|idx| {
                        deployment.add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("pbft")
                                .profile(profile)
                                .display_name(format!("replicas/{}", idx)),
                        )
                    })
                    .collect()
            }),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
        );

        let mut deployment = deployment.into_inner();

        deployment.deploy().await.unwrap();

        deployment.start().await.unwrap();

        tokio::signal::ctrl_c().await.unwrap()
    }
}
