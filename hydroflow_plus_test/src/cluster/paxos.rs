use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use tokio::time::Instant;
use watermill::quantile::RollingQuantile;
use watermill::stats::Univariate;

use hydroflow_plus::*;
use hydroflow_plus::util::cli::HydroCLI;
use hydroflow_plus_cli_integration::*;
use stageleft::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct ClientPayload {
    key: u32,
    value: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct ReplicaPayload { // Note: Important that seq is the first member of the struct for sorting
    seq: i32,
    key: u32,
    value: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct Ballot { // Note: Important that num comes before id, since Ord is defined lexicographically
    num: u32,
    id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct P1a {
    ballot: Ballot,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct LogValue {
    ballot: Ballot,
    value: ClientPayload,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct P1b {
    ballot: Ballot,
    max_ballot: Ballot,
    accepted: HashMap::<i32, LogValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct P2a {
    ballot: Ballot,
    slot: i32,
    value: ClientPayload,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct P2b {
    ballot: Ballot,
    max_ballot: Ballot,
    slot: i32,
    value: ClientPayload,
}

// Important: By convention, all relations that represent booleans either have a single "true" value or nothing.
// This allows us to use the continue_if_exists() and continue_if_empty() operators as if they were if (true) and if (false) statements.
pub fn paxos<'a, D: Deploy<'a, ClusterId = u32>>(
    flow: &FlowBuilder<'a, D>,
    proposers_spec: &impl ClusterSpec<'a, D>,
    acceptors_spec: &impl ClusterSpec<'a, D>,
    clients_spec: &impl ClusterSpec<'a, D>,
    replicas_spec: &impl ClusterSpec<'a, D>,
    f: RuntimeData<&'a usize>,
    num_clients_per_node: RuntimeData<&'a usize>,
    kv_num_keys: RuntimeData<&'a usize>, // How many unique keys are expected by the state machine kv store?
    kv_value_size: RuntimeData<&'a usize>, // Number of bytes in each value in the kv store
    median_latency_window_size: RuntimeData<&'a usize>, // How many latencies to keep in the window for calculating the median
    i_am_leader_send_timeout: RuntimeData<&'a Duration>, // How often to heartbeat
    i_am_leader_check_timeout: RuntimeData<&'a Duration>, // How often to check if heartbeat expired
    i_am_leader_check_timeout_delay_multiplier: RuntimeData<&'a usize>, // Initial delay, multiplied by proposer pid, to stagger proposers checking for timeouts
) -> (D::Cluster, D::Cluster, D::Cluster, D::Cluster) {
    let proposers = flow.cluster(proposers_spec);
    let acceptors = flow.cluster(acceptors_spec);
    let clients = flow.cluster(clients_spec);
    let replicas = flow.cluster(replicas_spec);


    /* Clients. All relations for clients will be prefixed with c. All ClientPayloads will contain the virtual client number as key and the client's machine ID (to string) as value. */

    
    let c_id = clients.self_id();
    let (r_to_clients_payload_applied_cycle, r_to_clients_payload_applied) = flow.cycle(&clients);
    let (p_to_clients_leader_elected_cycle, p_to_clients_leader_elected) = flow.cycle(&clients);
    let c_leader_ballots = p_to_clients_leader_elected
        .map(q!(|(leader_id, leader_ballot_num): (u32, u32)| Ballot { num: leader_ballot_num, id: leader_id })); 
    // c_leader_ballots.clone().for_each(q!(|ballot: Ballot| println!("Client notified that leader was elected: {:?}", ballot)));
    // r_to_clients_payload_applied.clone().for_each(q!(|payload: ReplicaPayload| println!("Client received payload: {:?}", payload)));
    // TODO: Timeout logic per client in case some commits never make it to replicas because of leader reelection
    // Only keep the latest leader
    let c_max_leader_ballot = c_leader_ballots
        .all_ticks()
        .reduce(q!(|curr_max_ballot: &mut Ballot, new_ballot: Ballot| {
            if new_ballot > *curr_max_ballot {
                *curr_max_ballot = new_ballot;
            }
        }));
    let c_new_leader_ballot = c_max_leader_ballot
        .clone()
        .delta();
    // Whenever the leader changes, make all clients send a message
    let c_new_payloads_when_leader_elected = c_new_leader_ballot
        .clone()
        .flat_map(q!(move |leader_ballot: Ballot| (0..*num_clients_per_node).map(move |i| (leader_ballot.id, ClientPayload { key: i as u32, value: c_id }))));
    // Whenever replicas confirm that a payload was committed, send another payload
    let c_new_payloads_when_committed = r_to_clients_payload_applied
        .clone()
        .tick_batch()
        .zip_with_singleton(c_max_leader_ballot.clone())
        .map(q!(|(payload, leader_ballot): (ReplicaPayload, Ballot)| (leader_ballot.id, ClientPayload { key: payload.key, value: payload.value })));
    let c_to_proposers = c_new_payloads_when_leader_elected
        .union(c_new_payloads_when_committed)
        .send_bincode_interleaved(&proposers);

    /* Track statistics */
    let (c_timers_complete_cycle, c_timers) = flow.cycle(&clients);
    let c_new_timers_when_leader_elected = c_new_leader_ballot
        .map(q!(|_: Ballot| SystemTime::now()))
        .flat_map(q!(move |now: SystemTime| (0..*num_clients_per_node).map(move |virtual_id| (virtual_id, now))));
    let c_updated_timers = r_to_clients_payload_applied
        .clone()
        .tick_batch()
        .map(q!(|payload: ReplicaPayload| (payload.key as usize, SystemTime::now())));
    let c_new_timers = c_timers
        .clone() // Update c_timers in tick+1 so we can record differences during this tick (to track latency)
        .union(c_new_timers_when_leader_elected)
        .union(c_updated_timers.clone())
        .reduce_keyed(q!(|curr_time: &mut SystemTime, new_time: SystemTime| {
            if new_time > *curr_time {
                *curr_time = new_time;
            }
        }))
        .defer_tick();
    c_timers_complete_cycle.complete(c_new_timers);

    let c_stats_output_timer = flow.source_interval(&clients, q!(Duration::from_secs(1)))
        .tick_batch();

    let c_latency_reset = c_stats_output_timer
        .clone()
        .map(q!(|_: ()| None))
        .defer_tick();

    let c_latencies = c_timers
        .join(c_updated_timers)
        .map(q!(|(virtual_id, (prev_time, curr_time)): (usize, (SystemTime, SystemTime))| Some(curr_time.duration_since(prev_time).unwrap().as_micros())))
        .union(c_latency_reset)
        .all_ticks()
        .fold(q!(|| (Rc::new(RefCell::new(RollingQuantile::<f64>::new(0.5_f64, *median_latency_window_size).unwrap())), false)), q!(|(latencies, has_any_value): &mut (Rc<RefCell<RollingQuantile<f64>>>, bool), latency: Option<u128>| { // 0.5 quantile = median
            if let Some(latency) = latency {
                latencies.borrow_mut().update(latency as f64);
                *has_any_value = true;
            } else {
                *latencies.borrow_mut() = RollingQuantile::<f64>::new(0.5_f64, *median_latency_window_size).unwrap();
            }
        }))
        .map(q!(|(latencies, has_any_value): (Rc<RefCell<RollingQuantile<f64>>>, bool)| {
            if has_any_value {
                latencies.borrow_mut().get()
            } else {
                -1.0
            }
        }));
    let c_throughput_new_batch = r_to_clients_payload_applied
        .clone()
        .tick_batch()
        .count()
        .continue_unless(c_stats_output_timer.clone())
        .map(q!(|batch_size: usize| (batch_size, false)));

    let c_throughput_reset = c_stats_output_timer
        .clone()
        .map(q!(|_: ()| (0, true)))
        .defer_tick();

    let c_throughput = c_throughput_new_batch
        .union(c_throughput_reset)
        .all_ticks()
        .fold(q!(|| (0,0)), q!(|(total, num_ticks): &mut (u32, u32), (batch_size, reset): (usize, bool)| {
            if reset {
                *total = 0;
                *num_ticks = 0;
            }
            else {
                *total += batch_size as u32;
                *num_ticks += 1;
            }
        }));
    c_stats_output_timer
        .zip_with_singleton(c_latencies)
        .zip_with_singleton(c_throughput)
        .for_each(q!(|((_, latencies_us), (throughput, num_ticks)): (((), f64), (u32, u32))| {
            println!("Median latency: {}ms", latencies_us / 1000.0);
            println!("Throughput: {} requests/s", throughput);
            println!("Num ticks per second: {}", num_ticks);
        }));
    /* End track statistics */


    /* Proposers. All relations for proposers will be prefixed with p. */


    flow.source_iter(&proposers, q!(["Proposers say hello"]))
        .for_each(q!(|s| println!("{}", s)));

    /* State */
    let p_id = proposers.self_id();
    let (p_ballot_num_complete_cycle, p_ballot_num) = flow.cycle(&proposers);
    let (p_is_leader_complete_cycle, p_is_leader) = flow.cycle(&proposers);

    /* Channels */
    let (p_to_proposers_i_am_leader_complete_cycle, p_to_proposers_i_am_leader) = flow.cycle(&proposers);
    let (a_to_proposers_p1b_complete_cycle, a_to_proposers_p1b) = flow.cycle(&proposers);
    // a_to_proposers_p1b.clone().for_each(q!(|(_, p1b): (u32, P1b)| println!("Proposer received P1b: {:?}", p1b)));
    let (a_to_proposers_p2b_complete_cycle, a_to_proposers_p2b) = flow.cycle(&proposers);
    // a_to_proposers_p2b.clone().for_each(q!(|(_, p2b): (u32, P2b)| println!("Proposer received P2b: {:?}", p2b)));
    // p_to_proposers_i_am_leader.clone().for_each(q!(|ballot: Ballot| println!("Proposer received I am leader: {:?}", ballot)));
    // c_to_proposers.clone().for_each(q!(|payload: ClientPayload| println!("Client sent proposer payload: {:?}", payload)));

    /* Stable leader election */
    let p_received_p1b_ballots = a_to_proposers_p1b
        .clone()
        .map(q!(|(_, p1b): (_, P1b)| p1b.max_ballot));
    let p_received_p2b_ballots = a_to_proposers_p2b
        .clone()
        .map(q!(|(_, p2b): (_, P2b)| p2b.max_ballot));
    let p_received_max_ballot = p_received_p1b_ballots
        .union(p_received_p2b_ballots)
        .union(p_to_proposers_i_am_leader.clone())
        .all_ticks()
        .fold(q!(|| Ballot { num: 0, id: 0 }), q!(|curr_max_ballot: &mut Ballot, new_ballot: Ballot| {
            if new_ballot > *curr_max_ballot {
                *curr_max_ballot = new_ballot;
            }
        }));
    let p_has_largest_ballot = p_received_max_ballot
        .clone()
        .zip_with_singleton(p_ballot_num.clone())
        .filter(q!(move |(received_max_ballot, ballot_num): &(Ballot, u32)| *received_max_ballot <= Ballot { num: *ballot_num, id: p_id }));

    let p_to_proposers_i_am_leader_new = p_ballot_num
        .clone()
        .sample_every(q!(*i_am_leader_send_timeout))
        .continue_if(p_is_leader.clone())
        .map(q!(move |ballot_num: u32| Ballot { num: ballot_num, id: p_id }))
        .broadcast_bincode_interleaved(&proposers);
    p_to_proposers_i_am_leader_complete_cycle.complete(p_to_proposers_i_am_leader_new);
    let p_latest_received_i_am_leader = p_to_proposers_i_am_leader
        .clone()
        .all_ticks()
        .fold(q!(|| SystemTime::UNIX_EPOCH), q!(|latest: &mut SystemTime, _: Ballot| { // Note: May want to check received ballot against our own?
            *latest = SystemTime::now();
        }));
    // Add random delay depending on node ID so not everyone sends p1a at the same time
    let p_leader_expired = flow.source_interval_delayed(&proposers, q!(Duration::from_secs((p_id * *i_am_leader_check_timeout_delay_multiplier as u32).into())), q!(*i_am_leader_check_timeout))
        .tick_batch()
        .zip_with_singleton(p_latest_received_i_am_leader.clone())
        // .inspect(q!(|v| println!("Proposer checking if leader expired")))
        // .continue_if(p_is_leader.clone().count().filter(q!(|c| *c == 0)).inspect(q!(|c| println!("Proposer is_leader count: {}", c))))
        .continue_unless(p_is_leader.clone())
        .filter(q!(|(_, latest_received_i_am_leader): &(Instant, SystemTime)| SystemTime::now() - *i_am_leader_check_timeout > *latest_received_i_am_leader));
    // p_leader_expired.clone().for_each(q!(|_| println!("Proposer leader expired")));

    let p_to_acceptors_p1a = p_ballot_num
        .clone()
        .continue_if(p_leader_expired.clone())
        .map(q!(move |ballot_num: u32| P1a { ballot: Ballot { num: ballot_num, id: p_id } }))
        .broadcast_bincode_interleaved(&acceptors);

    let p_new_ballot_num = p_received_max_ballot
        .clone()
        .zip_with_singleton(p_ballot_num.clone())
        .map(q!(move |(received_max_ballot, ballot_num): (Ballot, u32)| {
            if received_max_ballot > (Ballot { num: ballot_num, id: p_id }) {
                received_max_ballot.num + 1
            }
            else {
                ballot_num
            }
        }))
        .defer_tick();
    let p_start_ballot_num = flow
        .source_iter(&proposers, q!([0]));
    p_ballot_num_complete_cycle.complete(p_start_ballot_num.union(p_new_ballot_num));
    /* End stable leader election */

    /* Reconcile p1b log with local log */
    let p_relevant_p1bs = a_to_proposers_p1b
        .clone()
        .all_ticks()
        .zip_with_singleton(p_ballot_num.clone())
        .filter(q!(move |((sender, p1b), ballot_num): &((u32, P1b), u32)| p1b.ballot == Ballot { num: *ballot_num, id: p_id}));
    let p_received_quorum_of_p1bs = p_relevant_p1bs
        .clone()
        .map(q!(|((sender, p1b), ballot_num): ((u32, P1b), u32)| sender))
        .unique()
        .count()
        .filter_map(q!(|num_received: usize| if num_received > *f {
            Some(true)
        } 
        else {
            None
        }));
    let p_is_leader_new = p_received_quorum_of_p1bs
        .continue_if(p_has_largest_ballot.clone());
    p_is_leader_complete_cycle.complete(p_is_leader_new);

    let p_p1b_highest_entries_and_count = p_relevant_p1bs
        .clone()
        .flat_map(q!(|((_, p1b), _): ((u32, P1b), u32)| p1b.accepted.into_iter())) // Convert HashMap log back to stream
        .fold_keyed(q!(|| (0, LogValue { ballot: Ballot { num: 0, id: 0 }, value: ClientPayload { key: 0, value: 123456 } })), q!(|curr_entry: &mut (u32, LogValue), new_entry: LogValue| {
            let same_values = new_entry.value == curr_entry.1.value;
            let higher_ballot = new_entry.ballot > curr_entry.1.ballot;
            // Increment count if the values are the same
            if same_values {
                curr_entry.0 += 1;
            }
            // Replace the ballot with the largest one
            if higher_ballot {
                curr_entry.1.ballot = new_entry.ballot;
                // Replace the value with the one from the largest ballot, if necessary
                if !same_values {
                    curr_entry.0 = 1;
                    curr_entry.1.value = new_entry.value;
                }
            }
        }));
    let p_log_to_try_commit = p_p1b_highest_entries_and_count
        .clone()
        .zip_with_singleton(p_ballot_num.clone())
        .filter_map(q!(move |((slot, (count, entry)), ballot_num): ((i32, (u32, LogValue)), u32)|
            if count <= *f as u32 { 
                Some(P2a { ballot: Ballot { num: ballot_num, id: p_id }, slot: slot, value: entry.value})
            } 
            else { 
                None 
            }));
    let p_max_slot = p_p1b_highest_entries_and_count
        .clone()
        .fold(q!(|| -1), q!(|max_slot: &mut i32, (slot, (count, entry)): (i32, (u32, LogValue))| {
            if slot > *max_slot {
                *max_slot = slot;
            }
        }));
    let p_proposed_slots = p_p1b_highest_entries_and_count
        .clone()
        .map(q!(|(slot, _): (i32, (u32, LogValue))| slot));
    let p_log_holes = p_max_slot
        .clone()
        .flat_map(q!(|max_slot: i32| 0..max_slot))
        .filter_not_in(p_proposed_slots)
        .zip_with_singleton(p_ballot_num.clone())
        .map(q!(move |(slot, ballot_num): (i32, u32)| P2a { ballot: Ballot { num: ballot_num, id: p_id }, slot: slot, value: ClientPayload { key: 0, value: 0 } }));
        
        let context = flow.runtime_context();
    let (p_next_slot_complete_cycle, p_next_slot) = flow.cycle(&proposers);
    let p_next_slot_after_reconciling_p1bs = p_max_slot
        // .inspect(q!(|max_slot| println!("{} p_max_slot: {:?}", context.current_tick(), max_slot)))
        .clone()
        .continue_unless(p_next_slot.clone())
        .map(q!(|max_slot: i32| max_slot + 1));
    /* End reconcile p1b log with local log */

    /* Send p2as */
    let p_indexed_payloads = c_to_proposers
        .clone()
        .tick_batch()
        .enumerate()
        .zip_with_singleton(p_next_slot.clone())
        // .inspect(q!(|next| println!("{} p_indexed_payloads next slot: {}", context.current_tick(), next))))
        .zip_with_singleton(p_ballot_num.clone())
        // .inspect(q!(|ballot_num| println!("{} p_indexed_payloads ballot_num: {}", context.current_tick(), ballot_num))))
        .map(q!(move |(((index, payload), next_slot), ballot_num): (((usize, ClientPayload), i32), u32)| P2a { ballot: Ballot { num: ballot_num, id: p_id }, slot: next_slot + index as i32, value: payload }));
        // .inspect(q!(|p2a: &P2a| println!("{} p_indexed_payloads P2a: {:?}", context.current_tick(), p2a)));
    let p_to_acceptors_p2a = p_log_to_try_commit
        .union(p_log_holes)
        .continue_unless(p_next_slot.clone()) // Only resend p1b stuff once. Once it's resent, next_slot will exist.
        .union(p_indexed_payloads)
        .continue_if(p_is_leader.clone())
        .broadcast_bincode_interleaved(&acceptors);

    let p_num_payloads = c_to_proposers
        .clone()
        .tick_batch()
        .count();
    let p_exists_payloads = p_num_payloads
        .clone()
        .filter(q!(|num_payloads: &usize| *num_payloads > 0));
    let p_next_slot_after_sending_payloads = p_num_payloads
        .continue_if(p_exists_payloads.clone())
        .clone()
        .zip_with_singleton(p_next_slot.clone())
        .map(q!(|(num_payloads, next_slot): (usize, i32)| next_slot + num_payloads as i32));
    let p_next_slot_if_no_payloads = p_next_slot
        .clone()
        .continue_unless(p_exists_payloads);
    let p_new_next_slot_calculated = p_next_slot_after_reconciling_p1bs
        // .inspect(q!(|slot| println!("{} p_new_next_slot_after_reconciling_p1bs: {:?}", context.current_tick(), slot)))
        .union(p_next_slot_after_sending_payloads)
        // .inspect(q!(|slot| println!("{} p_next_slot_after_sending_payloads: {:?}", context.current_tick(), slot))))
        .union(p_next_slot_if_no_payloads)
        // .inspect(q!(|slot| println!("{} p_next_slot_if_no_payloads: {:?}", context.current_tick(), slot))))
        .continue_if(p_is_leader.clone());
    let p_new_next_slot_default = p_is_leader // Default next slot to 0 if there haven't been any payloads at all
        .clone()
        .continue_unless(p_new_next_slot_calculated.clone())
        .map(q!(|_: bool| 0));
        // .inspect(q!(|slot| println!("{} p_new_next_slot_default: {:?}", context.current_tick(), slot)));
    let p_new_next_slot = p_new_next_slot_calculated
        .union(p_new_next_slot_default)
        .defer_tick();
    p_next_slot_complete_cycle.complete(p_new_next_slot);
    /* End send p2as */

    /* Tell clients that leader election has completed and they can begin sending messages */
    let p_to_clients_new_leader_elected = p_is_leader.clone()
        .continue_unless(p_next_slot.clone())
        .zip_with_singleton(p_ballot_num.clone())
        .map(q!(|(is_leader, ballot_num): (bool, u32)| ballot_num)) // Only tell the clients once when leader election concludes
        .broadcast_bincode(&clients);
    p_to_clients_leader_elected_cycle.complete(p_to_clients_new_leader_elected);
    /* End tell clients that leader election has completed */
    
    /* Process p2bs */
    let (p_broadcasted_p2b_slots_complete_cycle, p_broadcasted_p2b_slots) = flow.cycle(&proposers);
    let (p_persisted_p2bs_complete_cycle, p_persisted_p2bs) = flow.cycle(&proposers);
    let p_p2b = a_to_proposers_p2b
        .clone()
        .tick_batch()
        .union(p_persisted_p2bs);
    let p_count_matching_p2bs = p_p2b
        .clone()
        .filter_map(q!(|(sender, p2b): (u32, P2b)| 
            if p2b.ballot == p2b.max_ballot { // Only consider p2bs where max ballot = ballot, which means that no one preempted us
                Some((p2b.slot, (sender, p2b)))
            }
            else {
                None
            }
        ))
        .fold_keyed(q!(|| (0, P2b { ballot: Ballot { num: 0, id: 0 }, max_ballot: Ballot { num: 0, id: 0 }, slot: 0, value: ClientPayload { key: 0, value: 0 } } )),
            q!(|accum: &mut (usize, P2b), (sender, p2b): (u32, P2b)| {
            accum.0 += 1;
            accum.1 = p2b;
        }));
    let p_p2b_quorum_reached = p_count_matching_p2bs
        .clone()
        .filter(q!(|(slot, (count, p2b)): &(i32, (usize, P2b))| *count > *f));
    let p_to_replicas = p_p2b_quorum_reached
        .clone()
        .anti_join(p_broadcasted_p2b_slots) // Only tell the replicas about committed values once
        .map(q!(|(slot, (count, p2b)): (i32, (usize, P2b))| ReplicaPayload { seq: p2b.slot, key: p2b.value.key, value: p2b.value.value }))
        .broadcast_bincode_interleaved(&replicas);

    let p_p2b_all_commit_slots = p_count_matching_p2bs
        .clone()
        .filter_map(q!(|(slot, (count, p2b)): (i32, (usize, P2b))| 
            if count == 2 * *f + 1 {
                Some(slot)
            }
            else {
                None
            }
        ));
    // p_p2b_all_commit_slots.clone().for_each(q!(|slot: i32| println!("Proposer slot all received: {:?}", slot)));
    let p_broadcasted_p2b_slots_new = p_p2b_quorum_reached
        .clone()    
        .map(q!(|(slot, (count, p2b)): (i32, (usize, P2b))| slot))
        .filter_not_in(p_p2b_all_commit_slots.clone())
        .defer_tick();
    // p_broadcasted_p2b_slots_new.clone().for_each(q!(|slot: i32| println!("Proposer slot broadcasted: {:?}", slot)));
    p_broadcasted_p2b_slots_complete_cycle.complete(p_broadcasted_p2b_slots_new);
    let p_persisted_p2bs_new = p_p2b
        .clone()
        .map(q!(|(sender, p2b): (u32, P2b)| (p2b.slot, (sender, p2b))))
        .anti_join(p_p2b_all_commit_slots.clone())
        .map(q!(|(slot, (sender, p2b)): (i32, (u32, P2b))| (sender, p2b)))
        .defer_tick();
    // p_persisted_p2bs_new.clone().for_each(q!(|(sender, p2b): (u32, P2b)| println!("Proposer persisting p2b: {:?}", p2b)));
    p_persisted_p2bs_complete_cycle.complete(p_persisted_p2bs_new);
    /* End process p2bs */
    

    /* Acceptors. All relations for acceptors will be prefixed with a. */


    flow.source_iter(&acceptors, q!(["Acceptors say hello"]))
        .for_each(q!(|s| println!("{}", s)));
    // p_to_acceptors_p1a.clone().for_each(q!(|p1a: P1a| println!("Acceptor received P1a: {:?}", p1a)));
    // p_to_acceptors_p2a.clone().for_each(q!(|p2a: P2a| println!("Acceptor received P2a: {:?}", p2a)));
    let a_max_ballot = p_to_acceptors_p1a
        .clone()
        .all_ticks()
        .fold(q!(|| Ballot { num: 0, id: 0 }), q!(|max_ballot: &mut Ballot, p1a: P1a| {
            if p1a.ballot > *max_ballot {
                *max_ballot = p1a.ballot;
            }
        }));
    let a_log = p_to_acceptors_p2a
        .clone()
        .tick_batch()
        .zip_with_singleton(a_max_ballot.clone())
        .filter(q!(|(p2a, max_ballot): &(P2a, Ballot)| p2a.ballot >= *max_ballot)) // Don't consider p2as if the current ballot is higher
        .map(q!(|(p2a, _): (P2a, Ballot)| (p2a.slot, p2a))) // Group by slot
        .all_ticks()
        .reduce_keyed(q!(|curr_entry: &mut P2a, new_entry: P2a| {
            if new_entry.ballot > curr_entry.ballot { // Update the log
                *curr_entry = new_entry;
            }
        }))
        .continue_if(p_to_acceptors_p1a.clone().tick_batch()) // TODO: Hack to avoid creating HashMap until we receive p1a
        // TODO: Would be nice if there was a "collect" operator here. Will need later to partition the log anyway
        .fold(q!(|| HashMap::<i32, LogValue>::new()), q!(|log: &mut HashMap::<i32, LogValue>, (slot, p2a): (i32, P2a)| {
            log.insert(slot, LogValue { ballot: p2a.ballot, value: p2a.value });
        }));
        
    let a_to_proposers_p1b_new = p_to_acceptors_p1a
        .clone()
        .tick_batch()
        .zip_with_singleton(a_max_ballot.clone())
        .zip_with_singleton(a_log)
        .map(q!(|((p1a, max_ballot), log): ((P1a, Ballot), HashMap::<i32, LogValue>)| (p1a.ballot.id, P1b { ballot: p1a.ballot, max_ballot: max_ballot, accepted: log })))
        .send_bincode(&proposers);
    a_to_proposers_p1b_complete_cycle.complete(a_to_proposers_p1b_new);

    let a_to_proposers_p2b_new: Stream<(u32, P2b), stream::Async, <D as Deploy<'a>>::Cluster> = p_to_acceptors_p2a
        .tick_batch()
        .zip_with_singleton(a_max_ballot.clone())
        .map(q!(|(p2a, max_ballot): (P2a, Ballot)| (p2a.ballot.id, P2b { ballot: p2a.ballot, max_ballot: max_ballot, slot: p2a.slot, value: p2a.value })))
        .send_bincode(&proposers);
    a_to_proposers_p2b_complete_cycle.complete(a_to_proposers_p2b_new);


    /* Replicas. All relations for replicas will be prefixed with r. */


    let (r_buffered_payloads_complete_cycle, r_buffered_payloads) = flow.cycle(&replicas);
    // p_to_replicas.clone().for_each(q!(|payload: ReplicaPayload| println!("Replica received payload: {:?}", payload)));
    let r_sorted_payloads = p_to_replicas
        .clone()
        .tick_batch()
        .union(r_buffered_payloads) // Combine with all payloads that we've received and not processed yet
        .sort();
    // Create a cycle since we'll use this seq before we define it
    let (r_highest_seq_complete_cycle, r_highest_seq) = flow.cycle(&replicas);
    let empty_slot = flow.source_iter(&replicas, q!([-1]));
    // Either the max sequence number executed so far or -1. Need to union otherwise r_highest_seq is empty and joins with it will fail
    let r_highest_seq_with_default = r_highest_seq
        .union(empty_slot);
    // Find highest the sequence number of any payload that can be processed in this tick. This is the payload right before a hole.
    let r_highest_seq_processable_payload = r_sorted_payloads
        .clone()
        .zip_with_singleton(r_highest_seq_with_default)
        .fold(q!(|| -1), q!(| filled_slot: &mut i32, (sorted_payload, highest_seq): (ReplicaPayload, i32)| { // Note: This function only works if the input is sorted on seq.
            let next_slot = std::cmp::max(*filled_slot, highest_seq);

            *filled_slot = if sorted_payload.seq == next_slot + 1 {
                sorted_payload.seq
            } else {
                *filled_slot
            };
        }));
    // Find all payloads that can and cannot be processed in this tick.
    let r_processable_payloads = r_sorted_payloads
        .clone()
        .zip_with_singleton(r_highest_seq_processable_payload.clone())
        .filter(q!(|(sorted_payload, highest_seq): &(ReplicaPayload, i32)| sorted_payload.seq <= *highest_seq))
        .map(q!(|(sorted_payload, _): (ReplicaPayload, i32)| sorted_payload));
    let r_new_non_processable_payloads = r_sorted_payloads
        .clone()
        .zip_with_singleton(r_highest_seq_processable_payload.clone())
        .filter(q!(|(sorted_payload, highest_seq): &(ReplicaPayload, i32)| sorted_payload.seq > *highest_seq))
        .map(q!(|(sorted_payload, _): (ReplicaPayload, i32)| sorted_payload))
        .defer_tick(); // Save these, we can process them once the hole has been filled
    r_buffered_payloads_complete_cycle.complete(r_new_non_processable_payloads);

    let r_kv_store = r_processable_payloads
        .clone()
        .all_ticks() // Optimization: all_ticks() + fold() = fold<static>, where the state of the previous fold is saved and persisted values are deleted.
        .fold(q!(|| (HashMap::<u32, u32>::new(), -1)), q!(|state: &mut (HashMap::<u32, u32>, i32), payload: ReplicaPayload| {
            let ref mut kv_store = state.0;
            let ref mut last_seq = state.1;
            kv_store.insert(payload.key, payload.value);
            debug_assert!(payload.seq == *last_seq + 1, "Hole in log between seq {} and {}", *last_seq, payload.seq);
            *last_seq = payload.seq;
            // println!("Replica kv store: {:?}", kv_store);
        }));
    // Update the highest seq for the next tick
    let r_new_highest_seq = r_kv_store
        .map(q!(|(kv_store, highest_seq): (HashMap::<u32, u32>, i32)| highest_seq))
        .defer_tick();
    r_highest_seq_complete_cycle.complete(r_new_highest_seq);

    // Tell clients that the payload has been committed. All ReplicaPayloads contain the client's machine ID (to string) as value.
    let r_new_processed_payloads = p_to_replicas
        .tick_batch()
        .map(q!(|payload: ReplicaPayload| (payload.value, payload)))
        .send_bincode_interleaved(&clients);
    r_to_clients_payload_applied_cycle.complete(r_new_processed_payloads);

    (proposers, acceptors, clients, replicas)
}

#[stageleft::entry]
pub fn paxos_runtime<'a>(
    flow: FlowBuilder<'a, CLIRuntime>,
    cli: RuntimeData<&'a HydroCLI<HydroflowPlusMeta>>,
    f: RuntimeData<&'a usize>,
    num_clients_per_node: RuntimeData<&'a usize>,
    kv_num_keys: RuntimeData<&'a usize>,
    kv_value_size: RuntimeData<&'a usize>,
    median_latency_window_size: RuntimeData<&'a usize>,
    i_am_leader_send_timeout: RuntimeData<&'a Duration>,
    i_am_leader_check_timeout: RuntimeData<&'a Duration>,
    i_am_leader_check_timeout_delay_multiplier: RuntimeData<&'a usize>,
) -> impl Quoted<'a, Hydroflow<'a>> {
    let _ = paxos(&flow, &cli, &cli, &cli, &cli, f, num_clients_per_node, kv_num_keys, kv_value_size, median_latency_window_size, i_am_leader_send_timeout, i_am_leader_check_timeout, i_am_leader_check_timeout_delay_multiplier);
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

    // cargo test -p hydroflow_plus_test paxos -- --nocapture
    #[tokio::test]
    async fn paxos() {
        let deployment = RefCell::new(Deployment::new());
        let localhost = deployment.borrow_mut().Localhost();

        let builder = hydroflow_plus::FlowBuilder::new();
        let f = 1;
        let num_clients = 1;
        let num_replicas = 1;
        let profile = "profile";
        let (proposers, acceptors, clients, replicas) = super::paxos(
            &builder,
            &DeployClusterSpec::new(|| {
                (0..(f+1))
                    .map(|i| {
                        deployment.borrow_mut().add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("paxos")
                                .profile(profile)
                                .perf(format!("proposer{}.perf.data", i))
                                .display_name(format!("Proposer{}", i)),
                        )
                    })
                    .collect()
            }),
            &DeployClusterSpec::new(|| {
                (0..(2*f+1))
                    .map(|i| {
                        deployment.borrow_mut().add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("paxos")
                                .profile(profile)
                                .perf(format!("acceptor{}.perf.data", i))
                                .display_name(format!("Acceptor{}", i)),
                        )
                    })
                    .collect()
            }),
            &DeployClusterSpec::new(|| {
                (0..num_clients)
                    .map(|i| {
                        deployment.borrow_mut().add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("paxos")
                                .profile(profile)
                                .perf(format!("client{}.perf.data", i))
                                .display_name(format!("Client{}", i)),
                        )
                    })
                    .collect()
            }),
            &DeployClusterSpec::new(|| {
                (0..num_replicas)
                    .map(|i| {
                        deployment.borrow_mut().add_service(
                            HydroflowCrate::new(".", localhost.clone())
                                .bin("paxos")
                                .profile(profile)
                                .perf(format!("replica{}.perf.data", i))
                                .display_name(format!("Replica{}", i)),
                        )
                    })
                    .collect()
            }),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
            RuntimeData::new("Fake"),
        );

        // insta::assert_debug_snapshot!(builder.extract().ir());

        let mut deployment = deployment.into_inner();

        deployment.deploy().await.unwrap();

        deployment.start().await.unwrap();

        tokio::signal::ctrl_c().await.unwrap();

        // Expected output:
        /*
        [service/6] Replica kv store: {10: "Hello, Berkeley!"}
        [service/6] Replica kv store: {10: "Hello again, Berkeley!"}
        [service/6] Replica kv store: {10: "Goodbye, Berkeley"}
        [service/6] Replica kv store: {20: "Hello, SF", 10: "Goodbye, Berkeley"}
        [service/6] Replica kv store: {20: "Hello, SF", 10: "No more Berkeley"}
        [service/6] Replica kv store: {20: "Goodbye, SF", 10: "No more Berkeley"}
         */
    }
}