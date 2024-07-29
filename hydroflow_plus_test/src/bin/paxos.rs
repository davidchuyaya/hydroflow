// cannot use hydroflow::main because connect_local_blocking causes a deadlock
use std::time::Duration;

#[tokio::main]
async fn main() {
    let f = 1;
    let num_clients_per_node = 1; // TODO: Need to change based on experiment between 1, 50, 100.
    let kv_num_keys = 1;
    let kv_value_size = 16;
    let i_am_leader_send_timeout = Duration::from_secs(5);
    let i_am_leader_check_timeout = Duration::from_secs(10);
    
    // hydroflow_plus::launch!(|ports| hydroflow_plus_test::cluster::paxos::paxos_runtime!(
    //     ports,
    //     &f,
    //     &num_clients_per_node,
    //     &kv_num_keys,
    //     &kv_value_size,
    //     &i_am_leader_send_timeout,
    //     &i_am_leader_check_timeout,
    // ))
    // .await;
    // Recursive expansion of launch! macro
// =====================================

async {
    let ports = hydroflow_plus::util::cli::init_no_ack_start().await;
    let flow = (|ports|{
        use hydroflow_plus_test::__staged::cluster::paxos:: * ;
        fn expand_staged<'a>(cli: &'a HydroCLI<HydroflowPlusMeta> ,f: &'a usize,num_clients_per_node: &'a usize,kv_num_keys: &'a usize,kv_value_size: &'a usize,i_am_leader_send_timeout: &'a Duration,i_am_leader_check_timeout: &'a Duration) -> Hydroflow<'a>{
            {
                let __given_id = {
                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                    let cli = cli;
                    cli.meta.subgraph_id
                };
                if __given_id==0usize {
                    {
                        #[allow(unused_qualifications)]
                        {
                            use hydroflow_plus::{
                                var_expr,var_args
                            };
                            let mut df = hydroflow_plus::scheduled::graph::Hydroflow::new();
                            df.__assign_meta_graph("{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_iter ({use crate :: __staged :: cluster :: paxos :: * ; [\\\"Proposers say hello\\\"]})\"},\"version\":1},{\"value\":{\"Operator\":\"for_each ({use crate :: __staged :: cluster :: paxos :: * ; | s | println ! (\\\"{}\\\" , s)})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"for_each ({use crate :: __staged :: cluster :: paxos :: * ; | (_ , p1b) : (u32 , P1b) | println ! (\\\"Proposer received P1b: {:?}\\\" , p1b)})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"for_each ({use crate :: __staged :: cluster :: paxos :: * ; | (_ , p2b) : (u32 , P2b) | println ! (\\\"Proposer received P2b: {:?}\\\" , p2b)})\"},\"version\":1},{\"value\":{\"Operator\":\"source_iter ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; let self_id = 0usize ; cli . meta . clusters . get (& self_id) . unwrap ()})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | d | d . clone ()})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"source_interval ({use crate :: __staged :: cluster :: paxos :: * ; let i_am_leader_send_timeout = i_am_leader_send_timeout ; * i_am_leader_send_timeout})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (a , _) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (a , _) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; move | ballot_num | Ballot {num : ballot_num , id : p_id}})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'static , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map (| (id , data) | {(id , hydroflow_plus :: runtime_support :: bincode :: serialize :: < crate :: __staged :: cluster :: paxos :: Ballot > (& data) . unwrap () . into ())})\"},\"version\":1},{\"value\":{\"Operator\":\"dest_sink ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_1\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedDemux < ConnectedDirect > > () . into_sink ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"source_stream ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_2\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedTagged < ConnectedDirect > > () . into_source ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| res | {let (id , b) = res . unwrap () ; (id , hydroflow_plus :: runtime_support :: bincode :: deserialize :: < crate :: __staged :: cluster :: paxos :: Ballot > (& b) . unwrap ())})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (_ , b) | b})\"},\"version\":1},{\"value\":{\"Operator\":\"source_iter ({use crate :: __staged :: cluster :: paxos :: * ; [0]})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (_ , p1b) : (_ , P1b) | p1b . max_ballot})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (_ , p2b) : (_ , P2b) | p2b . max_ballot})\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'static > ({use crate :: __staged :: cluster :: paxos :: * ; | | Ballot {num : 0 , id : 0}} , {use crate :: __staged :: cluster :: paxos :: * ; | curr_max_ballot , new_ballot | {if new_ballot > * curr_max_ballot {* curr_max_ballot = new_ballot ;}}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; move | (received_max_ballot , ballot_num) | {if received_max_ballot > (Ballot {num : ballot_num , id : p_id}) {received_max_ballot . num + 1} else {ballot_num}}})\"},\"version\":1},{\"value\":{\"Operator\":\"defer_tick ()\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'static , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"filter ({use crate :: __staged :: cluster :: paxos :: * ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; move | ((sender , p1b) , ballot_num) | p1b . ballot == Ballot {num : * ballot_num , id : p_id}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | ((sender , p1b) , ballot_num) | sender})\"},\"version\":1},{\"value\":{\"Operator\":\"unique :: < 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'tick > ({use hydroflow_plus :: __staged :: stream :: * ; | | 0usize} , {use hydroflow_plus :: __staged :: stream :: * ; | count , _ | * count += 1})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; let f = f ; | num_received | num_received > * f})\"},\"version\":1},{\"value\":{\"Operator\":\"filter ({use hydroflow_plus :: __staged :: stream :: * ; | b | * b})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; move | (received_max_ballot , ballot_num) | received_max_ballot <= Ballot {num : ballot_num , id : p_id}})\"},\"version\":1},{\"value\":{\"Operator\":\"filter ({use hydroflow_plus :: __staged :: stream :: * ; | b | * b})\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":3},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (a , _) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"flat_map ({use crate :: __staged :: cluster :: paxos :: * ; | ((_ , p1b) , _) | p1b . accepted . into_iter ()})\"},\"version\":1},{\"value\":{\"Operator\":\"fold_keyed :: < 'tick > ({use crate :: __staged :: cluster :: paxos :: * ; | | (0 , LogValue {ballot : Ballot {num : 0 , id : 0} , value : ClientPayload {key : 0 , value : \\\"\\\" . to_string ()}})} , {use crate :: __staged :: cluster :: paxos :: * ; | curr_entry , new_entry | {let same_values = new_entry . value == curr_entry . 1 . value ; let higher_ballot = new_entry . ballot > curr_entry . 1 . ballot ; if same_values {curr_entry . 0 += 1 ;} if higher_ballot {curr_entry . 1 . ballot = new_entry . ballot ; if ! same_values {curr_entry . 0 = 1 ; curr_entry . 1 . value = new_entry . value ;}}}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'tick > ({use crate :: __staged :: cluster :: paxos :: * ; | | 0} , {use crate :: __staged :: cluster :: paxos :: * ; | max_slot , (slot , (count , entry)) | {if slot > * max_slot {* max_slot = slot ;}}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | max_slot | max_slot + 1})\"},\"version\":1},{\"value\":{\"Operator\":\"source_stream ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_0\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedTagged < ConnectedDirect > > () . into_source ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| res | {let (id , b) = res . unwrap () ; (id , hydroflow_plus :: runtime_support :: bincode :: deserialize :: < crate :: __staged :: cluster :: paxos :: ClientPayload > (& b) . unwrap ())})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (_ , b) | b})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'tick > ({use hydroflow_plus :: __staged :: stream :: * ; | | 0usize} , {use hydroflow_plus :: __staged :: stream :: * ; | count , _ | * count += 1})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (num_payloads , next_slot) : (usize , i32) | next_slot + num_payloads as i32})\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | a | (() , a)})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | num_payloads | num_payloads > 0})\"},\"version\":1},{\"value\":{\"Operator\":\"filter ({use hydroflow_plus :: __staged :: stream :: * ; | b | * b})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | _ | ()})\"},\"version\":1},{\"value\":{\"Operator\":\"anti_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (_ , a) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (a , _) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"defer_tick ()\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"filter_map ({use crate :: __staged :: cluster :: paxos :: * ; | (sender , p2b) | if p2b . ballot == p2b . max_ballot {Some ((p2b . slot , (sender , p2b)))} else {None}})\"},\"version\":1},{\"value\":{\"Operator\":\"fold_keyed :: < 'tick > ({use crate :: __staged :: cluster :: paxos :: * ; | | (0 , P2b {ballot : Ballot {num : 0 , id : 0} , max_ballot : Ballot {num : 0 , id : 0} , slot : 0 , value : ClientPayload {key : 0 , value : \\\"\\\" . to_string ()}})} , {use crate :: __staged :: cluster :: paxos :: * ; | accum , (sender , p2b) | {accum . 0 += 1 ; accum . 1 = p2b ;}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"anti_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"filter_map ({use crate :: __staged :: cluster :: paxos :: * ; let f = f ; | (slot , (count , p2b)) | if count > * f {Some (p2b)} else {None}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | p2b | p2b . slot})\"},\"version\":1},{\"value\":{\"Operator\":\"filter_map ({use crate :: __staged :: cluster :: paxos :: * ; let f = f ; | (slot , (count , p2b)) | if count == 2 * * f + 1 {Some (slot)} else {None}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"difference_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"defer_tick ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (sender , p2b) | (p2b . slot , (sender , p2b))})\"},\"version\":1},{\"value\":{\"Operator\":\"anti_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (slot , (sender , p2b)) | (sender , p2b)})\"},\"version\":1},{\"value\":{\"Operator\":\"defer_tick ()\"},\"version\":1},{\"value\":{\"Operator\":\"source_iter ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; let self_id = 1usize ; cli . meta . clusters . get (& self_id) . unwrap ()})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | d | d . clone ()})\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'static > ({use crate :: __staged :: cluster :: paxos :: * ; | | SystemTime :: now ()} , {use crate :: __staged :: cluster :: paxos :: * ; | latest , _ | {* latest = SystemTime :: now () ;}})\"},\"version\":1},{\"value\":{\"Operator\":\"source_interval ({use crate :: __staged :: cluster :: paxos :: * ; let i_am_leader_check_timeout = i_am_leader_check_timeout ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; * i_am_leader_check_timeout + Duration :: from_secs (p_id . into ())})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (a , _) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | a | (() , a)})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | _ | ()})\"},\"version\":1},{\"value\":{\"Operator\":\"anti_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (_ , a) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; let i_am_leader_check_timeout = i_am_leader_check_timeout ; | latest_received_i_am_leader | SystemTime :: now () - * i_am_leader_check_timeout > latest_received_i_am_leader})\"},\"version\":1},{\"value\":{\"Operator\":\"filter ({use hydroflow_plus :: __staged :: stream :: * ; | b | * b})\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":3},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (a , _) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; move | ballot_num | P1a {ballot : Ballot {num : ballot_num , id : p_id}}})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'static , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map (| (id , data) | {(id , hydroflow_plus :: runtime_support :: bincode :: serialize :: < crate :: __staged :: cluster :: paxos :: P1a > (& data) . unwrap () . into ())})\"},\"version\":1},{\"value\":{\"Operator\":\"dest_sink ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_3\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedDemux < ConnectedDirect > > () . into_sink ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"source_iter ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; let self_id = 1usize ; cli . meta . clusters . get (& self_id) . unwrap ()})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | d | d . clone ()})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"filter_map ({use crate :: __staged :: cluster :: paxos :: * ; let f = f ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; move | ((slot , (count , entry)) , ballot_num) | if count <= * f {Some (P2a {ballot : Ballot {num : ballot_num , id : p_id} , slot : slot , value : entry . value})} else {None}})\"},\"version\":1},{\"value\":{\"Operator\":\"flat_map ({use crate :: __staged :: cluster :: paxos :: * ; | max_slot | 0 .. max_slot})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (slot , _) | slot})\"},\"version\":1},{\"value\":{\"Operator\":\"difference_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; move | (slot , ballot_num) | P2a {ballot : Ballot {num : ballot_num , id : p_id} , slot : slot , value : ClientPayload {key : 0 , value : \\\"\\\" . to_string ()}}})\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | a | (() , a)})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | _ : i32 | true})\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":3},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | _ | ()})\"},\"version\":1},{\"value\":{\"Operator\":\"anti_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (_ , a) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"enumerate ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; let p_id = {use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; cli . meta . cluster_id . expect (\\\"Tried to read Cluster ID on a non-cluster node\\\")} ; move | (((index , payload) , next_slot) , ballot_num) : (((usize , ClientPayload) , i32) , u32) | P2a {ballot : Ballot {num : ballot_num , id : p_id} , slot : next_slot + index as i32 , value : payload}})\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (a , _) | a})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'static , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map (| (id , data) | {(id , hydroflow_plus :: runtime_support :: bincode :: serialize :: < crate :: __staged :: cluster :: paxos :: P2a > (& data) . unwrap () . into ())})\"},\"version\":1},{\"value\":{\"Operator\":\"dest_sink ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_4\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedDemux < ConnectedDirect > > () . into_sink ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"source_stream ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_6\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedTagged < ConnectedDirect > > () . into_source ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| res | {let (id , b) = res . unwrap () ; (id , hydroflow_plus :: runtime_support :: bincode :: deserialize :: < crate :: __staged :: cluster :: paxos :: P1b > (& b) . unwrap ())})\"},\"version\":1},{\"value\":{\"Operator\":\"source_stream ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_7\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedTagged < ConnectedDirect > > () . into_source ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| res | {let (id , b) = res . unwrap () ; (id , hydroflow_plus :: runtime_support :: bincode :: deserialize :: < crate :: __staged :: cluster :: paxos :: P2b > (& b) . unwrap ())})\"},\"version\":1},{\"value\":{\"Operator\":\"source_iter ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; let self_id = 3usize ; cli . meta . clusters . get (& self_id) . unwrap ()})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | d | d . clone ()})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | p2b | ReplicaPayload {seq : p2b . slot , key : p2b . value . key , value : p2b . value . value}})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'static , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map (| (id , data) | {(id , hydroflow_plus :: runtime_support :: bincode :: serialize :: < crate :: __staged :: cluster :: paxos :: ReplicaPayload > (& data) . unwrap () . into ())})\"},\"version\":1},{\"value\":{\"Operator\":\"dest_sink ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_5\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedDemux < ConnectedDirect > > () . into_sink ()}})\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":138,\"version\":1},{\"idx\":3,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":140,\"version\":1},{\"idx\":5,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":7,\"version\":1},{\"idx\":8,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":34,\"version\":1},{\"idx\":9,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":123,\"version\":3}],\"version\":3},{\"value\":[{\"idx\":10,\"version\":1},{\"idx\":192,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":11,\"version\":1},{\"idx\":12,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":48,\"version\":1},{\"idx\":13,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":12,\"version\":1},{\"idx\":14,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":13,\"version\":1},{\"idx\":104,\"version\":3}],\"version\":3},{\"value\":[{\"idx\":14,\"version\":1},{\"idx\":15,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":15,\"version\":1},{\"idx\":16,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":8,\"version\":1},{\"idx\":17,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":16,\"version\":1},{\"idx\":17,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":18,\"version\":1},{\"idx\":19,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":17,\"version\":1},{\"idx\":18,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":20,\"version\":1},{\"idx\":21,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":21,\"version\":1},{\"idx\":22,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":24,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":25,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":24,\"version\":1},{\"idx\":46,\"version\":3}],\"version\":3},{\"value\":[{\"idx\":25,\"version\":1},{\"idx\":147,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":22,\"version\":1},{\"idx\":27,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":26,\"version\":1},{\"idx\":28,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":27,\"version\":1},{\"idx\":148,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":28,\"version\":1},{\"idx\":149,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":29,\"version\":1},{\"idx\":30,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":30,\"version\":1},{\"idx\":150,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":151,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":31,\"version\":1},{\"idx\":32,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":32,\"version\":1},{\"idx\":152,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":23,\"version\":1},{\"idx\":34,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":33,\"version\":1},{\"idx\":34,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":153,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":154,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":35,\"version\":1},{\"idx\":36,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":36,\"version\":1},{\"idx\":37,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":37,\"version\":1},{\"idx\":38,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":38,\"version\":1},{\"idx\":39,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":39,\"version\":1},{\"idx\":155,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":40,\"version\":1},{\"idx\":41,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":41,\"version\":1},{\"idx\":42,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":30,\"version\":1},{\"idx\":156,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":157,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":43,\"version\":1},{\"idx\":44,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":44,\"version\":1},{\"idx\":45,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":46,\"version\":3},{\"idx\":26,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":42,\"version\":1},{\"idx\":47,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":45,\"version\":1},{\"idx\":47,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":47,\"version\":1},{\"idx\":48,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":37,\"version\":1},{\"idx\":49,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":49,\"version\":1},{\"idx\":158,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":50,\"version\":1},{\"idx\":51,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":51,\"version\":1},{\"idx\":159,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":52,\"version\":1},{\"idx\":53,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":53,\"version\":1},{\"idx\":54,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":55,\"version\":1},{\"idx\":56,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":56,\"version\":1},{\"idx\":57,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":57,\"version\":1},{\"idx\":58,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":58,\"version\":1},{\"idx\":160,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":59,\"version\":1},{\"idx\":60,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":74,\"version\":1},{\"idx\":61,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":60,\"version\":1},{\"idx\":161,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":61,\"version\":1},{\"idx\":162,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":62,\"version\":1},{\"idx\":63,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":54,\"version\":1},{\"idx\":163,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":63,\"version\":1},{\"idx\":64,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":61,\"version\":1},{\"idx\":65,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":60,\"version\":1},{\"idx\":66,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":66,\"version\":1},{\"idx\":67,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":67,\"version\":1},{\"idx\":68,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":65,\"version\":1},{\"idx\":164,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":68,\"version\":1},{\"idx\":165,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":69,\"version\":1},{\"idx\":70,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":64,\"version\":1},{\"idx\":71,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":70,\"version\":1},{\"idx\":71,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":71,\"version\":1},{\"idx\":72,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":13,\"version\":1},{\"idx\":166,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":72,\"version\":1},{\"idx\":73,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":73,\"version\":1},{\"idx\":167,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":168,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":91,\"version\":1},{\"idx\":75,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":75,\"version\":1},{\"idx\":76,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":76,\"version\":1},{\"idx\":77,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":77,\"version\":1},{\"idx\":169,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":78,\"version\":1},{\"idx\":79,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":79,\"version\":1},{\"idx\":170,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":87,\"version\":1},{\"idx\":171,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":80,\"version\":1},{\"idx\":81,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":81,\"version\":1},{\"idx\":82,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":82,\"version\":1},{\"idx\":83,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":79,\"version\":1},{\"idx\":84,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":84,\"version\":1},{\"idx\":85,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":83,\"version\":1},{\"idx\":172,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":85,\"version\":1},{\"idx\":173,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":86,\"version\":1},{\"idx\":174,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":76,\"version\":1},{\"idx\":88,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":88,\"version\":1},{\"idx\":175,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":85,\"version\":1},{\"idx\":176,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":89,\"version\":1},{\"idx\":90,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":90,\"version\":1},{\"idx\":177,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":92,\"version\":1},{\"idx\":93,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":27,\"version\":1},{\"idx\":178,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":94,\"version\":1},{\"idx\":96,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":95,\"version\":1},{\"idx\":193,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":96,\"version\":1},{\"idx\":97,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":97,\"version\":1},{\"idx\":98,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":13,\"version\":1},{\"idx\":99,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":98,\"version\":1},{\"idx\":100,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":99,\"version\":1},{\"idx\":179,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":100,\"version\":1},{\"idx\":101,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":101,\"version\":1},{\"idx\":102,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":102,\"version\":1},{\"idx\":103,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":104,\"version\":3},{\"idx\":14,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":180,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":103,\"version\":1},{\"idx\":105,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":105,\"version\":1},{\"idx\":106,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":106,\"version\":1},{\"idx\":107,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":93,\"version\":1},{\"idx\":108,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":107,\"version\":1},{\"idx\":108,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":109,\"version\":1},{\"idx\":110,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":108,\"version\":1},{\"idx\":109,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":111,\"version\":1},{\"idx\":112,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":51,\"version\":1},{\"idx\":181,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":182,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":113,\"version\":1},{\"idx\":114,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":53,\"version\":1},{\"idx\":115,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":51,\"version\":1},{\"idx\":116,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":115,\"version\":1},{\"idx\":183,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":116,\"version\":1},{\"idx\":184,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":117,\"version\":1},{\"idx\":118,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":185,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":118,\"version\":1},{\"idx\":119,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":114,\"version\":1},{\"idx\":120,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":119,\"version\":1},{\"idx\":120,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":120,\"version\":1},{\"idx\":121,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":61,\"version\":1},{\"idx\":122,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":123,\"version\":3},{\"idx\":11,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":122,\"version\":1},{\"idx\":124,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":121,\"version\":1},{\"idx\":125,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":124,\"version\":1},{\"idx\":186,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":125,\"version\":1},{\"idx\":126,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":58,\"version\":1},{\"idx\":127,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":127,\"version\":1},{\"idx\":187,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":61,\"version\":1},{\"idx\":188,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":128,\"version\":1},{\"idx\":129,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":189,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":129,\"version\":1},{\"idx\":130,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":126,\"version\":1},{\"idx\":131,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":130,\"version\":1},{\"idx\":131,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":131,\"version\":1},{\"idx\":132,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":13,\"version\":1},{\"idx\":190,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":132,\"version\":1},{\"idx\":133,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":112,\"version\":1},{\"idx\":134,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":133,\"version\":1},{\"idx\":134,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":135,\"version\":1},{\"idx\":136,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":134,\"version\":1},{\"idx\":135,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":137,\"version\":1},{\"idx\":138,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":139,\"version\":1},{\"idx\":140,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":141,\"version\":1},{\"idx\":142,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":82,\"version\":1},{\"idx\":143,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":142,\"version\":1},{\"idx\":144,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":143,\"version\":1},{\"idx\":191,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":145,\"version\":1},{\"idx\":146,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":144,\"version\":1},{\"idx\":145,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":147,\"version\":1},{\"idx\":26,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":148,\"version\":1},{\"idx\":28,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":149,\"version\":1},{\"idx\":29,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":150,\"version\":1},{\"idx\":31,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":151,\"version\":1},{\"idx\":31,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":152,\"version\":1},{\"idx\":33,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":153,\"version\":1},{\"idx\":35,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":154,\"version\":1},{\"idx\":35,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":155,\"version\":1},{\"idx\":40,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":156,\"version\":1},{\"idx\":43,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":157,\"version\":1},{\"idx\":43,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":158,\"version\":1},{\"idx\":50,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":159,\"version\":1},{\"idx\":52,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":160,\"version\":1},{\"idx\":59,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":161,\"version\":1},{\"idx\":62,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":162,\"version\":1},{\"idx\":62,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":163,\"version\":1},{\"idx\":64,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":164,\"version\":1},{\"idx\":69,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":165,\"version\":1},{\"idx\":69,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":166,\"version\":1},{\"idx\":72,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":167,\"version\":1},{\"idx\":74,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":168,\"version\":1},{\"idx\":75,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":169,\"version\":1},{\"idx\":78,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":170,\"version\":1},{\"idx\":80,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":171,\"version\":1},{\"idx\":80,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":172,\"version\":1},{\"idx\":86,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":173,\"version\":1},{\"idx\":86,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":174,\"version\":1},{\"idx\":87,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":175,\"version\":1},{\"idx\":89,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":176,\"version\":1},{\"idx\":89,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":177,\"version\":1},{\"idx\":91,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":178,\"version\":1},{\"idx\":94,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":179,\"version\":1},{\"idx\":100,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":180,\"version\":1},{\"idx\":105,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":181,\"version\":1},{\"idx\":113,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":182,\"version\":1},{\"idx\":113,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":183,\"version\":1},{\"idx\":117,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":184,\"version\":1},{\"idx\":117,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":185,\"version\":1},{\"idx\":118,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":186,\"version\":1},{\"idx\":125,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":187,\"version\":1},{\"idx\":128,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":188,\"version\":1},{\"idx\":128,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":189,\"version\":1},{\"idx\":129,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":190,\"version\":1},{\"idx\":132,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":191,\"version\":1},{\"idx\":144,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":192,\"version\":1},{\"idx\":11,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":193,\"version\":1},{\"idx\":96,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"pos\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Path\":\"pos\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"pos\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"neg\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"pos\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"neg\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"pos\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"neg\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"pos\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"neg\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"neg\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"pos\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"neg\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Path\":\"neg\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":27,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":6,\"version\":1},\"version\":1},{\"value\":{\"idx\":6,\"version\":1},\"version\":1},{\"value\":{\"idx\":6,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":{\"idx\":6,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":{\"idx\":8,\"version\":1},\"version\":1},{\"value\":{\"idx\":8,\"version\":1},\"version\":1},{\"value\":{\"idx\":9,\"version\":1},\"version\":1},{\"value\":{\"idx\":9,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":10,\"version\":1},\"version\":1},{\"value\":{\"idx\":10,\"version\":1},\"version\":1},{\"value\":{\"idx\":10,\"version\":1},\"version\":1},{\"value\":{\"idx\":10,\"version\":1},\"version\":1},{\"value\":{\"idx\":10,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":10,\"version\":1},\"version\":1},{\"value\":{\"idx\":21,\"version\":1},\"version\":1},{\"value\":{\"idx\":21,\"version\":1},\"version\":1},{\"value\":{\"idx\":20,\"version\":1},\"version\":1},{\"value\":{\"idx\":20,\"version\":1},\"version\":1},{\"value\":{\"idx\":20,\"version\":1},\"version\":1},{\"value\":{\"idx\":23,\"version\":1},\"version\":1},{\"value\":{\"idx\":23,\"version\":1},\"version\":1},{\"value\":{\"idx\":23,\"version\":1},\"version\":1},{\"value\":{\"idx\":23,\"version\":1},\"version\":1},{\"value\":{\"idx\":11,\"version\":1},\"version\":1},{\"value\":{\"idx\":11,\"version\":1},\"version\":1},{\"value\":{\"idx\":22,\"version\":1},\"version\":1},{\"value\":{\"idx\":12,\"version\":1},\"version\":1},{\"value\":{\"idx\":12,\"version\":1},\"version\":1},{\"value\":{\"idx\":12,\"version\":1},\"version\":1},{\"value\":{\"idx\":22,\"version\":1},\"version\":1},{\"value\":{\"idx\":11,\"version\":1},\"version\":1},{\"value\":{\"idx\":11,\"version\":1},\"version\":1},{\"value\":{\"idx\":11,\"version\":1},\"version\":1},{\"value\":{\"idx\":12,\"version\":1},\"version\":1},{\"value\":{\"idx\":12,\"version\":1},\"version\":1},{\"value\":{\"idx\":12,\"version\":1},\"version\":1},{\"value\":{\"idx\":12,\"version\":1},\"version\":1},{\"value\":{\"idx\":12,\"version\":1},\"version\":1},{\"value\":{\"idx\":22,\"version\":1},\"version\":1},{\"value\":{\"idx\":16,\"version\":1},\"version\":1},{\"value\":{\"idx\":16,\"version\":1},\"version\":1},{\"value\":{\"idx\":16,\"version\":1},\"version\":1},{\"value\":{\"idx\":13,\"version\":1},\"version\":1},{\"value\":{\"idx\":13,\"version\":1},\"version\":1},{\"value\":{\"idx\":25,\"version\":1},\"version\":1},{\"value\":{\"idx\":25,\"version\":1},\"version\":1},{\"value\":{\"idx\":25,\"version\":1},\"version\":1},{\"value\":{\"idx\":25,\"version\":1},\"version\":1},{\"value\":{\"idx\":13,\"version\":1},\"version\":1},{\"value\":{\"idx\":13,\"version\":1},\"version\":1},{\"value\":{\"idx\":14,\"version\":1},\"version\":1},{\"value\":{\"idx\":15,\"version\":1},\"version\":1},{\"value\":{\"idx\":16,\"version\":1},\"version\":1},{\"value\":{\"idx\":17,\"version\":1},\"version\":1},{\"value\":{\"idx\":17,\"version\":1},\"version\":1},{\"value\":{\"idx\":16,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":28,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":18,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":19,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":20,\"version\":1},\"version\":1},{\"value\":{\"idx\":21,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":22,\"version\":1},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"idx\":22,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":23,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":24,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":26,\"version\":1},\"version\":1},{\"value\":{\"idx\":26,\"version\":1},\"version\":1},{\"value\":{\"idx\":25,\"version\":1},\"version\":1},{\"value\":{\"idx\":26,\"version\":1},\"version\":1},{\"value\":{\"idx\":26,\"version\":1},\"version\":1},{\"value\":{\"idx\":26,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":23,\"version\":1},{\"idx\":33,\"version\":1},{\"idx\":34,\"version\":1},{\"idx\":9,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":7,\"version\":1},{\"idx\":8,\"version\":1},{\"idx\":11,\"version\":1},{\"idx\":12,\"version\":1},{\"idx\":14,\"version\":1},{\"idx\":15,\"version\":1},{\"idx\":16,\"version\":1},{\"idx\":17,\"version\":1},{\"idx\":18,\"version\":1},{\"idx\":19,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":137,\"version\":1},{\"idx\":138,\"version\":1},{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1},{\"idx\":24,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":139,\"version\":1},{\"idx\":140,\"version\":1},{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1},{\"idx\":25,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":20,\"version\":1},{\"idx\":21,\"version\":1},{\"idx\":22,\"version\":1},{\"idx\":27,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":26,\"version\":1},{\"idx\":28,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":29,\"version\":1},{\"idx\":30,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":31,\"version\":1},{\"idx\":32,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":35,\"version\":1},{\"idx\":36,\"version\":1},{\"idx\":37,\"version\":1},{\"idx\":38,\"version\":1},{\"idx\":39,\"version\":1},{\"idx\":49,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":59,\"version\":1},{\"idx\":60,\"version\":1},{\"idx\":66,\"version\":1},{\"idx\":67,\"version\":1},{\"idx\":68,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":62,\"version\":1},{\"idx\":63,\"version\":1},{\"idx\":64,\"version\":1},{\"idx\":69,\"version\":1},{\"idx\":70,\"version\":1},{\"idx\":71,\"version\":1},{\"idx\":72,\"version\":1},{\"idx\":73,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":78,\"version\":1},{\"idx\":79,\"version\":1},{\"idx\":84,\"version\":1},{\"idx\":85,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":86,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":87,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":91,\"version\":1},{\"idx\":75,\"version\":1},{\"idx\":76,\"version\":1},{\"idx\":77,\"version\":1},{\"idx\":88,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":89,\"version\":1},{\"idx\":90,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":40,\"version\":1},{\"idx\":41,\"version\":1},{\"idx\":42,\"version\":1},{\"idx\":43,\"version\":1},{\"idx\":44,\"version\":1},{\"idx\":45,\"version\":1},{\"idx\":47,\"version\":1},{\"idx\":48,\"version\":1},{\"idx\":13,\"version\":1},{\"idx\":99,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":92,\"version\":1},{\"idx\":93,\"version\":1},{\"idx\":94,\"version\":1},{\"idx\":96,\"version\":1},{\"idx\":97,\"version\":1},{\"idx\":98,\"version\":1},{\"idx\":100,\"version\":1},{\"idx\":101,\"version\":1},{\"idx\":102,\"version\":1},{\"idx\":103,\"version\":1},{\"idx\":105,\"version\":1},{\"idx\":106,\"version\":1},{\"idx\":107,\"version\":1},{\"idx\":108,\"version\":1},{\"idx\":109,\"version\":1},{\"idx\":110,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":52,\"version\":1},{\"idx\":53,\"version\":1},{\"idx\":54,\"version\":1},{\"idx\":115,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":50,\"version\":1},{\"idx\":51,\"version\":1},{\"idx\":116,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":74,\"version\":1},{\"idx\":61,\"version\":1},{\"idx\":65,\"version\":1},{\"idx\":122,\"version\":1},{\"idx\":124,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":55,\"version\":1},{\"idx\":56,\"version\":1},{\"idx\":57,\"version\":1},{\"idx\":58,\"version\":1},{\"idx\":127,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":111,\"version\":1},{\"idx\":112,\"version\":1},{\"idx\":113,\"version\":1},{\"idx\":114,\"version\":1},{\"idx\":117,\"version\":1},{\"idx\":118,\"version\":1},{\"idx\":119,\"version\":1},{\"idx\":120,\"version\":1},{\"idx\":121,\"version\":1},{\"idx\":125,\"version\":1},{\"idx\":126,\"version\":1},{\"idx\":128,\"version\":1},{\"idx\":129,\"version\":1},{\"idx\":130,\"version\":1},{\"idx\":131,\"version\":1},{\"idx\":132,\"version\":1},{\"idx\":133,\"version\":1},{\"idx\":134,\"version\":1},{\"idx\":135,\"version\":1},{\"idx\":136,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":80,\"version\":1},{\"idx\":81,\"version\":1},{\"idx\":82,\"version\":1},{\"idx\":83,\"version\":1},{\"idx\":143,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":141,\"version\":1},{\"idx\":142,\"version\":1},{\"idx\":144,\"version\":1},{\"idx\":145,\"version\":1},{\"idx\":146,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":10,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":95,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1},{\"value\":1,\"version\":1},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1},{\"value\":1,\"version\":1},{\"value\":1,\"version\":1},{\"value\":0,\"version\":1},{\"value\":1,\"version\":1},{\"value\":2,\"version\":1},{\"value\":1,\"version\":1},{\"value\":2,\"version\":1},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1},{\"value\":2,\"version\":1},{\"value\":1,\"version\":1},{\"value\":2,\"version\":1},{\"value\":2,\"version\":1},{\"value\":1,\"version\":1},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1},{\"value\":2,\"version\":1},{\"value\":1,\"version\":1},{\"value\":1,\"version\":1},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1}],\"node_singleton_references\":[{\"value\":null,\"version\":0},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"stream_0\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":\"stream_1\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":\"stream_2\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":\"stream_3\",\"version\":1},{\"value\":\"stream_4\",\"version\":1},{\"value\":\"stream_5\",\"version\":1},{\"value\":\"stream_6\",\"version\":1},{\"value\":\"stream_7\",\"version\":1},{\"value\":\"stream_8\",\"version\":1},{\"value\":\"stream_9\",\"version\":1},{\"value\":\"stream_10\",\"version\":1},{\"value\":\"stream_11\",\"version\":1},{\"value\":\"stream_12\",\"version\":1},{\"value\":\"stream_13\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":\"stream_14\",\"version\":1},{\"value\":\"stream_14\",\"version\":1},{\"value\":\"stream_15\",\"version\":1},{\"value\":\"stream_16\",\"version\":1},{\"value\":\"stream_17\",\"version\":1},{\"value\":\"stream_18\",\"version\":1},{\"value\":\"stream_19\",\"version\":1},{\"value\":\"stream_20\",\"version\":1},{\"value\":\"stream_21\",\"version\":1},{\"value\":\"stream_22\",\"version\":1},{\"value\":\"stream_23\",\"version\":1},{\"value\":\"stream_24\",\"version\":1},{\"value\":\"stream_25\",\"version\":1},{\"value\":\"stream_26\",\"version\":1},{\"value\":\"stream_27\",\"version\":1},{\"value\":\"stream_28\",\"version\":1},{\"value\":\"stream_29\",\"version\":1},{\"value\":\"stream_30\",\"version\":1},{\"value\":\"stream_31\",\"version\":1},{\"value\":\"stream_32\",\"version\":1},{\"value\":\"stream_33\",\"version\":1},{\"value\":\"stream_34\",\"version\":1},{\"value\":\"stream_35\",\"version\":1},{\"value\":\"stream_36\",\"version\":1},{\"value\":\"stream_37\",\"version\":1},{\"value\":\"stream_38\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":\"stream_40\",\"version\":1},{\"value\":\"stream_41\",\"version\":1},{\"value\":\"stream_42\",\"version\":1},{\"value\":\"stream_43\",\"version\":1},{\"value\":\"stream_44\",\"version\":1},{\"value\":\"stream_45\",\"version\":1},{\"value\":\"stream_46\",\"version\":1},{\"value\":\"stream_47\",\"version\":1},{\"value\":\"stream_52\",\"version\":1},{\"value\":\"stream_52\",\"version\":1},{\"value\":\"stream_53\",\"version\":1},{\"value\":\"stream_54\",\"version\":1},{\"value\":\"stream_55\",\"version\":1},{\"value\":\"stream_56\",\"version\":1},{\"value\":\"stream_57\",\"version\":1},{\"value\":\"stream_58\",\"version\":1},{\"value\":\"stream_59\",\"version\":1},{\"value\":\"stream_60\",\"version\":1},{\"value\":\"stream_61\",\"version\":1},{\"value\":\"stream_62\",\"version\":1},{\"value\":\"stream_63\",\"version\":1},{\"value\":\"stream_64\",\"version\":1},{\"value\":\"stream_65\",\"version\":1},{\"value\":\"stream_66\",\"version\":1},{\"value\":\"stream_67\",\"version\":1},{\"value\":\"stream_68\",\"version\":1},{\"value\":\"stream_69\",\"version\":1},{\"value\":\"stream_70\",\"version\":1},{\"value\":\"stream_71\",\"version\":1},{\"value\":\"stream_72\",\"version\":1},{\"value\":\"stream_73\",\"version\":1},{\"value\":\"stream_74\",\"version\":1},{\"value\":\"stream_75\",\"version\":1},{\"value\":\"stream_76\",\"version\":1},{\"value\":\"stream_77\",\"version\":1},{\"value\":\"stream_78\",\"version\":1},{\"value\":\"stream_79\",\"version\":1},{\"value\":\"stream_80\",\"version\":1},{\"value\":\"stream_81\",\"version\":1},{\"value\":\"stream_82\",\"version\":1},{\"value\":\"stream_83\",\"version\":1},{\"value\":\"stream_84\",\"version\":1},{\"value\":\"stream_85\",\"version\":1},{\"value\":\"stream_86\",\"version\":1},{\"value\":\"stream_87\",\"version\":1},{\"value\":\"stream_89\",\"version\":1},{\"value\":\"stream_90\",\"version\":1},{\"value\":\"stream_91\",\"version\":1},{\"value\":\"stream_92\",\"version\":1},{\"value\":\"stream_93\",\"version\":1},{\"value\":\"stream_94\",\"version\":1},{\"value\":\"stream_95\",\"version\":1},{\"value\":\"stream_96\",\"version\":1},{\"value\":\"stream_97\",\"version\":1},{\"value\":\"stream_98\",\"version\":1},{\"value\":\"stream_99\",\"version\":1},{\"value\":\"stream_100\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":\"stream_102\",\"version\":1},{\"value\":\"stream_103\",\"version\":1},{\"value\":\"stream_104\",\"version\":1},{\"value\":\"stream_105\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":\"stream_112\",\"version\":1},{\"value\":\"stream_113\",\"version\":1},{\"value\":\"stream_114\",\"version\":1},{\"value\":\"stream_115\",\"version\":1},{\"value\":\"stream_116\",\"version\":1},{\"value\":\"stream_117\",\"version\":1},{\"value\":\"stream_118\",\"version\":1},{\"value\":\"stream_119\",\"version\":1},{\"value\":\"stream_120\",\"version\":1},{\"value\":\"stream_121\",\"version\":1},{\"value\":\"stream_122\",\"version\":1},{\"value\":\"stream_123\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":\"stream_125\",\"version\":1},{\"value\":\"stream_126\",\"version\":1},{\"value\":\"stream_127\",\"version\":1},{\"value\":\"stream_128\",\"version\":1},{\"value\":\"stream_129\",\"version\":1},{\"value\":\"stream_130\",\"version\":1},{\"value\":\"stream_131\",\"version\":1},{\"value\":\"stream_132\",\"version\":1},{\"value\":\"stream_133\",\"version\":1},{\"value\":\"stream_134\",\"version\":1},{\"value\":\"stream_135\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":\"stream_146\",\"version\":1},{\"value\":\"stream_146\",\"version\":1},{\"value\":\"stream_149\",\"version\":1},{\"value\":\"stream_149\",\"version\":1},{\"value\":\"stream_150\",\"version\":1},{\"value\":\"stream_151\",\"version\":1},{\"value\":\"stream_152\",\"version\":1},{\"value\":\"stream_153\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":1},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":10,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":15,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":54,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":60,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":67,\"lattice_flow_type\":null},\"version\":1},{\"value\":{\"star_ord\":67,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":67,\"lattice_flow_type\":null},\"version\":1},{\"value\":{\"star_ord\":67,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":99,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":99,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":109,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":15,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":67,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":41,\"lattice_flow_type\":null},\"version\":1}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}");
                            df.__assign_diagnostics("[]");
                            let(hoff_46v3_send,hoff_46v3_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(46v3)");
                            let(hoff_104v3_send,hoff_104v3_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(104v3)");
                            let(hoff_123v3_send,hoff_123v3_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(123v3)");
                            let(hoff_147v1_send,hoff_147v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(147v1)");
                            let(hoff_148v1_send,hoff_148v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(148v1)");
                            let(hoff_149v1_send,hoff_149v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(149v1)");
                            let(hoff_150v1_send,hoff_150v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(150v1)");
                            let(hoff_151v1_send,hoff_151v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(151v1)");
                            let(hoff_152v1_send,hoff_152v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(152v1)");
                            let(hoff_153v1_send,hoff_153v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(153v1)");
                            let(hoff_154v1_send,hoff_154v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(154v1)");
                            let(hoff_155v1_send,hoff_155v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(155v1)");
                            let(hoff_156v1_send,hoff_156v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(156v1)");
                            let(hoff_157v1_send,hoff_157v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(157v1)");
                            let(hoff_158v1_send,hoff_158v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(158v1)");
                            let(hoff_159v1_send,hoff_159v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(159v1)");
                            let(hoff_160v1_send,hoff_160v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(160v1)");
                            let(hoff_161v1_send,hoff_161v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(161v1)");
                            let(hoff_162v1_send,hoff_162v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(162v1)");
                            let(hoff_163v1_send,hoff_163v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(163v1)");
                            let(hoff_164v1_send,hoff_164v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(164v1)");
                            let(hoff_165v1_send,hoff_165v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(165v1)");
                            let(hoff_166v1_send,hoff_166v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(166v1)");
                            let(hoff_167v1_send,hoff_167v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(167v1)");
                            let(hoff_168v1_send,hoff_168v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(168v1)");
                            let(hoff_169v1_send,hoff_169v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(169v1)");
                            let(hoff_170v1_send,hoff_170v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(170v1)");
                            let(hoff_171v1_send,hoff_171v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(171v1)");
                            let(hoff_172v1_send,hoff_172v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(172v1)");
                            let(hoff_173v1_send,hoff_173v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(173v1)");
                            let(hoff_174v1_send,hoff_174v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(174v1)");
                            let(hoff_175v1_send,hoff_175v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(175v1)");
                            let(hoff_176v1_send,hoff_176v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(176v1)");
                            let(hoff_177v1_send,hoff_177v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(177v1)");
                            let(hoff_178v1_send,hoff_178v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(178v1)");
                            let(hoff_179v1_send,hoff_179v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(179v1)");
                            let(hoff_180v1_send,hoff_180v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(180v1)");
                            let(hoff_181v1_send,hoff_181v1_recv) = df.make_edge::<&str, hydroflow_plus::scheduled::handoff::VecHandoff<(i32, (usize, hydroflow_plus_test::__staged::cluster::paxos::LogValue))>>("handoff GraphNodeId(181v1)");
                            let(hoff_182v1_send,hoff_182v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(182v1)");
                            let(hoff_183v1_send,hoff_183v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(183v1)");
                            let(hoff_184v1_send,hoff_184v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(184v1)");
                            let(hoff_185v1_send,hoff_185v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(185v1)");
                            let(hoff_186v1_send,hoff_186v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(186v1)");
                            let(hoff_187v1_send,hoff_187v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(187v1)");
                            let(hoff_188v1_send,hoff_188v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(188v1)");
                            let(hoff_189v1_send,hoff_189v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(189v1)");
                            let(hoff_190v1_send,hoff_190v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(190v1)");
                            let(hoff_191v1_send,hoff_191v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(191v1)");
                            let(hoff_192v1_send,hoff_192v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(192v1)");
                            let(hoff_193v1_send,hoff_193v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(193v1)");
                            let mut sg_1v1_node_1v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    ["Proposers say hello"]
                                })
                            };
                            let mut sg_2v1_node_23v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    [0]
                                })
                            };
                            let mut sg_3v1_node_7v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let cli = cli;
                                    let self_id = 0usize;
                                    cli.meta.clusters.get(&self_id).unwrap()
                                })
                            };
                            let sg_3v1_node_11v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_3v1_node_11v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_3v1_node_11v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_3v1_node_11v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_3v1_node_14v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_3v1_node_14v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_3v1_node_14v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_3v1_node_14v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_3v1_node_17v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            let sg_3v1_node_17v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_3v1_node_17v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let(sg_3v1_node_19v1_item_send,sg_3v1_node_19v1_item_recv) = hydroflow_plus::tokio::sync::mpsc::unbounded_channel();
                            {
                                #[doc = r" Function is needed so `Item` is so no ambiguity for what `Item` is used"]
                                #[doc = r" when calling `.flush()`."]
                                async fn sink_feed_flush<Sink,Item>(mut recv:hydroflow_plus::tokio::sync::mpsc::UnboundedReceiver<Item> ,mut sink:Sink,)where Sink: ::std::marker::Unpin+hydroflow_plus::futures::Sink<Item> ,Sink::Error: ::std::fmt::Debug,{
                                    use hydroflow_plus::futures::SinkExt;
                                    while let Some(item) = recv.recv().await {
                                        sink.feed(item).await.expect("Error processing async sink item.");
                                        while let Ok(item) = recv.try_recv(){
                                            sink.feed(item).await.expect("Error processing async sink item.");
                                        }sink.flush().await.expect("Failed to flush sink.");
                                    }
                                }
                                df.request_task(sink_feed_flush(sg_3v1_node_19v1_item_recv,{
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_1";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedDemux<ConnectedDirect> >().into_sink()
                                    }
                                }));
                            }let mut sg_4v1_node_137v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_6";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedTagged<ConnectedDirect> >().into_source()
                                    }
                                })
                            };
                            let mut sg_5v1_node_139v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_7";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedTagged<ConnectedDirect> >().into_source()
                                    }
                                })
                            };
                            let mut sg_6v1_node_20v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_2";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedTagged<ConnectedDirect> >().into_source()
                                    }
                                })
                            };
                            let mut sg_19v1_node_92v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let cli = cli;
                                    let self_id = 1usize;
                                    cli.meta.clusters.get(&self_id).unwrap()
                                })
                            };
                            #[allow(unused_mut)]
                            let mut sg_19v1_node_94v1_initializer_func = {
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                | |SystemTime::now()
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_94v1 = df.add_state(::std::cell::RefCell::new((sg_19v1_node_94v1_initializer_func)()));
                            let sg_19v1_node_96v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_19v1_node_96v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_19v1_node_96v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_19v1_node_96v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_19v1_node_100v1_antijoindata_neg = df.add_state(std::cell::RefCell::new(hydroflow_plus::util::monotonic_map::MonotonicMap:: <_,hydroflow_plus::rustc_hash::FxHashSet<_> > ::default()));
                            let sg_19v1_node_105v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_19v1_node_105v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_19v1_node_105v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_19v1_node_105v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_19v1_node_108v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            let sg_19v1_node_108v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_19v1_node_108v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let(sg_19v1_node_110v1_item_send,sg_19v1_node_110v1_item_recv) = hydroflow_plus::tokio::sync::mpsc::unbounded_channel();
                            {
                                #[doc = r" Function is needed so `Item` is so no ambiguity for what `Item` is used"]
                                #[doc = r" when calling `.flush()`."]
                                async fn sink_feed_flush<Sink,Item>(mut recv:hydroflow_plus::tokio::sync::mpsc::UnboundedReceiver<Item> ,mut sink:Sink,)where Sink: ::std::marker::Unpin+hydroflow_plus::futures::Sink<Item> ,Sink::Error: ::std::fmt::Debug,{
                                    use hydroflow_plus::futures::SinkExt;
                                    while let Some(item) = recv.recv().await {
                                        sink.feed(item).await.expect("Error processing async sink item.");
                                        while let Ok(item) = recv.try_recv(){
                                            sink.feed(item).await.expect("Error processing async sink item.");
                                        }sink.flush().await.expect("Failed to flush sink.");
                                    }
                                }
                                df.request_task(sink_feed_flush(sg_19v1_node_110v1_item_recv,{
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_3";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedDemux<ConnectedDirect> >().into_sink()
                                    }
                                }));
                            }let mut sg_23v1_node_55v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_0";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedTagged<ConnectedDirect> >().into_source()
                                    }
                                })
                            };
                            let sg_23v1_node_127v1_counterdata = df.add_state(::std::cell::RefCell::new(0..));
                            df.set_state_tick_hook(sg_23v1_node_127v1_counterdata, |rcell|{
                                rcell.replace(0..);
                            });
                            let mut sg_24v1_node_111v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let cli = cli;
                                    let self_id = 1usize;
                                    cli.meta.clusters.get(&self_id).unwrap()
                                })
                            };
                            let sg_24v1_node_113v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_113v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_113v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_113v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_117v1_antijoindata_neg = df.add_state(std::cell::RefCell::new(hydroflow_plus::util::monotonic_map::MonotonicMap:: <_,hydroflow_plus::rustc_hash::FxHashSet<_> > ::default()));
                            let sg_24v1_node_118v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_118v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_118v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_118v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_125v1_antijoindata_neg = df.add_state(std::cell::RefCell::new(hydroflow_plus::util::monotonic_map::MonotonicMap:: <_,hydroflow_plus::rustc_hash::FxHashSet<_> > ::default()));
                            let sg_24v1_node_128v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_128v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_128v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_128v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_129v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_129v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_129v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_129v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_132v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_132v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_132v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_132v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_24v1_node_134v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            let sg_24v1_node_134v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_24v1_node_134v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let(sg_24v1_node_136v1_item_send,sg_24v1_node_136v1_item_recv) = hydroflow_plus::tokio::sync::mpsc::unbounded_channel();
                            {
                                #[doc = r" Function is needed so `Item` is so no ambiguity for what `Item` is used"]
                                #[doc = r" when calling `.flush()`."]
                                async fn sink_feed_flush<Sink,Item>(mut recv:hydroflow_plus::tokio::sync::mpsc::UnboundedReceiver<Item> ,mut sink:Sink,)where Sink: ::std::marker::Unpin+hydroflow_plus::futures::Sink<Item> ,Sink::Error: ::std::fmt::Debug,{
                                    use hydroflow_plus::futures::SinkExt;
                                    while let Some(item) = recv.recv().await {
                                        sink.feed(item).await.expect("Error processing async sink item.");
                                        while let Ok(item) = recv.try_recv(){
                                            sink.feed(item).await.expect("Error processing async sink item.");
                                        }sink.flush().await.expect("Failed to flush sink.");
                                    }
                                }
                                df.request_task(sink_feed_flush(sg_24v1_node_136v1_item_recv,{
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_4";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedDemux<ConnectedDirect> >().into_sink()
                                    }
                                }));
                            }let mut sg_26v1_node_141v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let cli = cli;
                                    let self_id = 3usize;
                                    cli.meta.clusters.get(&self_id).unwrap()
                                })
                            };
                            let sg_26v1_node_144v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            let sg_26v1_node_144v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_26v1_node_144v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let(sg_26v1_node_146v1_item_send,sg_26v1_node_146v1_item_recv) = hydroflow_plus::tokio::sync::mpsc::unbounded_channel();
                            {
                                #[doc = r" Function is needed so `Item` is so no ambiguity for what `Item` is used"]
                                #[doc = r" when calling `.flush()`."]
                                async fn sink_feed_flush<Sink,Item>(mut recv:hydroflow_plus::tokio::sync::mpsc::UnboundedReceiver<Item> ,mut sink:Sink,)where Sink: ::std::marker::Unpin+hydroflow_plus::futures::Sink<Item> ,Sink::Error: ::std::fmt::Debug,{
                                    use hydroflow_plus::futures::SinkExt;
                                    while let Some(item) = recv.recv().await {
                                        sink.feed(item).await.expect("Error processing async sink item.");
                                        while let Ok(item) = recv.try_recv(){
                                            sink.feed(item).await.expect("Error processing async sink item.");
                                        }sink.flush().await.expect("Failed to flush sink.");
                                    }
                                }
                                df.request_task(sink_feed_flush(sg_26v1_node_146v1_item_recv,{
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_5";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedDemux<ConnectedDirect> >().into_sink()
                                    }
                                }));
                            }let sg_27v1_node_10v1_intervalstream = hydroflow_plus::tokio_stream::StreamExt::map(hydroflow_plus::tokio_stream::wrappers::IntervalStream::new(hydroflow_plus::tokio::time::interval({
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                let i_am_leader_send_timeout = i_am_leader_send_timeout;
                                *i_am_leader_send_timeout
                            })), |_|{}
                            );
                            let mut sg_27v1_node_10v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream(sg_27v1_node_10v1_intervalstream)
                            };
                            let sg_28v1_node_95v1_intervalstream = hydroflow_plus::tokio_stream::StreamExt::map(hydroflow_plus::tokio_stream::wrappers::IntervalStream::new(hydroflow_plus::tokio::time::interval({
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                let i_am_leader_check_timeout = i_am_leader_check_timeout;
                                let p_id = {
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let cli = cli;
                                    cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                };
                                *i_am_leader_check_timeout+Duration::from_secs(p_id.into())
                            })), |_|{}
                            );
                            let mut sg_28v1_node_95v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream(sg_28v1_node_95v1_intervalstream)
                            };
                            #[allow(unused_mut)]
                            let mut sg_8v1_node_29v1_initializer_func = {
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                | |Ballot {
                                    num:0,id:0
                                }
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_29v1 = df.add_state(::std::cell::RefCell::new((sg_8v1_node_29v1_initializer_func)()));
                            let sg_9v1_node_31v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_9v1_node_31v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_9v1_node_31v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_9v1_node_31v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_10v1_node_35v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            let sg_10v1_node_35v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_10v1_node_35v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_10v1_node_39v1_uniquedata = df.add_state(::std::cell::RefCell::new(hydroflow_plus::util::monotonic_map::MonotonicMap:: <_,hydroflow_plus::rustc_hash::FxHashSet<_> > ::default(),));
                            #[allow(unused_mut)]
                            let mut sg_11v1_node_59v1_initializer_func = {
                                use hydroflow_plus::__staged::stream:: * ;
                                | |0usize
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_59v1 = df.add_state(::std::cell::RefCell::new((sg_11v1_node_59v1_initializer_func)()));
                            df.set_state_tick_hook(singleton_op_59v1,move|rcell|{
                                rcell.replace((sg_11v1_node_59v1_initializer_func)());
                            });
                            let sg_12v1_node_62v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_12v1_node_62v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_12v1_node_62v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_12v1_node_62v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_12v1_node_69v1_antijoindata_neg = df.add_state(std::cell::RefCell::new(hydroflow_plus::util::monotonic_map::MonotonicMap:: <_,hydroflow_plus::rustc_hash::FxHashSet<_> > ::default()));
                            let sg_12v1_node_72v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_12v1_node_72v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_12v1_node_72v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_12v1_node_72v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_13v1_node_78v1_groupbydata = df.add_state(::std::cell::RefCell::new(hydroflow_plus::rustc_hash::FxHashMap:: <_,_> ::default()));
                            let sg_14v1_node_86v1_antijoindata_neg = df.add_state(std::cell::RefCell::new(hydroflow_plus::util::monotonic_map::MonotonicMap:: <_,hydroflow_plus::rustc_hash::FxHashSet<_> > ::default()));
                            let sg_17v1_node_89v1_antijoindata_neg = df.add_state(std::cell::RefCell::new(hydroflow_plus::util::monotonic_map::MonotonicMap:: <_,hydroflow_plus::rustc_hash::FxHashSet<_> > ::default()));
                            #[allow(unused_mut)]
                            let mut sg_18v1_node_40v1_initializer_func = {
                                use hydroflow_plus::__staged::stream:: * ;
                                | |0usize
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_40v1 = df.add_state(::std::cell::RefCell::new((sg_18v1_node_40v1_initializer_func)()));
                            df.set_state_tick_hook(singleton_op_40v1,move|rcell|{
                                rcell.replace((sg_18v1_node_40v1_initializer_func)());
                            });
                            let sg_18v1_node_43v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_18v1_node_43v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_18v1_node_43v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_18v1_node_43v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_18v1_node_47v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_18v1_node_47v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_18v1_node_47v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_18v1_node_47v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            #[allow(unused_mut)]
                            let mut sg_20v1_node_52v1_initializer_func = {
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                | |0
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_52v1 = df.add_state(::std::cell::RefCell::new((sg_20v1_node_52v1_initializer_func)()));
                            df.set_state_tick_hook(singleton_op_52v1,move|rcell|{
                                rcell.replace((sg_20v1_node_52v1_initializer_func)());
                            });
                            let sg_21v1_node_50v1_groupbydata = df.add_state(::std::cell::RefCell::new(hydroflow_plus::rustc_hash::FxHashMap:: <_,_> ::default()));
                            let sg_25v1_node_80v1_antijoindata_neg = df.add_state(std::cell::RefCell::new(hydroflow_plus::util::monotonic_map::MonotonicMap:: <_,hydroflow_plus::rustc_hash::FxHashSet<_> > ::default()));
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(1v1)",0,var_expr!(),var_expr!(),false,move|context,var_args!(),var_args!()|{
                                let op_1v1 = sg_1v1_node_1v1_iter.by_ref();
                                let op_1v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_1v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_1v1__source_iter__loc_unknown_start_0_0_end_0_0(op_1v1)
                                };
                                let op_2v1 = hydroflow_plus::pusherator::for_each::ForEach::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |s|{
                                        //($crate::format_args_nl!("{}",s));
                                    }
                                });
                                let op_2v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_2v1__for_each__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_2v1__for_each__loc_unknown_start_0_0_end_0_0(op_2v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_1v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_1v1(op_1v1,op_2v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(2v1)",0,var_expr!(hoff_152v1_recv),var_expr!(hoff_123v3_send,hoff_151v1_send,hoff_154v1_send,hoff_157v1_send,hoff_180v1_send,hoff_182v1_send,hoff_185v1_send,hoff_189v1_send),false,move|context,var_args!(hoff_152v1_recv),var_args!(hoff_123v3_send,hoff_151v1_send,hoff_154v1_send,hoff_157v1_send,hoff_180v1_send,hoff_182v1_send,hoff_185v1_send,hoff_189v1_send)|{
                                let mut hoff_152v1_recv = hoff_152v1_recv.borrow_mut_swap();
                                let hoff_152v1_recv = hoff_152v1_recv.drain(..);
                                let hoff_123v3_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_123v3_send.give(Some(v));
                                });
                                let hoff_151v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_151v1_send.give(Some(v));
                                });
                                let hoff_154v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_154v1_send.give(Some(v));
                                });
                                let hoff_157v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_157v1_send.give(Some(v));
                                });
                                let hoff_180v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_180v1_send.give(Some(v));
                                });
                                let hoff_182v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_182v1_send.give(Some(v));
                                });
                                let hoff_185v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_185v1_send.give(Some(v));
                                });
                                let hoff_189v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_189v1_send.give(Some(v));
                                });
                                let op_23v1 = sg_2v1_node_23v1_iter.by_ref();
                                let op_23v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_23v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_23v1__source_iter__loc_unknown_start_0_0_end_0_0(op_23v1)
                                };
                                let op_33v1 = {
                                    fn check_input<Iter: ::std::iter::Iterator<Item = Item> ,Item>(iter:Iter) -> impl ::std::iter::Iterator<Item = Item>{
                                        iter
                                    }
                                    check_input:: <_,_>(hoff_152v1_recv)
                                };
                                let op_33v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_33v1__defer_tick__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_33v1__defer_tick__loc_unknown_start_0_0_end_0_0(op_33v1)
                                };
                                let op_34v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(op_23v1,op_33v1)
                                };
                                let op_34v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_34v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_34v1__union__loc_unknown_start_0_0_end_0_0(op_34v1)
                                };
                                let op_9v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_123v3_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_151v1_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_154v1_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_157v1_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_180v1_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_182v1_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_185v1_send,hoff_189v1_send)))))));
                                let op_9v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_9v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_9v1__tee__loc_unknown_start_0_0_end_0_0(op_9v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_2v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_2v1(op_34v1,op_9v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(3v1)",1,var_expr!(hoff_104v3_recv,hoff_123v3_recv,hoff_192v1_recv),var_expr!(),false,move|context,var_args!(hoff_104v3_recv,hoff_123v3_recv,hoff_192v1_recv),var_args!()|{
                                let mut hoff_104v3_recv = hoff_104v3_recv.borrow_mut_swap();
                                let hoff_104v3_recv = hoff_104v3_recv.drain(..);
                                let mut hoff_123v3_recv = hoff_123v3_recv.borrow_mut_swap();
                                let hoff_123v3_recv = hoff_123v3_recv.drain(..);
                                let mut hoff_192v1_recv = hoff_192v1_recv.borrow_mut_swap();
                                let hoff_192v1_recv = hoff_192v1_recv.drain(..);
                                let op_7v1 = sg_3v1_node_7v1_iter.by_ref();
                                let op_7v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_7v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_7v1__source_iter__loc_unknown_start_0_0_end_0_0(op_7v1)
                                };
                                let op_8v1 = op_7v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |d|d.clone()
                                });
                                let op_8v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_8v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_8v1__map__loc_unknown_start_0_0_end_0_0(op_8v1)
                                };
                                let hoff_123v3_recv = hoff_123v3_recv.map(|a|((),a));
                                let hoff_192v1_recv = hoff_192v1_recv.map(|b|((),b));
                                let mut sg_3v1_node_11v1_joindata_lhs_borrow = context.state_ref(sg_3v1_node_11v1_joindata_lhs).borrow_mut();
                                let mut sg_3v1_node_11v1_joindata_rhs_borrow = context.state_ref(sg_3v1_node_11v1_joindata_rhs).borrow_mut();
                                let op_11v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_123v3_recv,hoff_192v1_recv, &mut *sg_3v1_node_11v1_joindata_lhs_borrow, &mut *sg_3v1_node_11v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_11v1 = op_11v1.map(|((),(a,b))|(a,b));
                                let op_11v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_11v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_11v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_11v1)
                                };
                                let op_12v1 = op_11v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(a,_)|a
                                });
                                let op_12v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_12v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_12v1__map__loc_unknown_start_0_0_end_0_0(op_12v1)
                                };
                                let op_12v1 = op_12v1.map(|a|((),a));
                                let hoff_104v3_recv = hoff_104v3_recv.map(|b|((),b));
                                let mut sg_3v1_node_14v1_joindata_lhs_borrow = context.state_ref(sg_3v1_node_14v1_joindata_lhs).borrow_mut();
                                let mut sg_3v1_node_14v1_joindata_rhs_borrow = context.state_ref(sg_3v1_node_14v1_joindata_rhs).borrow_mut();
                                let op_14v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_12v1,hoff_104v3_recv, &mut *sg_3v1_node_14v1_joindata_lhs_borrow, &mut *sg_3v1_node_14v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_14v1 = op_14v1.map(|((),(a,b))|(a,b));
                                let op_14v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_14v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_14v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_14v1)
                                };
                                let op_15v1 = op_14v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(a,_)|a
                                });
                                let op_15v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_15v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_15v1__map__loc_unknown_start_0_0_end_0_0(op_15v1)
                                };
                                let op_16v1 = op_15v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let p_id = {
                                        use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                        let cli = cli;
                                        cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                    };
                                    move|ballot_num|Ballot {
                                        num:ballot_num,id:p_id
                                    }
                                });
                                let op_16v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_16v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_16v1__map__loc_unknown_start_0_0_end_0_0(op_16v1)
                                };
                                let op_8v1 = op_8v1.map(|a|((),a));
                                let op_16v1 = op_16v1.map(|b|((),b));
                                let mut sg_3v1_node_17v1_joindata_lhs_borrow = context.state_ref(sg_3v1_node_17v1_joindata_lhs).borrow_mut();
                                let mut sg_3v1_node_17v1_joindata_rhs_borrow = context.state_ref(sg_3v1_node_17v1_joindata_rhs).borrow_mut();
                                let op_17v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_8v1,op_16v1, &mut *sg_3v1_node_17v1_joindata_lhs_borrow, &mut *sg_3v1_node_17v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_17v1 = op_17v1.map(|((),(a,b))|(a,b));
                                let op_17v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_17v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_17v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_17v1)
                                };
                                let op_18v1 = op_17v1.map(|(id,data)|{
                                    (id,hydroflow_plus::runtime_support::bincode::serialize:: <hydroflow_plus_test::__staged::cluster::paxos::Ballot>(&data).unwrap().into())
                                });
                                let op_18v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_18v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_18v1__map__loc_unknown_start_0_0_end_0_0(op_18v1)
                                };
                                let op_19v1 = hydroflow_plus::pusherator::for_each::ForEach::new(|item|{
                                    if let Err(err) = sg_3v1_node_19v1_item_send.send(item){
                                        {
                                            //$crate::panicking::panic_fmt($crate::const_format_args!("Failed to send async write item for processing.: {}",err));
                                        };
                                    }
                                });
                                let op_19v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_19v1__dest_sink__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_19v1__dest_sink__loc_unknown_start_0_0_end_0_0(op_19v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_3v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_3v1(op_18v1,op_19v1);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(4v1)",0,var_expr!(),var_expr!(hoff_46v3_send,hoff_153v1_send),false,move|context,var_args!(),var_args!(hoff_46v3_send,hoff_153v1_send)|{
                                let hoff_46v3_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_46v3_send.give(Some(v));
                                });
                                let hoff_153v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_153v1_send.give(Some(v));
                                });
                                let op_137v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_4v1_node_137v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_137v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_137v1__source_stream__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_137v1__source_stream__loc_unknown_start_0_0_end_0_0(op_137v1)
                                };
                                let op_138v1 = op_137v1.map(|res|{
                                    let(id,b) = res.unwrap();
                                    (id,hydroflow_plus::runtime_support::bincode::deserialize:: <hydroflow_plus_test::__staged::cluster::paxos::P1b>(&b).unwrap())
                                });
                                let op_138v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_138v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_138v1__map__loc_unknown_start_0_0_end_0_0(op_138v1)
                                };
                                let op_24v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(_,p1b):(_,P1b)|p1b.max_ballot
                                },hoff_46v3_send);
                                let op_24v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_24v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_24v1__map__loc_unknown_start_0_0_end_0_0(op_24v1)
                                };
                                let op_4v1 = hydroflow_plus::pusherator::for_each::ForEach::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(_,p1b):(u32,P1b)|{
                                        //($crate::format_args_nl!("Proposer received P1b: {:?}",p1b));
                                    }
                                });
                                let op_4v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_4v1__for_each__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_4v1__for_each__loc_unknown_start_0_0_end_0_0(op_4v1)
                                };
                                let op_3v1 = hydroflow_plus::pusherator::tee::Tee::new(op_4v1,hydroflow_plus::pusherator::tee::Tee::new(op_24v1,hoff_153v1_send));
                                let op_3v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_3v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_3v1__tee__loc_unknown_start_0_0_end_0_0(op_3v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_4v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_4v1(op_138v1,op_3v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(5v1)",0,var_expr!(),var_expr!(hoff_147v1_send,hoff_168v1_send),false,move|context,var_args!(),var_args!(hoff_147v1_send,hoff_168v1_send)|{
                                let hoff_147v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_147v1_send.give(Some(v));
                                });
                                let hoff_168v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_168v1_send.give(Some(v));
                                });
                                let op_139v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_5v1_node_139v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_139v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_139v1__source_stream__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_139v1__source_stream__loc_unknown_start_0_0_end_0_0(op_139v1)
                                };
                                let op_140v1 = op_139v1.map(|res|{
                                    let(id,b) = res.unwrap();
                                    (id,hydroflow_plus::runtime_support::bincode::deserialize:: <hydroflow_plus_test::__staged::cluster::paxos::P2b>(&b).unwrap())
                                });
                                let op_140v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_140v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_140v1__map__loc_unknown_start_0_0_end_0_0(op_140v1)
                                };
                                let op_25v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(_,p2b):(_,P2b)|p2b.max_ballot
                                },hoff_147v1_send);
                                let op_25v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_25v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_25v1__map__loc_unknown_start_0_0_end_0_0(op_25v1)
                                };
                                let op_6v1 = hydroflow_plus::pusherator::for_each::ForEach::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(_,p2b):(u32,P2b)|{
                                        //($crate::format_args_nl!("Proposer received P2b: {:?}",p2b));
                                    }
                                });
                                let op_6v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_6v1__for_each__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_6v1__for_each__loc_unknown_start_0_0_end_0_0(op_6v1)
                                };
                                let op_5v1 = hydroflow_plus::pusherator::tee::Tee::new(op_6v1,hydroflow_plus::pusherator::tee::Tee::new(op_25v1,hoff_168v1_send));
                                let op_5v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_5v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_5v1__tee__loc_unknown_start_0_0_end_0_0(op_5v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_5v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_5v1(op_140v1,op_5v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(6v1)",0,var_expr!(),var_expr!(hoff_148v1_send,hoff_178v1_send),false,move|context,var_args!(),var_args!(hoff_148v1_send,hoff_178v1_send)|{
                                let hoff_148v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_148v1_send.give(Some(v));
                                });
                                let hoff_178v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_178v1_send.give(Some(v));
                                });
                                let op_20v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_6v1_node_20v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_20v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_20v1__source_stream__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_20v1__source_stream__loc_unknown_start_0_0_end_0_0(op_20v1)
                                };
                                let op_21v1 = op_20v1.map(|res|{
                                    let(id,b) = res.unwrap();
                                    (id,hydroflow_plus::runtime_support::bincode::deserialize:: <hydroflow_plus_test::__staged::cluster::paxos::Ballot>(&b).unwrap())
                                });
                                let op_21v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_21v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_21v1__map__loc_unknown_start_0_0_end_0_0(op_21v1)
                                };
                                let op_22v1 = op_21v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(_,b)|b
                                });
                                let op_22v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_22v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_22v1__map__loc_unknown_start_0_0_end_0_0(op_22v1)
                                };
                                let op_27v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_148v1_send,hoff_178v1_send);
                                let op_27v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_27v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_27v1__tee__loc_unknown_start_0_0_end_0_0(op_27v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_6v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_6v1(op_22v1,op_27v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(19v1)",2,var_expr!(hoff_178v1_recv,hoff_179v1_recv,hoff_180v1_recv,hoff_193v1_recv),var_expr!(),false,move|context,var_args!(hoff_178v1_recv,hoff_179v1_recv,hoff_180v1_recv,hoff_193v1_recv),var_args!()|{
                                let mut hoff_178v1_recv = hoff_178v1_recv.borrow_mut_swap();
                                let hoff_178v1_recv = hoff_178v1_recv.drain(..);
                                let mut hoff_179v1_recv = hoff_179v1_recv.borrow_mut_swap();
                                let hoff_179v1_recv = hoff_179v1_recv.drain(..);
                                let mut hoff_180v1_recv = hoff_180v1_recv.borrow_mut_swap();
                                let hoff_180v1_recv = hoff_180v1_recv.drain(..);
                                let mut hoff_193v1_recv = hoff_193v1_recv.borrow_mut_swap();
                                let hoff_193v1_recv = hoff_193v1_recv.drain(..);
                                let op_92v1 = sg_19v1_node_92v1_iter.by_ref();
                                let op_92v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_92v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_92v1__source_iter__loc_unknown_start_0_0_end_0_0(op_92v1)
                                };
                                let op_93v1 = op_92v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |d|d.clone()
                                });
                                let op_93v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_93v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_93v1__map__loc_unknown_start_0_0_end_0_0(op_93v1)
                                };
                                let op_94v1 = {
                                    let mut sg_19v1_node_94v1_accumulator = context.state_ref(singleton_op_94v1).borrow_mut();
                                    hoff_178v1_recv.for_each(|sg_19v1_node_94v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_19v1_node_94v1_accumulator,sg_19v1_node_94v1_iterator_item,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |latest,_|{
                                                *latest = SystemTime::now();
                                            }
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_19v1_node_94v1_accumulator))
                                    }
                                };
                                let op_94v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_94v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_94v1__fold__loc_unknown_start_0_0_end_0_0(op_94v1)
                                };
                                let op_94v1 = op_94v1.map(|a|((),a));
                                let hoff_193v1_recv = hoff_193v1_recv.map(|b|((),b));
                                let mut sg_19v1_node_96v1_joindata_lhs_borrow = context.state_ref(sg_19v1_node_96v1_joindata_lhs).borrow_mut();
                                let mut sg_19v1_node_96v1_joindata_rhs_borrow = context.state_ref(sg_19v1_node_96v1_joindata_rhs).borrow_mut();
                                let op_96v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_94v1,hoff_193v1_recv, &mut *sg_19v1_node_96v1_joindata_lhs_borrow, &mut *sg_19v1_node_96v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_96v1 = op_96v1.map(|((),(a,b))|(a,b));
                                let op_96v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_96v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_96v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_96v1)
                                };
                                let op_97v1 = op_96v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(a,_)|a
                                });
                                let op_97v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_97v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_97v1__map__loc_unknown_start_0_0_end_0_0(op_97v1)
                                };
                                let op_98v1 = op_97v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |a|((),a)
                                });
                                let op_98v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_98v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_98v1__map__loc_unknown_start_0_0_end_0_0(op_98v1)
                                };
                                let mut sg_19v1_node_100v1_antijoindata_neg_borrow = context.state_ref(sg_19v1_node_100v1_antijoindata_neg).borrow_mut();
                                #[allow(clippy::needless_borrow)]
                                (&mut *sg_19v1_node_100v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).extend(hoff_179v1_recv);
                                let op_100v1 = op_98v1.filter(|x: &(_,_)|{
                                    #[allow(clippy::needless_borrow)]
                                    #[allow(clippy::unnecessary_mut_passed)]
                                    !(&mut *sg_19v1_node_100v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).contains(&x.0)
                                });
                                let op_100v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_100v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_100v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0(op_100v1)
                                };
                                let op_101v1 = op_100v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(_,a)|a
                                });
                                let op_101v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_101v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_101v1__map__loc_unknown_start_0_0_end_0_0(op_101v1)
                                };
                                let op_102v1 = op_101v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let i_am_leader_check_timeout = i_am_leader_check_timeout;
                                    |latest_received_i_am_leader|SystemTime::now()- *i_am_leader_check_timeout>latest_received_i_am_leader
                                });
                                let op_102v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_102v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_102v1__map__loc_unknown_start_0_0_end_0_0(op_102v1)
                                };
                                let op_103v1 = op_102v1.filter({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |b| *b
                                });
                                let op_103v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_103v1__filter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_103v1__filter__loc_unknown_start_0_0_end_0_0(op_103v1)
                                };
                                let hoff_180v1_recv = hoff_180v1_recv.map(|a|((),a));
                                let op_103v1 = op_103v1.map(|b|((),b));
                                let mut sg_19v1_node_105v1_joindata_lhs_borrow = context.state_ref(sg_19v1_node_105v1_joindata_lhs).borrow_mut();
                                let mut sg_19v1_node_105v1_joindata_rhs_borrow = context.state_ref(sg_19v1_node_105v1_joindata_rhs).borrow_mut();
                                let op_105v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_180v1_recv,op_103v1, &mut *sg_19v1_node_105v1_joindata_lhs_borrow, &mut *sg_19v1_node_105v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_105v1 = op_105v1.map(|((),(a,b))|(a,b));
                                let op_105v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_105v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_105v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_105v1)
                                };
                                let op_106v1 = op_105v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(a,_)|a
                                });
                                let op_106v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_106v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_106v1__map__loc_unknown_start_0_0_end_0_0(op_106v1)
                                };
                                let op_107v1 = op_106v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let p_id = {
                                        use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                        let cli = cli;
                                        cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                    };
                                    move|ballot_num|P1a {
                                        ballot:Ballot {
                                            num:ballot_num,id:p_id
                                        }
                                    }
                                });
                                let op_107v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_107v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_107v1__map__loc_unknown_start_0_0_end_0_0(op_107v1)
                                };
                                let op_93v1 = op_93v1.map(|a|((),a));
                                let op_107v1 = op_107v1.map(|b|((),b));
                                let mut sg_19v1_node_108v1_joindata_lhs_borrow = context.state_ref(sg_19v1_node_108v1_joindata_lhs).borrow_mut();
                                let mut sg_19v1_node_108v1_joindata_rhs_borrow = context.state_ref(sg_19v1_node_108v1_joindata_rhs).borrow_mut();
                                let op_108v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_93v1,op_107v1, &mut *sg_19v1_node_108v1_joindata_lhs_borrow, &mut *sg_19v1_node_108v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_108v1 = op_108v1.map(|((),(a,b))|(a,b));
                                let op_108v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_108v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_108v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_108v1)
                                };
                                let op_109v1 = op_108v1.map(|(id,data)|{
                                    (id,hydroflow_plus::runtime_support::bincode::serialize:: <hydroflow_plus_test::__staged::cluster::paxos::P1a>(&data).unwrap().into())
                                });
                                let op_109v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_109v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_109v1__map__loc_unknown_start_0_0_end_0_0(op_109v1)
                                };
                                let op_110v1 = hydroflow_plus::pusherator::for_each::ForEach::new(|item|{
                                    if let Err(err) = sg_19v1_node_110v1_item_send.send(item){
                                        {
                                            //$crate::panicking::panic_fmt($crate::const_format_args!("Failed to send async write item for processing.: {}",err));
                                        };
                                    }
                                });
                                let op_110v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_110v1__dest_sink__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_110v1__dest_sink__loc_unknown_start_0_0_end_0_0(op_110v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_19v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_19v1(op_109v1,op_110v1);
                                context.schedule_subgraph(context.current_subgraph(),false);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(23v1)",0,var_expr!(),var_expr!(hoff_160v1_send,hoff_187v1_send),false,move|context,var_args!(),var_args!(hoff_160v1_send,hoff_187v1_send)|{
                                let hoff_160v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_160v1_send.give(Some(v));
                                });
                                let hoff_187v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_187v1_send.give(Some(v));
                                });
                                let op_55v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_23v1_node_55v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_55v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_55v1__source_stream__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_55v1__source_stream__loc_unknown_start_0_0_end_0_0(op_55v1)
                                };
                                let op_56v1 = op_55v1.map(|res|{
                                    let(id,b) = res.unwrap();
                                    (id,hydroflow_plus::runtime_support::bincode::deserialize:: <hydroflow_plus_test::__staged::cluster::paxos::ClientPayload>(&b).unwrap())
                                });
                                let op_56v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_56v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_56v1__map__loc_unknown_start_0_0_end_0_0(op_56v1)
                                };
                                let op_57v1 = op_56v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(_,b)|b
                                });
                                let op_57v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_57v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_57v1__map__loc_unknown_start_0_0_end_0_0(op_57v1)
                                };
                                let op_127v1 = hydroflow_plus::pusherator::map::Map::new(|item|{
                                    let mut counter = context.state_ref(sg_23v1_node_127v1_counterdata).borrow_mut();
                                    (counter.next().unwrap(),item)
                                },hoff_187v1_send);
                                let op_127v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_127v1__enumerate__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_127v1__enumerate__loc_unknown_start_0_0_end_0_0(op_127v1)
                                };
                                let op_58v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_160v1_send,op_127v1);
                                let op_58v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_58v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_58v1__tee__loc_unknown_start_0_0_end_0_0(op_58v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_23v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_23v1(op_57v1,op_58v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(24v1)",2,var_expr!(hoff_181v1_recv,hoff_182v1_recv,hoff_183v1_recv,hoff_184v1_recv,hoff_185v1_recv,hoff_186v1_recv,hoff_187v1_recv,hoff_188v1_recv,hoff_189v1_recv,hoff_190v1_recv),var_expr!(),false,move|context,var_args!(hoff_181v1_recv,hoff_182v1_recv,hoff_183v1_recv,hoff_184v1_recv,hoff_185v1_recv,hoff_186v1_recv,hoff_187v1_recv,hoff_188v1_recv,hoff_189v1_recv,hoff_190v1_recv),var_args!()|{
                                let mut hoff_181v1_recv = hoff_181v1_recv.borrow_mut_swap();
                                let hoff_181v1_recv = hoff_181v1_recv.drain(..);
                                let mut hoff_182v1_recv = hoff_182v1_recv.borrow_mut_swap();
                                let hoff_182v1_recv = hoff_182v1_recv.drain(..);
                                let mut hoff_183v1_recv = hoff_183v1_recv.borrow_mut_swap();
                                let hoff_183v1_recv = hoff_183v1_recv.drain(..);
                                let mut hoff_184v1_recv = hoff_184v1_recv.borrow_mut_swap();
                                let hoff_184v1_recv = hoff_184v1_recv.drain(..);
                                let mut hoff_185v1_recv = hoff_185v1_recv.borrow_mut_swap();
                                let hoff_185v1_recv = hoff_185v1_recv.drain(..);
                                let mut hoff_186v1_recv = hoff_186v1_recv.borrow_mut_swap();
                                let hoff_186v1_recv = hoff_186v1_recv.drain(..);
                                let mut hoff_187v1_recv = hoff_187v1_recv.borrow_mut_swap();
                                let hoff_187v1_recv = hoff_187v1_recv.drain(..);
                                let mut hoff_188v1_recv = hoff_188v1_recv.borrow_mut_swap();
                                let hoff_188v1_recv = hoff_188v1_recv.drain(..);
                                let mut hoff_189v1_recv = hoff_189v1_recv.borrow_mut_swap();
                                let hoff_189v1_recv = hoff_189v1_recv.drain(..);
                                let mut hoff_190v1_recv = hoff_190v1_recv.borrow_mut_swap();
                                let hoff_190v1_recv = hoff_190v1_recv.drain(..);
                                let op_111v1 = sg_24v1_node_111v1_iter.by_ref();
                                let op_111v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_111v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_111v1__source_iter__loc_unknown_start_0_0_end_0_0(op_111v1)
                                };
                                let op_112v1 = op_111v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |d|d.clone()
                                });
                                let op_112v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_112v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_112v1__map__loc_unknown_start_0_0_end_0_0(op_112v1)
                                };
                                let hoff_181v1_recv = hoff_181v1_recv.map(|a: (i32, (_,_))|((),a));
                                let hoff_182v1_recv = hoff_182v1_recv.map(|b|((),b));
                                let mut sg_24v1_node_113v1_joindata_lhs_borrow = context.state_ref(sg_24v1_node_113v1_joindata_lhs).borrow_mut();
                                let mut sg_24v1_node_113v1_joindata_rhs_borrow = context.state_ref(sg_24v1_node_113v1_joindata_rhs).borrow_mut();
                                let op_113v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_181v1_recv,hoff_182v1_recv, &mut *sg_24v1_node_113v1_joindata_lhs_borrow, &mut *sg_24v1_node_113v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_113v1 = op_113v1.map(|((),(a,b))|(a,b));
                                let op_113v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_113v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_113v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_113v1)
                                };
                                let op_114v1 = op_113v1.filter_map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let f = f;
                                    let p_id = {
                                        use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                        let cli = cli;
                                        cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                    };
                                    move|((slot,(count,entry)),ballot_num)|if count<= *f {
                                        Some(P2a {
                                            ballot:Ballot {
                                                num:ballot_num,id:p_id
                                            },slot:slot,value:entry.value
                                        })
                                    }else {
                                        None
                                    }
                                });
                                let op_114v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_114v1__filter_map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_114v1__filter_map__loc_unknown_start_0_0_end_0_0(op_114v1)
                                };
                                let hoff_183v1_recv = hoff_183v1_recv.map(|k|(k,()));
                                let mut sg_24v1_node_117v1_antijoindata_neg_borrow = context.state_ref(sg_24v1_node_117v1_antijoindata_neg).borrow_mut();
                                #[allow(clippy::needless_borrow)]
                                (&mut *sg_24v1_node_117v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).extend(hoff_184v1_recv);
                                let op_117v1 = hoff_183v1_recv.filter(|x: &(_,_)|{
                                    #[allow(clippy::needless_borrow)]
                                    #[allow(clippy::unnecessary_mut_passed)]
                                    !(&mut *sg_24v1_node_117v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).contains(&x.0)
                                });
                                let op_117v1 = op_117v1.map(|(k,())|k);
                                let op_117v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_117v1__difference_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_117v1__difference_multiset__loc_unknown_start_0_0_end_0_0(op_117v1)
                                };
                                let op_117v1 = op_117v1.map(|a|((),a));
                                let hoff_185v1_recv = hoff_185v1_recv.map(|b|((),b));
                                let mut sg_24v1_node_118v1_joindata_lhs_borrow = context.state_ref(sg_24v1_node_118v1_joindata_lhs).borrow_mut();
                                let mut sg_24v1_node_118v1_joindata_rhs_borrow = context.state_ref(sg_24v1_node_118v1_joindata_rhs).borrow_mut();
                                let op_118v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_117v1,hoff_185v1_recv, &mut *sg_24v1_node_118v1_joindata_lhs_borrow, &mut *sg_24v1_node_118v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_118v1 = op_118v1.map(|((),(a,b))|(a,b));
                                let op_118v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_118v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_118v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_118v1)
                                };
                                let op_119v1 = op_118v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let p_id = {
                                        use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                        let cli = cli;
                                        cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                    };
                                    move|(slot,ballot_num)|P2a {
                                        ballot:Ballot {
                                            num:ballot_num,id:p_id
                                        },slot:slot,value:ClientPayload {
                                            key:0,value:"".to_string()
                                        }
                                    }
                                });
                                let op_119v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_119v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_119v1__map__loc_unknown_start_0_0_end_0_0(op_119v1)
                                };
                                let op_120v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(op_114v1,op_119v1)
                                };
                                let op_120v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_120v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_120v1__union__loc_unknown_start_0_0_end_0_0(op_120v1)
                                };
                                let op_121v1 = op_120v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |a|((),a)
                                });
                                let op_121v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_121v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_121v1__map__loc_unknown_start_0_0_end_0_0(op_121v1)
                                };
                                let mut sg_24v1_node_125v1_antijoindata_neg_borrow = context.state_ref(sg_24v1_node_125v1_antijoindata_neg).borrow_mut();
                                #[allow(clippy::needless_borrow)]
                                (&mut *sg_24v1_node_125v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).extend(hoff_186v1_recv);
                                let op_125v1 = op_121v1.filter(|x: &(_,_)|{
                                    #[allow(clippy::needless_borrow)]
                                    #[allow(clippy::unnecessary_mut_passed)]
                                    !(&mut *sg_24v1_node_125v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).contains(&x.0)
                                });
                                let op_125v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_125v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_125v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0(op_125v1)
                                };
                                let op_126v1 = op_125v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(_,a)|a
                                });
                                let op_126v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_126v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_126v1__map__loc_unknown_start_0_0_end_0_0(op_126v1)
                                };
                                let hoff_187v1_recv = hoff_187v1_recv.map(|a|((),a));
                                let hoff_188v1_recv = hoff_188v1_recv.map(|b|((),b));
                                let mut sg_24v1_node_128v1_joindata_lhs_borrow = context.state_ref(sg_24v1_node_128v1_joindata_lhs).borrow_mut();
                                let mut sg_24v1_node_128v1_joindata_rhs_borrow = context.state_ref(sg_24v1_node_128v1_joindata_rhs).borrow_mut();
                                let op_128v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_187v1_recv,hoff_188v1_recv, &mut *sg_24v1_node_128v1_joindata_lhs_borrow, &mut *sg_24v1_node_128v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_128v1 = op_128v1.map(|((),(a,b))|(a,b));
                                let op_128v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_128v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_128v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_128v1)
                                };
                                let op_128v1 = op_128v1.map(|a|((),a));
                                let hoff_189v1_recv = hoff_189v1_recv.map(|b|((),b));
                                let mut sg_24v1_node_129v1_joindata_lhs_borrow = context.state_ref(sg_24v1_node_129v1_joindata_lhs).borrow_mut();
                                let mut sg_24v1_node_129v1_joindata_rhs_borrow = context.state_ref(sg_24v1_node_129v1_joindata_rhs).borrow_mut();
                                let op_129v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_128v1,hoff_189v1_recv, &mut *sg_24v1_node_129v1_joindata_lhs_borrow, &mut *sg_24v1_node_129v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_129v1 = op_129v1.map(|((),(a,b))|(a,b));
                                let op_129v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_129v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_129v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_129v1)
                                };
                                let op_130v1 = op_129v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let p_id = {
                                        use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                        let cli = cli;
                                        cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                    };
                                    move|(((index,payload),next_slot),ballot_num):(((usize,ClientPayload),i32),u32)|P2a {
                                        ballot:Ballot {
                                            num:ballot_num,id:p_id
                                        },slot:next_slot+index as i32,value:payload
                                    }
                                });
                                let op_130v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_130v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_130v1__map__loc_unknown_start_0_0_end_0_0(op_130v1)
                                };
                                let op_131v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(op_126v1,op_130v1)
                                };
                                let op_131v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_131v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_131v1__union__loc_unknown_start_0_0_end_0_0(op_131v1)
                                };
                                let op_131v1 = op_131v1.map(|a|((),a));
                                let hoff_190v1_recv = hoff_190v1_recv.map(|b|((),b));
                                let mut sg_24v1_node_132v1_joindata_lhs_borrow = context.state_ref(sg_24v1_node_132v1_joindata_lhs).borrow_mut();
                                let mut sg_24v1_node_132v1_joindata_rhs_borrow = context.state_ref(sg_24v1_node_132v1_joindata_rhs).borrow_mut();
                                let op_132v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_131v1,hoff_190v1_recv, &mut *sg_24v1_node_132v1_joindata_lhs_borrow, &mut *sg_24v1_node_132v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_132v1 = op_132v1.map(|((),(a,b))|(a,b));
                                let op_132v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_132v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_132v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_132v1)
                                };
                                let op_133v1 = op_132v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(a,_)|a
                                });
                                let op_133v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_133v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_133v1__map__loc_unknown_start_0_0_end_0_0(op_133v1)
                                };
                                let op_112v1 = op_112v1.map(|a|((),a));
                                let op_133v1 = op_133v1.map(|b|((),b));
                                let mut sg_24v1_node_134v1_joindata_lhs_borrow = context.state_ref(sg_24v1_node_134v1_joindata_lhs).borrow_mut();
                                let mut sg_24v1_node_134v1_joindata_rhs_borrow = context.state_ref(sg_24v1_node_134v1_joindata_rhs).borrow_mut();
                                let op_134v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_112v1,op_133v1, &mut *sg_24v1_node_134v1_joindata_lhs_borrow, &mut *sg_24v1_node_134v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_134v1 = op_134v1.map(|((),(a,b))|(a,b));
                                let op_134v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_134v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_134v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_134v1)
                                };
                                let op_135v1 = op_134v1.map(|(id,data)|{
                                    (id,hydroflow_plus::runtime_support::bincode::serialize:: <hydroflow_plus_test::__staged::cluster::paxos::P2a>(&data).unwrap().into())
                                });
                                let op_135v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_135v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_135v1__map__loc_unknown_start_0_0_end_0_0(op_135v1)
                                };
                                let op_136v1 = hydroflow_plus::pusherator::for_each::ForEach::new(|item|{
                                    if let Err(err) = sg_24v1_node_136v1_item_send.send(item){
                                        {
                                            //$crate::panicking::panic_fmt($crate::const_format_args!("Failed to send async write item for processing.: {}",err));
                                        };
                                    }
                                });
                                let op_136v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_136v1__dest_sink__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_136v1__dest_sink__loc_unknown_start_0_0_end_0_0(op_136v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_24v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_24v1(op_135v1,op_136v1);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(26v1)",1,var_expr!(hoff_191v1_recv),var_expr!(),false,move|context,var_args!(hoff_191v1_recv),var_args!()|{
                                let mut hoff_191v1_recv = hoff_191v1_recv.borrow_mut_swap();
                                let hoff_191v1_recv = hoff_191v1_recv.drain(..);
                                let op_141v1 = sg_26v1_node_141v1_iter.by_ref();
                                let op_141v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_141v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_141v1__source_iter__loc_unknown_start_0_0_end_0_0(op_141v1)
                                };
                                let op_142v1 = op_141v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |d|d.clone()
                                });
                                let op_142v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_142v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_142v1__map__loc_unknown_start_0_0_end_0_0(op_142v1)
                                };
                                let op_142v1 = op_142v1.map(|a|((),a));
                                let hoff_191v1_recv = hoff_191v1_recv.map(|b|((),b));
                                let mut sg_26v1_node_144v1_joindata_lhs_borrow = context.state_ref(sg_26v1_node_144v1_joindata_lhs).borrow_mut();
                                let mut sg_26v1_node_144v1_joindata_rhs_borrow = context.state_ref(sg_26v1_node_144v1_joindata_rhs).borrow_mut();
                                let op_144v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_142v1,hoff_191v1_recv, &mut *sg_26v1_node_144v1_joindata_lhs_borrow, &mut *sg_26v1_node_144v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_144v1 = op_144v1.map(|((),(a,b))|(a,b));
                                let op_144v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_144v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_144v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_144v1)
                                };
                                let op_145v1 = op_144v1.map(|(id,data)|{
                                    (id,hydroflow_plus::runtime_support::bincode::serialize:: <hydroflow_plus_test::__staged::cluster::paxos::ReplicaPayload>(&data).unwrap().into())
                                });
                                let op_145v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_145v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_145v1__map__loc_unknown_start_0_0_end_0_0(op_145v1)
                                };
                                let op_146v1 = hydroflow_plus::pusherator::for_each::ForEach::new(|item|{
                                    if let Err(err) = sg_26v1_node_146v1_item_send.send(item){
                                        {
                                            //$crate::panicking::panic_fmt($crate::const_format_args!("Failed to send async write item for processing.: {}",err));
                                        };
                                    }
                                });
                                let op_146v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_146v1__dest_sink__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_146v1__dest_sink__loc_unknown_start_0_0_end_0_0(op_146v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_26v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_26v1(op_145v1,op_146v1);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(27v1)",0,var_expr!(),var_expr!(hoff_192v1_send),false,move|context,var_args!(),var_args!(hoff_192v1_send)|{
                                let hoff_192v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_192v1_send.give(Some(v));
                                });
                                let op_10v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_27v1_node_10v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_10v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_10v1__source_interval__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_10v1__source_interval__loc_unknown_start_0_0_end_0_0(op_10v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_27v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_27v1(op_10v1,hoff_192v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(28v1)",0,var_expr!(),var_expr!(hoff_193v1_send),false,move|context,var_args!(),var_args!(hoff_193v1_send)|{
                                let hoff_193v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_193v1_send.give(Some(v));
                                });
                                let op_95v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_28v1_node_95v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_95v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_95v1__source_interval__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_95v1__source_interval__loc_unknown_start_0_0_end_0_0(op_95v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_28v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_28v1(op_95v1,hoff_193v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(7v1)",0,var_expr!(hoff_46v3_recv,hoff_147v1_recv,hoff_148v1_recv),var_expr!(hoff_149v1_send),false,move|context,var_args!(hoff_46v3_recv,hoff_147v1_recv,hoff_148v1_recv),var_args!(hoff_149v1_send)|{
                                let mut hoff_46v3_recv = hoff_46v3_recv.borrow_mut_swap();
                                let hoff_46v3_recv = hoff_46v3_recv.drain(..);
                                let mut hoff_147v1_recv = hoff_147v1_recv.borrow_mut_swap();
                                let hoff_147v1_recv = hoff_147v1_recv.drain(..);
                                let mut hoff_148v1_recv = hoff_148v1_recv.borrow_mut_swap();
                                let hoff_148v1_recv = hoff_148v1_recv.drain(..);
                                let hoff_149v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_149v1_send.give(Some(v));
                                });
                                let op_26v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(hoff_46v3_recv,hoff_147v1_recv)
                                };
                                let op_26v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_26v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_26v1__union__loc_unknown_start_0_0_end_0_0(op_26v1)
                                };
                                let op_28v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(op_26v1,hoff_148v1_recv)
                                };
                                let op_28v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_28v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_28v1__union__loc_unknown_start_0_0_end_0_0(op_28v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_7v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_7v1(op_28v1,hoff_149v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(8v1)",1,var_expr!(hoff_149v1_recv),var_expr!(hoff_150v1_send,hoff_156v1_send),false,move|context,var_args!(hoff_149v1_recv),var_args!(hoff_150v1_send,hoff_156v1_send)|{
                                let mut hoff_149v1_recv = hoff_149v1_recv.borrow_mut_swap();
                                let hoff_149v1_recv = hoff_149v1_recv.drain(..);
                                let hoff_150v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_150v1_send.give(Some(v));
                                });
                                let hoff_156v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_156v1_send.give(Some(v));
                                });
                                let op_29v1 = {
                                    let mut sg_8v1_node_29v1_accumulator = context.state_ref(singleton_op_29v1).borrow_mut();
                                    hoff_149v1_recv.for_each(|sg_8v1_node_29v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_8v1_node_29v1_accumulator,sg_8v1_node_29v1_iterator_item,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |curr_max_ballot,new_ballot|{
                                                if new_ballot> *curr_max_ballot {
                                                    *curr_max_ballot = new_ballot;
                                                }
                                            }
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_8v1_node_29v1_accumulator))
                                    }
                                };
                                let op_29v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_29v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_29v1__fold__loc_unknown_start_0_0_end_0_0(op_29v1)
                                };
                                let op_30v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_150v1_send,hoff_156v1_send);
                                let op_30v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_30v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_30v1__tee__loc_unknown_start_0_0_end_0_0(op_30v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_8v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_8v1(op_29v1,op_30v1);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(9v1)",1,var_expr!(hoff_150v1_recv,hoff_151v1_recv),var_expr!(hoff_152v1_send),false,move|context,var_args!(hoff_150v1_recv,hoff_151v1_recv),var_args!(hoff_152v1_send)|{
                                let mut hoff_150v1_recv = hoff_150v1_recv.borrow_mut_swap();
                                let hoff_150v1_recv = hoff_150v1_recv.drain(..);
                                let mut hoff_151v1_recv = hoff_151v1_recv.borrow_mut_swap();
                                let hoff_151v1_recv = hoff_151v1_recv.drain(..);
                                let hoff_152v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_152v1_send.give(Some(v));
                                });
                                let hoff_150v1_recv = hoff_150v1_recv.map(|a|((),a));
                                let hoff_151v1_recv = hoff_151v1_recv.map(|b|((),b));
                                let mut sg_9v1_node_31v1_joindata_lhs_borrow = context.state_ref(sg_9v1_node_31v1_joindata_lhs).borrow_mut();
                                let mut sg_9v1_node_31v1_joindata_rhs_borrow = context.state_ref(sg_9v1_node_31v1_joindata_rhs).borrow_mut();
                                let op_31v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_150v1_recv,hoff_151v1_recv, &mut *sg_9v1_node_31v1_joindata_lhs_borrow, &mut *sg_9v1_node_31v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_31v1 = op_31v1.map(|((),(a,b))|(a,b));
                                let op_31v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_31v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_31v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_31v1)
                                };
                                let op_32v1 = op_31v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let p_id = {
                                        use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                        let cli = cli;
                                        cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                    };
                                    move|(received_max_ballot,ballot_num)|{
                                        if received_max_ballot>(Ballot {
                                            num:ballot_num,id:p_id
                                        }){
                                            received_max_ballot.num+1
                                        }else {
                                            ballot_num
                                        }
                                    }
                                });
                                let op_32v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_32v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_32v1__map__loc_unknown_start_0_0_end_0_0(op_32v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_9v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_9v1(op_32v1,hoff_152v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(10v1)",0,var_expr!(hoff_153v1_recv,hoff_154v1_recv),var_expr!(hoff_155v1_send,hoff_158v1_send),false,move|context,var_args!(hoff_153v1_recv,hoff_154v1_recv),var_args!(hoff_155v1_send,hoff_158v1_send)|{
                                let mut hoff_153v1_recv = hoff_153v1_recv.borrow_mut_swap();
                                let hoff_153v1_recv = hoff_153v1_recv.drain(..);
                                let mut hoff_154v1_recv = hoff_154v1_recv.borrow_mut_swap();
                                let hoff_154v1_recv = hoff_154v1_recv.drain(..);
                                let hoff_155v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_155v1_send.give(Some(v));
                                });
                                let hoff_158v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_158v1_send.give(Some(v));
                                });
                                let hoff_153v1_recv = hoff_153v1_recv.map(|a|((),a));
                                let hoff_154v1_recv = hoff_154v1_recv.map(|b|((),b));
                                let mut sg_10v1_node_35v1_joindata_lhs_borrow = context.state_ref(sg_10v1_node_35v1_joindata_lhs).borrow_mut();
                                let mut sg_10v1_node_35v1_joindata_rhs_borrow = context.state_ref(sg_10v1_node_35v1_joindata_rhs).borrow_mut();
                                let op_35v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_153v1_recv,hoff_154v1_recv, &mut *sg_10v1_node_35v1_joindata_lhs_borrow, &mut *sg_10v1_node_35v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_35v1 = op_35v1.map(|((),(a,b))|(a,b));
                                let op_35v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_35v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_35v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_35v1)
                                };
                                let op_36v1 = op_35v1.filter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let p_id = {
                                        use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                        let cli = cli;
                                        cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                    };
                                    move|((sender,p1b),ballot_num)|p1b.ballot==Ballot {
                                        num: *ballot_num,id:p_id
                                    }
                                });
                                let op_36v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_36v1__filter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_36v1__filter__loc_unknown_start_0_0_end_0_0(op_36v1)
                                };
                                let op_49v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |((_,p1b),_)|p1b.accepted.into_iter()
                                },hydroflow_plus::pusherator::flatten::Flatten::new(hoff_158v1_send));
                                let op_49v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_49v1__flat_map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_49v1__flat_map__loc_unknown_start_0_0_end_0_0(op_49v1)
                                };
                                let op_39v1 = hydroflow_plus::pusherator::filter::Filter::new(|item|{
                                    let mut borrow = context.state_ref(sg_10v1_node_39v1_uniquedata).borrow_mut();
                                    let set = borrow.get_mut_clear((context.current_tick(),context.current_stratum()));
                                    if!set.contains(item){
                                        set.insert(::std::clone::Clone::clone(item));
                                        true
                                    }else {
                                        false
                                    }
                                },hoff_155v1_send);
                                let op_39v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_39v1__unique__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_39v1__unique__loc_unknown_start_0_0_end_0_0(op_39v1)
                                };
                                let op_38v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |((sender,p1b),ballot_num)|sender
                                },op_39v1);
                                let op_38v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_38v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_38v1__map__loc_unknown_start_0_0_end_0_0(op_38v1)
                                };
                                let op_37v1 = hydroflow_plus::pusherator::tee::Tee::new(op_38v1,op_49v1);
                                let op_37v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_37v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_37v1__tee__loc_unknown_start_0_0_end_0_0(op_37v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_10v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_10v1(op_36v1,op_37v1);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(11v1)",1,var_expr!(hoff_160v1_recv),var_expr!(hoff_161v1_send,hoff_165v1_send),false,move|context,var_args!(hoff_160v1_recv),var_args!(hoff_161v1_send,hoff_165v1_send)|{
                                let mut hoff_160v1_recv = hoff_160v1_recv.borrow_mut_swap();
                                let hoff_160v1_recv = hoff_160v1_recv.drain(..);
                                let hoff_161v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_161v1_send.give(Some(v));
                                });
                                let hoff_165v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_165v1_send.give(Some(v));
                                });
                                let op_59v1 = {
                                    let mut sg_11v1_node_59v1_accumulator = context.state_ref(singleton_op_59v1).borrow_mut();
                                    hoff_160v1_recv.for_each(|sg_11v1_node_59v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_11v1_node_59v1_accumulator,sg_11v1_node_59v1_iterator_item,{
                                            use hydroflow_plus::__staged::stream:: * ;
                                            |count,_| *count+=1
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_11v1_node_59v1_accumulator))
                                    }
                                };
                                let op_59v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_59v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_59v1__fold__loc_unknown_start_0_0_end_0_0(op_59v1)
                                };
                                let op_68v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |_|()
                                },hoff_165v1_send);
                                let op_68v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_68v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_68v1__map__loc_unknown_start_0_0_end_0_0(op_68v1)
                                };
                                let op_67v1 = hydroflow_plus::pusherator::filter::Filter::new({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |b| *b
                                },op_68v1);
                                let op_67v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_67v1__filter__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_67v1__filter__loc_unknown_start_0_0_end_0_0(op_67v1)
                                };
                                let op_66v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |num_payloads|num_payloads>0
                                },op_67v1);
                                let op_66v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_66v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_66v1__map__loc_unknown_start_0_0_end_0_0(op_66v1)
                                };
                                let op_60v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_161v1_send,op_66v1);
                                let op_60v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_60v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_60v1__tee__loc_unknown_start_0_0_end_0_0(op_60v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_11v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_11v1(op_59v1,op_60v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(12v1)",2,var_expr!(hoff_161v1_recv,hoff_162v1_recv,hoff_163v1_recv,hoff_164v1_recv,hoff_165v1_recv,hoff_166v1_recv),var_expr!(hoff_167v1_send),false,move|context,var_args!(hoff_161v1_recv,hoff_162v1_recv,hoff_163v1_recv,hoff_164v1_recv,hoff_165v1_recv,hoff_166v1_recv),var_args!(hoff_167v1_send)|{
                                let mut hoff_161v1_recv = hoff_161v1_recv.borrow_mut_swap();
                                let hoff_161v1_recv = hoff_161v1_recv.drain(..);
                                let mut hoff_162v1_recv = hoff_162v1_recv.borrow_mut_swap();
                                let hoff_162v1_recv = hoff_162v1_recv.drain(..);
                                let mut hoff_163v1_recv = hoff_163v1_recv.borrow_mut_swap();
                                let hoff_163v1_recv = hoff_163v1_recv.drain(..);
                                let mut hoff_164v1_recv = hoff_164v1_recv.borrow_mut_swap();
                                let hoff_164v1_recv = hoff_164v1_recv.drain(..);
                                let mut hoff_165v1_recv = hoff_165v1_recv.borrow_mut_swap();
                                let hoff_165v1_recv = hoff_165v1_recv.drain(..);
                                let mut hoff_166v1_recv = hoff_166v1_recv.borrow_mut_swap();
                                let hoff_166v1_recv = hoff_166v1_recv.drain(..);
                                let hoff_167v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_167v1_send.give(Some(v));
                                });
                                let hoff_161v1_recv = hoff_161v1_recv.map(|a|((),a));
                                let hoff_162v1_recv = hoff_162v1_recv.map(|b|((),b));
                                let mut sg_12v1_node_62v1_joindata_lhs_borrow = context.state_ref(sg_12v1_node_62v1_joindata_lhs).borrow_mut();
                                let mut sg_12v1_node_62v1_joindata_rhs_borrow = context.state_ref(sg_12v1_node_62v1_joindata_rhs).borrow_mut();
                                let op_62v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_161v1_recv,hoff_162v1_recv, &mut *sg_12v1_node_62v1_joindata_lhs_borrow, &mut *sg_12v1_node_62v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_62v1 = op_62v1.map(|((),(a,b))|(a,b));
                                let op_62v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_62v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_62v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_62v1)
                                };
                                let op_63v1 = op_62v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(num_payloads,next_slot):(usize,i32)|next_slot+num_payloads as i32
                                });
                                let op_63v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_63v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_63v1__map__loc_unknown_start_0_0_end_0_0(op_63v1)
                                };
                                let op_64v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(hoff_163v1_recv,op_63v1)
                                };
                                let op_64v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_64v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_64v1__union__loc_unknown_start_0_0_end_0_0(op_64v1)
                                };
                                let mut sg_12v1_node_69v1_antijoindata_neg_borrow = context.state_ref(sg_12v1_node_69v1_antijoindata_neg).borrow_mut();
                                #[allow(clippy::needless_borrow)]
                                (&mut *sg_12v1_node_69v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).extend(hoff_165v1_recv);
                                let op_69v1 = hoff_164v1_recv.filter(|x: &(_,_)|{
                                    #[allow(clippy::needless_borrow)]
                                    #[allow(clippy::unnecessary_mut_passed)]
                                    !(&mut *sg_12v1_node_69v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).contains(&x.0)
                                });
                                let op_69v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_69v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_69v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0(op_69v1)
                                };
                                let op_70v1 = op_69v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(_,a)|a
                                });
                                let op_70v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_70v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_70v1__map__loc_unknown_start_0_0_end_0_0(op_70v1)
                                };
                                let op_71v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(op_64v1,op_70v1)
                                };
                                let op_71v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_71v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_71v1__union__loc_unknown_start_0_0_end_0_0(op_71v1)
                                };
                                let op_71v1 = op_71v1.map(|a|((),a));
                                let hoff_166v1_recv = hoff_166v1_recv.map(|b|((),b));
                                let mut sg_12v1_node_72v1_joindata_lhs_borrow = context.state_ref(sg_12v1_node_72v1_joindata_lhs).borrow_mut();
                                let mut sg_12v1_node_72v1_joindata_rhs_borrow = context.state_ref(sg_12v1_node_72v1_joindata_rhs).borrow_mut();
                                let op_72v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_71v1,hoff_166v1_recv, &mut *sg_12v1_node_72v1_joindata_lhs_borrow, &mut *sg_12v1_node_72v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_72v1 = op_72v1.map(|((),(a,b))|(a,b));
                                let op_72v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_72v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_72v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_72v1)
                                };
                                let op_73v1 = op_72v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(a,_)|a
                                });
                                let op_73v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_73v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_73v1__map__loc_unknown_start_0_0_end_0_0(op_73v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_12v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_12v1(op_73v1,hoff_167v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(13v1)",1,var_expr!(hoff_169v1_recv),var_expr!(hoff_170v1_send,hoff_173v1_send,hoff_176v1_send),false,move|context,var_args!(hoff_169v1_recv),var_args!(hoff_170v1_send,hoff_173v1_send,hoff_176v1_send)|{
                                let mut hoff_169v1_recv = hoff_169v1_recv.borrow_mut_swap();
                                let hoff_169v1_recv = hoff_169v1_recv.drain(..);
                                let hoff_170v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_170v1_send.give(Some(v));
                                });
                                let hoff_173v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_173v1_send.give(Some(v));
                                });
                                let hoff_176v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_176v1_send.give(Some(v));
                                });
                                let mut sg_13v1_node_78v1_hashtable = context.state_ref(sg_13v1_node_78v1_groupbydata).borrow_mut();
                                {
                                    #[inline(always)]
                                    fn check_input<Iter,A,B>(iter:Iter) -> impl ::std::iter::Iterator<Item = (A,B)>where Iter:std::iter::Iterator<Item = (A,B)> ,A: ::std::clone::Clone,B: ::std::clone::Clone {
                                        iter
                                    }
                                    #[doc = r" A: accumulator type"]
                                    #[doc = r" T: iterator item type"]
                                    #[doc = r" O: output type"]
                                    #[inline(always)]
                                    fn call_comb_type<A,T,O>(a: &mut A,t:T,f:impl Fn(&mut A,T) -> O) -> O {
                                        (f)(a,t)
                                    }
                                    for kv in check_input(hoff_169v1_recv){
                                        #[allow(unknown_lints,clippy::unwrap_or_default)]
                                        let entry = sg_13v1_node_78v1_hashtable.entry(kv.0).or_insert_with({
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            | |(0,P2b {
                                                ballot:Ballot {
                                                    num:0,id:0
                                                },max_ballot:Ballot {
                                                    num:0,id:0
                                                },slot:0,value:ClientPayload {
                                                    key:0,value:"".to_string()
                                                }
                                            })
                                        });
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(entry,kv.1,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |accum,(sender,p2b)|{
                                                accum.0+=1;
                                                accum.1 = p2b;
                                            }
                                        });
                                    }
                                }let op_78v1 = sg_13v1_node_78v1_hashtable.drain();
                                let op_78v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_78v1__fold_keyed__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_78v1__fold_keyed__loc_unknown_start_0_0_end_0_0(op_78v1)
                                };
                                let op_85v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_173v1_send,hoff_176v1_send);
                                let op_85v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_85v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_85v1__tee__loc_unknown_start_0_0_end_0_0(op_85v1)
                                };
                                let op_84v1 = hydroflow_plus::pusherator::filter_map::FilterMap::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let f = f;
                                    |(slot,(count,p2b))|if count==2* *f+1 {
                                        Some(slot)
                                    }else {
                                        None
                                    }
                                },op_85v1);
                                let op_84v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_84v1__filter_map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_84v1__filter_map__loc_unknown_start_0_0_end_0_0(op_84v1)
                                };
                                let op_79v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_170v1_send,op_84v1);
                                let op_79v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_79v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_79v1__tee__loc_unknown_start_0_0_end_0_0(op_79v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_13v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_13v1(op_78v1,op_79v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(14v1)",2,var_expr!(hoff_172v1_recv,hoff_173v1_recv),var_expr!(hoff_174v1_send),false,move|context,var_args!(hoff_172v1_recv,hoff_173v1_recv),var_args!(hoff_174v1_send)|{
                                let mut hoff_172v1_recv = hoff_172v1_recv.borrow_mut_swap();
                                let hoff_172v1_recv = hoff_172v1_recv.drain(..);
                                let mut hoff_173v1_recv = hoff_173v1_recv.borrow_mut_swap();
                                let hoff_173v1_recv = hoff_173v1_recv.drain(..);
                                let hoff_174v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_174v1_send.give(Some(v));
                                });
                                let hoff_172v1_recv = hoff_172v1_recv.map(|k|(k,()));
                                let mut sg_14v1_node_86v1_antijoindata_neg_borrow = context.state_ref(sg_14v1_node_86v1_antijoindata_neg).borrow_mut();
                                #[allow(clippy::needless_borrow)]
                                (&mut *sg_14v1_node_86v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).extend(hoff_173v1_recv);
                                let op_86v1 = hoff_172v1_recv.filter(|x: &(_,_)|{
                                    #[allow(clippy::needless_borrow)]
                                    #[allow(clippy::unnecessary_mut_passed)]
                                    !(&mut *sg_14v1_node_86v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).contains(&x.0)
                                });
                                let op_86v1 = op_86v1.map(|(k,())|k);
                                let op_86v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_86v1__difference_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_86v1__difference_multiset__loc_unknown_start_0_0_end_0_0(op_86v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_14v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_14v1(op_86v1,hoff_174v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(15v1)",0,var_expr!(hoff_174v1_recv),var_expr!(hoff_171v1_send),false,move|context,var_args!(hoff_174v1_recv),var_args!(hoff_171v1_send)|{
                                let mut hoff_174v1_recv = hoff_174v1_recv.borrow_mut_swap();
                                let hoff_174v1_recv = hoff_174v1_recv.drain(..);
                                let hoff_171v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_171v1_send.give(Some(v));
                                });
                                let op_87v1 = {
                                    fn check_input<Iter: ::std::iter::Iterator<Item = Item> ,Item>(iter:Iter) -> impl ::std::iter::Iterator<Item = Item>{
                                        iter
                                    }
                                    check_input:: <_,_>(hoff_174v1_recv)
                                };
                                let op_87v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_87v1__defer_tick__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_87v1__defer_tick__loc_unknown_start_0_0_end_0_0(op_87v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_15v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_15v1(op_87v1,hoff_171v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(16v1)",0,var_expr!(hoff_168v1_recv,hoff_177v1_recv),var_expr!(hoff_169v1_send,hoff_175v1_send),false,move|context,var_args!(hoff_168v1_recv,hoff_177v1_recv),var_args!(hoff_169v1_send,hoff_175v1_send)|{
                                let mut hoff_168v1_recv = hoff_168v1_recv.borrow_mut_swap();
                                let hoff_168v1_recv = hoff_168v1_recv.drain(..);
                                let mut hoff_177v1_recv = hoff_177v1_recv.borrow_mut_swap();
                                let hoff_177v1_recv = hoff_177v1_recv.drain(..);
                                let hoff_169v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_169v1_send.give(Some(v));
                                });
                                let hoff_175v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_175v1_send.give(Some(v));
                                });
                                let op_91v1 = {
                                    fn check_input<Iter: ::std::iter::Iterator<Item = Item> ,Item>(iter:Iter) -> impl ::std::iter::Iterator<Item = Item>{
                                        iter
                                    }
                                    check_input:: <_,_>(hoff_177v1_recv)
                                };
                                let op_91v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_91v1__defer_tick__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_91v1__defer_tick__loc_unknown_start_0_0_end_0_0(op_91v1)
                                };
                                let op_75v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(hoff_168v1_recv,op_91v1)
                                };
                                let op_75v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_75v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_75v1__union__loc_unknown_start_0_0_end_0_0(op_75v1)
                                };
                                let op_88v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(sender,p2b)|(p2b.slot,(sender,p2b))
                                },hoff_175v1_send);
                                let op_88v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_88v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_88v1__map__loc_unknown_start_0_0_end_0_0(op_88v1)
                                };
                                let op_77v1 = hydroflow_plus::pusherator::filter_map::FilterMap::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(sender,p2b)|if p2b.ballot==p2b.max_ballot {
                                        Some((p2b.slot,(sender,p2b)))
                                    }else {
                                        None
                                    }
                                },hoff_169v1_send);
                                let op_77v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_77v1__filter_map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_77v1__filter_map__loc_unknown_start_0_0_end_0_0(op_77v1)
                                };
                                let op_76v1 = hydroflow_plus::pusherator::tee::Tee::new(op_77v1,op_88v1);
                                let op_76v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_76v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_76v1__tee__loc_unknown_start_0_0_end_0_0(op_76v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_16v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_16v1(op_75v1,op_76v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(17v1)",2,var_expr!(hoff_175v1_recv,hoff_176v1_recv),var_expr!(hoff_177v1_send),false,move|context,var_args!(hoff_175v1_recv,hoff_176v1_recv),var_args!(hoff_177v1_send)|{
                                let mut hoff_175v1_recv = hoff_175v1_recv.borrow_mut_swap();
                                let hoff_175v1_recv = hoff_175v1_recv.drain(..);
                                let mut hoff_176v1_recv = hoff_176v1_recv.borrow_mut_swap();
                                let hoff_176v1_recv = hoff_176v1_recv.drain(..);
                                let hoff_177v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_177v1_send.give(Some(v));
                                });
                                let mut sg_17v1_node_89v1_antijoindata_neg_borrow = context.state_ref(sg_17v1_node_89v1_antijoindata_neg).borrow_mut();
                                #[allow(clippy::needless_borrow)]
                                (&mut *sg_17v1_node_89v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).extend(hoff_176v1_recv);
                                let op_89v1 = hoff_175v1_recv.filter(|x: &(_,_)|{
                                    #[allow(clippy::needless_borrow)]
                                    #[allow(clippy::unnecessary_mut_passed)]
                                    !(&mut *sg_17v1_node_89v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).contains(&x.0)
                                });
                                let op_89v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_89v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_89v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0(op_89v1)
                                };
                                let op_90v1 = op_89v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(slot,(sender,p2b))|(sender,p2b)
                                });
                                let op_90v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_90v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_90v1__map__loc_unknown_start_0_0_end_0_0(op_90v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_17v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_17v1(op_90v1,hoff_177v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(18v1)",1,var_expr!(hoff_155v1_recv,hoff_156v1_recv,hoff_157v1_recv),var_expr!(hoff_104v3_send,hoff_166v1_send,hoff_179v1_send,hoff_190v1_send),false,move|context,var_args!(hoff_155v1_recv,hoff_156v1_recv,hoff_157v1_recv),var_args!(hoff_104v3_send,hoff_166v1_send,hoff_179v1_send,hoff_190v1_send)|{
                                let mut hoff_155v1_recv = hoff_155v1_recv.borrow_mut_swap();
                                let hoff_155v1_recv = hoff_155v1_recv.drain(..);
                                let mut hoff_156v1_recv = hoff_156v1_recv.borrow_mut_swap();
                                let hoff_156v1_recv = hoff_156v1_recv.drain(..);
                                let mut hoff_157v1_recv = hoff_157v1_recv.borrow_mut_swap();
                                let hoff_157v1_recv = hoff_157v1_recv.drain(..);
                                let hoff_104v3_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_104v3_send.give(Some(v));
                                });
                                let hoff_166v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_166v1_send.give(Some(v));
                                });
                                let hoff_179v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_179v1_send.give(Some(v));
                                });
                                let hoff_190v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_190v1_send.give(Some(v));
                                });
                                let op_40v1 = {
                                    let mut sg_18v1_node_40v1_accumulator = context.state_ref(singleton_op_40v1).borrow_mut();
                                    hoff_155v1_recv.for_each(|sg_18v1_node_40v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_18v1_node_40v1_accumulator,sg_18v1_node_40v1_iterator_item,{
                                            use hydroflow_plus::__staged::stream:: * ;
                                            |count,_| *count+=1
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_18v1_node_40v1_accumulator))
                                    }
                                };
                                let op_40v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_40v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_40v1__fold__loc_unknown_start_0_0_end_0_0(op_40v1)
                                };
                                let op_41v1 = op_40v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let f = f;
                                    |num_received|num_received> *f
                                });
                                let op_41v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_41v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_41v1__map__loc_unknown_start_0_0_end_0_0(op_41v1)
                                };
                                let op_42v1 = op_41v1.filter({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |b| *b
                                });
                                let op_42v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_42v1__filter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_42v1__filter__loc_unknown_start_0_0_end_0_0(op_42v1)
                                };
                                let hoff_156v1_recv = hoff_156v1_recv.map(|a|((),a));
                                let hoff_157v1_recv = hoff_157v1_recv.map(|b|((),b));
                                let mut sg_18v1_node_43v1_joindata_lhs_borrow = context.state_ref(sg_18v1_node_43v1_joindata_lhs).borrow_mut();
                                let mut sg_18v1_node_43v1_joindata_rhs_borrow = context.state_ref(sg_18v1_node_43v1_joindata_rhs).borrow_mut();
                                let op_43v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_156v1_recv,hoff_157v1_recv, &mut *sg_18v1_node_43v1_joindata_lhs_borrow, &mut *sg_18v1_node_43v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_43v1 = op_43v1.map(|((),(a,b))|(a,b));
                                let op_43v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_43v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_43v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_43v1)
                                };
                                let op_44v1 = op_43v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let p_id = {
                                        use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                        let cli = cli;
                                        cli.meta.cluster_id.expect("Tried to read Cluster ID on a non-cluster node")
                                    };
                                    move|(received_max_ballot,ballot_num)|received_max_ballot<=Ballot {
                                        num:ballot_num,id:p_id
                                    }
                                });
                                let op_44v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_44v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_44v1__map__loc_unknown_start_0_0_end_0_0(op_44v1)
                                };
                                let op_45v1 = op_44v1.filter({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |b| *b
                                });
                                let op_45v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_45v1__filter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_45v1__filter__loc_unknown_start_0_0_end_0_0(op_45v1)
                                };
                                let op_42v1 = op_42v1.map(|a|((),a));
                                let op_45v1 = op_45v1.map(|b|((),b));
                                let mut sg_18v1_node_47v1_joindata_lhs_borrow = context.state_ref(sg_18v1_node_47v1_joindata_lhs).borrow_mut();
                                let mut sg_18v1_node_47v1_joindata_rhs_borrow = context.state_ref(sg_18v1_node_47v1_joindata_rhs).borrow_mut();
                                let op_47v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_42v1,op_45v1, &mut *sg_18v1_node_47v1_joindata_lhs_borrow, &mut *sg_18v1_node_47v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_47v1 = op_47v1.map(|((),(a,b))|(a,b));
                                let op_47v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_47v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_47v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_47v1)
                                };
                                let op_48v1 = op_47v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(a,_)|a
                                });
                                let op_48v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_48v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_48v1__map__loc_unknown_start_0_0_end_0_0(op_48v1)
                                };
                                let op_99v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |_|()
                                },hoff_179v1_send);
                                let op_99v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_99v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_99v1__map__loc_unknown_start_0_0_end_0_0(op_99v1)
                                };
                                let op_13v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_104v3_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_166v1_send,hydroflow_plus::pusherator::tee::Tee::new(op_99v1,hoff_190v1_send)));
                                let op_13v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_13v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_13v1__tee__loc_unknown_start_0_0_end_0_0(op_13v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_18v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_18v1(op_48v1,op_13v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(20v1)",2,var_expr!(hoff_159v1_recv),var_expr!(hoff_163v1_send,hoff_183v1_send),false,move|context,var_args!(hoff_159v1_recv),var_args!(hoff_163v1_send,hoff_183v1_send)|{
                                let mut hoff_159v1_recv = hoff_159v1_recv.borrow_mut_swap();
                                let hoff_159v1_recv = hoff_159v1_recv.drain(..);
                                let hoff_163v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_163v1_send.give(Some(v));
                                });
                                let hoff_183v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_183v1_send.give(Some(v));
                                });
                                let op_52v1 = {
                                    let mut sg_20v1_node_52v1_accumulator = context.state_ref(singleton_op_52v1).borrow_mut();
                                    hoff_159v1_recv.for_each(|sg_20v1_node_52v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_20v1_node_52v1_accumulator,sg_20v1_node_52v1_iterator_item,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |max_slot,(slot,(count,entry))|{
                                                if slot> *max_slot {
                                                    *max_slot = slot;
                                                }
                                            }
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_20v1_node_52v1_accumulator))
                                    }
                                };
                                let op_52v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_52v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_52v1__fold__loc_unknown_start_0_0_end_0_0(op_52v1)
                                };
                                let op_115v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |max_slot|0..max_slot
                                },hydroflow_plus::pusherator::flatten::Flatten::new(hoff_183v1_send));
                                let op_115v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_115v1__flat_map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_115v1__flat_map__loc_unknown_start_0_0_end_0_0(op_115v1)
                                };
                                let op_54v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |max_slot|max_slot+1
                                },hoff_163v1_send);
                                let op_54v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_54v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_54v1__map__loc_unknown_start_0_0_end_0_0(op_54v1)
                                };
                                let op_53v1 = hydroflow_plus::pusherator::tee::Tee::new(op_54v1,op_115v1);
                                let op_53v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_53v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_53v1__tee__loc_unknown_start_0_0_end_0_0(op_53v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_20v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_20v1(op_52v1,op_53v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(21v1)",1,var_expr!(hoff_158v1_recv),var_expr!(hoff_159v1_send,hoff_181v1_send,hoff_184v1_send),false,move|context,var_args!(hoff_158v1_recv),var_args!(hoff_159v1_send,hoff_181v1_send,hoff_184v1_send)|{
                                let mut hoff_158v1_recv = hoff_158v1_recv.borrow_mut_swap();
                                let hoff_158v1_recv = hoff_158v1_recv.drain(..);
                                let hoff_159v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_159v1_send.give(Some(v));
                                });
                                let hoff_181v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_181v1_send.give(Some(v));
                                });
                                let hoff_184v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_184v1_send.give(Some(v));
                                });
                                let mut sg_21v1_node_50v1_hashtable = context.state_ref(sg_21v1_node_50v1_groupbydata).borrow_mut();
                                {
                                    #[inline(always)]
                                    fn check_input<Iter,A,B>(iter:Iter) -> impl ::std::iter::Iterator<Item = (A,B)>where Iter:std::iter::Iterator<Item = (A,B)> ,A: ::std::clone::Clone,B: ::std::clone::Clone {
                                        iter
                                    }
                                    #[doc = r" A: accumulator type"]
                                    #[doc = r" T: iterator item type"]
                                    #[doc = r" O: output type"]
                                    #[inline(always)]
                                    fn call_comb_type<A,T,O>(a: &mut A,t:T,f:impl Fn(&mut A,T) -> O) -> O {
                                        (f)(a,t)
                                    }
                                    for kv in check_input(hoff_158v1_recv){
                                        #[allow(unknown_lints,clippy::unwrap_or_default)]
                                        let entry = sg_21v1_node_50v1_hashtable.entry(kv.0).or_insert_with({
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            | |(0,LogValue {
                                                ballot:Ballot {
                                                    num:0,id:0
                                                },value:ClientPayload {
                                                    key:0,value:"".to_string()
                                                }
                                            })
                                        });
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(entry,kv.1,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |curr_entry,new_entry|{
                                                let same_values = new_entry.value==curr_entry.1.value;
                                                let higher_ballot = new_entry.ballot>curr_entry.1.ballot;
                                                if same_values {
                                                    curr_entry.0+=1;
                                                }if higher_ballot {
                                                    curr_entry.1.ballot = new_entry.ballot;
                                                    if!same_values {
                                                        curr_entry.0 = 1;
                                                        curr_entry.1.value = new_entry.value;
                                                    }
                                                }
                                            }
                                        });
                                    }
                                }let op_50v1 = sg_21v1_node_50v1_hashtable.drain();
                                let op_50v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_50v1__fold_keyed__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_50v1__fold_keyed__loc_unknown_start_0_0_end_0_0(op_50v1)
                                };
                                let op_116v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(slot,_)|slot
                                },hoff_184v1_send);
                                let op_116v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_116v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_116v1__map__loc_unknown_start_0_0_end_0_0(op_116v1)
                                };
                                let op_51v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_159v1_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_181v1_send,op_116v1));
                                let op_51v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_51v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_51v1__tee__loc_unknown_start_0_0_end_0_0(op_51v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_21v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_21v1(op_50v1,op_51v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(22v1)",0,var_expr!(hoff_167v1_recv),var_expr!(hoff_162v1_send,hoff_164v1_send,hoff_186v1_send,hoff_188v1_send),false,move|context,var_args!(hoff_167v1_recv),var_args!(hoff_162v1_send,hoff_164v1_send,hoff_186v1_send,hoff_188v1_send)|{
                                let mut hoff_167v1_recv = hoff_167v1_recv.borrow_mut_swap();
                                let hoff_167v1_recv = hoff_167v1_recv.drain(..);
                                let hoff_162v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_162v1_send.give(Some(v));
                                });
                                let hoff_164v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_164v1_send.give(Some(v));
                                });
                                let hoff_186v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_186v1_send.give(Some(v));
                                });
                                let hoff_188v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_188v1_send.give(Some(v));
                                });
                                let op_74v1 = {
                                    fn check_input<Iter: ::std::iter::Iterator<Item = Item> ,Item>(iter:Iter) -> impl ::std::iter::Iterator<Item = Item>{
                                        iter
                                    }
                                    check_input:: <_,_>(hoff_167v1_recv)
                                };
                                let op_74v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_74v1__defer_tick__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_74v1__defer_tick__loc_unknown_start_0_0_end_0_0(op_74v1)
                                };
                                let op_124v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |_|()
                                },hoff_186v1_send);
                                let op_124v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_124v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_124v1__map__loc_unknown_start_0_0_end_0_0(op_124v1)
                                };
                                let op_122v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |_:i32|true
                                },op_124v1);
                                let op_122v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_122v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_122v1__map__loc_unknown_start_0_0_end_0_0(op_122v1)
                                };
                                let op_65v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |a|((),a)
                                },hoff_164v1_send);
                                let op_65v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_65v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_65v1__map__loc_unknown_start_0_0_end_0_0(op_65v1)
                                };
                                let op_61v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_162v1_send,hydroflow_plus::pusherator::tee::Tee::new(op_65v1,hydroflow_plus::pusherator::tee::Tee::new(op_122v1,hoff_188v1_send)));
                                let op_61v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_61v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_61v1__tee__loc_unknown_start_0_0_end_0_0(op_61v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_22v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_22v1(op_74v1,op_61v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(25v1)",1,var_expr!(hoff_170v1_recv,hoff_171v1_recv),var_expr!(hoff_172v1_send,hoff_191v1_send),false,move|context,var_args!(hoff_170v1_recv,hoff_171v1_recv),var_args!(hoff_172v1_send,hoff_191v1_send)|{
                                let mut hoff_170v1_recv = hoff_170v1_recv.borrow_mut_swap();
                                let hoff_170v1_recv = hoff_170v1_recv.drain(..);
                                let mut hoff_171v1_recv = hoff_171v1_recv.borrow_mut_swap();
                                let hoff_171v1_recv = hoff_171v1_recv.drain(..);
                                let hoff_172v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_172v1_send.give(Some(v));
                                });
                                let hoff_191v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_191v1_send.give(Some(v));
                                });
                                let mut sg_25v1_node_80v1_antijoindata_neg_borrow = context.state_ref(sg_25v1_node_80v1_antijoindata_neg).borrow_mut();
                                #[allow(clippy::needless_borrow)]
                                (&mut *sg_25v1_node_80v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).extend(hoff_171v1_recv);
                                let op_80v1 = hoff_170v1_recv.filter(|x: &(_,_)|{
                                    #[allow(clippy::needless_borrow)]
                                    #[allow(clippy::unnecessary_mut_passed)]
                                    !(&mut *sg_25v1_node_80v1_antijoindata_neg_borrow).get_mut_clear(context.current_tick()).contains(&x.0)
                                });
                                let op_80v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_80v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_80v1__anti_join_multiset__loc_unknown_start_0_0_end_0_0(op_80v1)
                                };
                                let op_81v1 = op_80v1.filter_map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    let f = f;
                                    |(slot,(count,p2b))|if count> *f {
                                        Some(p2b)
                                    }else {
                                        None
                                    }
                                });
                                let op_81v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_81v1__filter_map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_81v1__filter_map__loc_unknown_start_0_0_end_0_0(op_81v1)
                                };
                                let op_143v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |p2b|ReplicaPayload {
                                        seq:p2b.slot,key:p2b.value.key,value:p2b.value.value
                                    }
                                },hoff_191v1_send);
                                let op_143v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_143v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_143v1__map__loc_unknown_start_0_0_end_0_0(op_143v1)
                                };
                                let op_83v1 = hydroflow_plus::pusherator::map::Map::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |p2b|p2b.slot
                                },hoff_172v1_send);
                                let op_83v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_83v1__map__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_83v1__map__loc_unknown_start_0_0_end_0_0(op_83v1)
                                };
                                let op_82v1 = hydroflow_plus::pusherator::tee::Tee::new(op_83v1,op_143v1);
                                let op_82v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_82v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_82v1__tee__loc_unknown_start_0_0_end_0_0(op_82v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_25v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_25v1(op_81v1,op_82v1);
                            },);
                            df
                        }
                    }
                }else if __given_id==1usize {
                    {
                        #[allow(unused_qualifications)]
                        {
                            use hydroflow_plus::{
                                var_expr,var_args
                            };
                            let mut df = hydroflow_plus::scheduled::graph::Hydroflow::new();
                            df.__assign_meta_graph("{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_iter ({use crate :: __staged :: cluster :: paxos :: * ; [\\\"Acceptors say hello\\\"]})\"},\"version\":1},{\"value\":{\"Operator\":\"for_each ({use crate :: __staged :: cluster :: paxos :: * ; | s | println ! (\\\"{}\\\" , s)})\"},\"version\":1},{\"value\":{\"Operator\":\"source_stream ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_0\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedTagged < ConnectedDirect > > () . into_source ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| res | {let (id , b) = res . unwrap () ; (id , hydroflow_plus :: runtime_support :: bincode :: deserialize :: < crate :: __staged :: cluster :: paxos :: P1a > (& b) . unwrap ())})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (_ , b) | b})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'static > ({use crate :: __staged :: cluster :: paxos :: * ; | | Ballot {num : 0 , id : 0}} , {use crate :: __staged :: cluster :: paxos :: * ; | max_ballot , p1a | {if p1a . ballot > * max_ballot {* max_ballot = p1a . ballot ;}}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"source_stream ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_1\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedTagged < ConnectedDirect > > () . into_source ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| res | {let (id , b) = res . unwrap () ; (id , hydroflow_plus :: runtime_support :: bincode :: deserialize :: < crate :: __staged :: cluster :: paxos :: P2a > (& b) . unwrap ())})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (_ , b) | b})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"filter ({use crate :: __staged :: cluster :: paxos :: * ; | (p2a , max_ballot) | p2a . ballot >= * max_ballot})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (p2a , _) | (p2a . slot , p2a)})\"},\"version\":1},{\"value\":{\"Operator\":\"reduce_keyed :: < 'static > ({use crate :: __staged :: cluster :: paxos :: * ; | curr_entry , new_entry | {if new_entry . ballot > curr_entry . ballot {* curr_entry = new_entry ;}}})\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'tick > ({use crate :: __staged :: cluster :: paxos :: * ; | | HashMap :: < i32 , LogValue > :: new ()} , {use crate :: __staged :: cluster :: paxos :: * ; | log , (slot , p2a) | {log . insert (slot , LogValue {ballot : p2a . ballot , value : p2a . value}) ;}})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | ((p1a , max_ballot) , log) | (p1a . ballot . id , P1b {ballot : p1a . ballot , max_ballot : max_ballot , accepted : log})})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| (id , data) | {(id , hydroflow_plus :: runtime_support :: bincode :: serialize :: < crate :: __staged :: cluster :: paxos :: P1b > (& data) . unwrap () . into ())})\"},\"version\":1},{\"value\":{\"Operator\":\"dest_sink ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_2\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedDemux < ConnectedDirect > > () . into_sink ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (p2a , max_ballot) | (p2a . ballot . id , P2b {ballot : p2a . ballot , max_ballot : max_ballot , slot : p2a . slot , value : p2a . value})})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| (id , data) | {(id , hydroflow_plus :: runtime_support :: bincode :: serialize :: < crate :: __staged :: cluster :: paxos :: P2b > (& data) . unwrap () . into ())})\"},\"version\":1},{\"value\":{\"Operator\":\"dest_sink ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_3\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedDemux < ConnectedDirect > > () . into_sink ()}})\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":4,\"version\":1},{\"idx\":5,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":6,\"version\":1},{\"idx\":27,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":7,\"version\":1},{\"idx\":8,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":6,\"version\":1},{\"idx\":28,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":8,\"version\":1},{\"idx\":29,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":10,\"version\":1},{\"idx\":11,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":11,\"version\":1},{\"idx\":12,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":12,\"version\":1},{\"idx\":13,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":13,\"version\":1},{\"idx\":30,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":8,\"version\":1},{\"idx\":31,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":14,\"version\":1},{\"idx\":15,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":15,\"version\":1},{\"idx\":16,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":16,\"version\":1},{\"idx\":32,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":17,\"version\":1},{\"idx\":33,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":19,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":18,\"version\":1},{\"idx\":19,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":19,\"version\":1},{\"idx\":20,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":21,\"version\":1},{\"idx\":22,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":20,\"version\":1},{\"idx\":21,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":13,\"version\":1},{\"idx\":34,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":8,\"version\":1},{\"idx\":35,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":23,\"version\":1},{\"idx\":24,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":25,\"version\":1},{\"idx\":26,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":24,\"version\":1},{\"idx\":25,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":27,\"version\":1},{\"idx\":7,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":28,\"version\":1},{\"idx\":9,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":29,\"version\":1},{\"idx\":9,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":30,\"version\":1},{\"idx\":14,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":31,\"version\":1},{\"idx\":14,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":32,\"version\":1},{\"idx\":17,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":33,\"version\":1},{\"idx\":18,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":34,\"version\":1},{\"idx\":23,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":35,\"version\":1},{\"idx\":23,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":6,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":{\"idx\":8,\"version\":1},\"version\":1},{\"value\":{\"idx\":8,\"version\":1},\"version\":1},{\"value\":{\"idx\":8,\"version\":1},\"version\":1},{\"value\":{\"idx\":8,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1},{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":7,\"version\":1},{\"idx\":8,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":10,\"version\":1},{\"idx\":11,\"version\":1},{\"idx\":12,\"version\":1},{\"idx\":13,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":14,\"version\":1},{\"idx\":15,\"version\":1},{\"idx\":16,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":17,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":18,\"version\":1},{\"idx\":19,\"version\":1},{\"idx\":20,\"version\":1},{\"idx\":21,\"version\":1},{\"idx\":22,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":23,\"version\":1},{\"idx\":24,\"version\":1},{\"idx\":25,\"version\":1},{\"idx\":26,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1},{\"value\":1,\"version\":1},{\"value\":0,\"version\":1},{\"value\":1,\"version\":1},{\"value\":2,\"version\":1},{\"value\":3,\"version\":1},{\"value\":1,\"version\":1}],\"node_singleton_references\":[{\"value\":null,\"version\":0},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"stream_88\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":\"stream_106\",\"version\":1},{\"value\":\"stream_106\",\"version\":1},{\"value\":\"stream_107\",\"version\":1},{\"value\":\"stream_108\",\"version\":1},{\"value\":\"stream_109\",\"version\":1},{\"value\":\"stream_110\",\"version\":1},{\"value\":\"stream_111\",\"version\":1},{\"value\":\"stream_136\",\"version\":1},{\"value\":\"stream_136\",\"version\":1},{\"value\":\"stream_137\",\"version\":1},{\"value\":\"stream_138\",\"version\":1},{\"value\":\"stream_139\",\"version\":1},{\"value\":\"stream_140\",\"version\":1},{\"value\":\"stream_141\",\"version\":1},{\"value\":\"stream_142\",\"version\":1},{\"value\":\"stream_143\",\"version\":1},{\"value\":\"stream_144\",\"version\":1},{\"value\":\"stream_145\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":\"stream_147\",\"version\":1},{\"value\":\"stream_148\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}");
                            df.__assign_diagnostics("[]");
                            let(hoff_27v1_send,hoff_27v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(27v1)");
                            let(hoff_28v1_send,hoff_28v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(28v1)");
                            let(hoff_29v1_send,hoff_29v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(29v1)");
                            let(hoff_30v1_send,hoff_30v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(30v1)");
                            let(hoff_31v1_send,hoff_31v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(31v1)");
                            let(hoff_32v1_send,hoff_32v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(32v1)");
                            let(hoff_33v1_send,hoff_33v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(33v1)");
                            let(hoff_34v1_send,hoff_34v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(34v1)");
                            let(hoff_35v1_send,hoff_35v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(35v1)");
                            let mut sg_1v1_node_1v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    ["Acceptors say hello"]
                                })
                            };
                            let mut sg_2v1_node_3v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_0";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedTagged<ConnectedDirect> >().into_source()
                                    }
                                })
                            };
                            let mut sg_4v1_node_10v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_1";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedTagged<ConnectedDirect> >().into_source()
                                    }
                                })
                            };
                            #[allow(unused_mut)]
                            let mut sg_3v1_node_7v1_initializer_func = {
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                | |Ballot {
                                    num:0,id:0
                                }
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_7v1 = df.add_state(::std::cell::RefCell::new((sg_3v1_node_7v1_initializer_func)()));
                            let sg_5v1_node_14v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_5v1_node_14v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_5v1_node_14v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_5v1_node_14v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_6v1_node_17v1_groupbydata = df.add_state(::std::cell::RefCell::new(hydroflow_plus::rustc_hash::FxHashMap:: <_,_> ::default()));
                            let sg_7v1_node_9v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_7v1_node_9v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_7v1_node_9v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_7v1_node_9v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            #[allow(unused_mut)]
                            let mut sg_7v1_node_18v1_initializer_func = {
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                | |HashMap:: <i32,LogValue> ::new()
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_18v1 = df.add_state(::std::cell::RefCell::new((sg_7v1_node_18v1_initializer_func)()));
                            df.set_state_tick_hook(singleton_op_18v1,move|rcell|{
                                rcell.replace((sg_7v1_node_18v1_initializer_func)());
                            });
                            let sg_7v1_node_19v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_7v1_node_19v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_7v1_node_19v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_7v1_node_19v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let(sg_7v1_node_22v1_item_send,sg_7v1_node_22v1_item_recv) = hydroflow_plus::tokio::sync::mpsc::unbounded_channel();
                            {
                                #[doc = r" Function is needed so `Item` is so no ambiguity for what `Item` is used"]
                                #[doc = r" when calling `.flush()`."]
                                async fn sink_feed_flush<Sink,Item>(mut recv:hydroflow_plus::tokio::sync::mpsc::UnboundedReceiver<Item> ,mut sink:Sink,)where Sink: ::std::marker::Unpin+hydroflow_plus::futures::Sink<Item> ,Sink::Error: ::std::fmt::Debug,{
                                    use hydroflow_plus::futures::SinkExt;
                                    while let Some(item) = recv.recv().await {
                                        sink.feed(item).await.expect("Error processing async sink item.");
                                        while let Ok(item) = recv.try_recv(){
                                            sink.feed(item).await.expect("Error processing async sink item.");
                                        }sink.flush().await.expect("Failed to flush sink.");
                                    }
                                }
                                df.request_task(sink_feed_flush(sg_7v1_node_22v1_item_recv,{
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_2";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedDemux<ConnectedDirect> >().into_sink()
                                    }
                                }));
                            }let sg_8v1_node_23v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_8v1_node_23v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_8v1_node_23v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_8v1_node_23v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let(sg_8v1_node_26v1_item_send,sg_8v1_node_26v1_item_recv) = hydroflow_plus::tokio::sync::mpsc::unbounded_channel();
                            {
                                #[doc = r" Function is needed so `Item` is so no ambiguity for what `Item` is used"]
                                #[doc = r" when calling `.flush()`."]
                                async fn sink_feed_flush<Sink,Item>(mut recv:hydroflow_plus::tokio::sync::mpsc::UnboundedReceiver<Item> ,mut sink:Sink,)where Sink: ::std::marker::Unpin+hydroflow_plus::futures::Sink<Item> ,Sink::Error: ::std::fmt::Debug,{
                                    use hydroflow_plus::futures::SinkExt;
                                    while let Some(item) = recv.recv().await {
                                        sink.feed(item).await.expect("Error processing async sink item.");
                                        while let Ok(item) = recv.try_recv(){
                                            sink.feed(item).await.expect("Error processing async sink item.");
                                        }sink.flush().await.expect("Failed to flush sink.");
                                    }
                                }
                                df.request_task(sink_feed_flush(sg_8v1_node_26v1_item_recv,{
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_3";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedDemux<ConnectedDirect> >().into_sink()
                                    }
                                }));
                            }df.add_subgraph_stratified("Subgraph GraphSubgraphId(1v1)",0,var_expr!(),var_expr!(),false,move|context,var_args!(),var_args!()|{
                                let op_1v1 = sg_1v1_node_1v1_iter.by_ref();
                                let op_1v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_1v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_1v1__source_iter__loc_unknown_start_0_0_end_0_0(op_1v1)
                                };
                                let op_2v1 = hydroflow_plus::pusherator::for_each::ForEach::new({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |s|{
                                        //($crate::format_args_nl!("{}",s));
                                    }
                                });
                                let op_2v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_2v1__for_each__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_2v1__for_each__loc_unknown_start_0_0_end_0_0(op_2v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_1v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_1v1(op_1v1,op_2v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(2v1)",0,var_expr!(),var_expr!(hoff_27v1_send,hoff_28v1_send),false,move|context,var_args!(),var_args!(hoff_27v1_send,hoff_28v1_send)|{
                                let hoff_27v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_27v1_send.give(Some(v));
                                });
                                let hoff_28v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_28v1_send.give(Some(v));
                                });
                                let op_3v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_2v1_node_3v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_3v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_3v1__source_stream__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_3v1__source_stream__loc_unknown_start_0_0_end_0_0(op_3v1)
                                };
                                let op_4v1 = op_3v1.map(|res|{
                                    let(id,b) = res.unwrap();
                                    (id,hydroflow_plus::runtime_support::bincode::deserialize:: <hydroflow_plus_test::__staged::cluster::paxos::P1a>(&b).unwrap())
                                });
                                let op_4v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_4v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_4v1__map__loc_unknown_start_0_0_end_0_0(op_4v1)
                                };
                                let op_5v1 = op_4v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(_,b)|b
                                });
                                let op_5v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_5v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_5v1__map__loc_unknown_start_0_0_end_0_0(op_5v1)
                                };
                                let op_6v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_27v1_send,hoff_28v1_send);
                                let op_6v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_6v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_6v1__tee__loc_unknown_start_0_0_end_0_0(op_6v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_2v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_2v1(op_5v1,op_6v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(4v1)",0,var_expr!(),var_expr!(hoff_30v1_send,hoff_34v1_send),false,move|context,var_args!(),var_args!(hoff_30v1_send,hoff_34v1_send)|{
                                let hoff_30v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_30v1_send.give(Some(v));
                                });
                                let hoff_34v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_34v1_send.give(Some(v));
                                });
                                let op_10v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_4v1_node_10v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_10v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_10v1__source_stream__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_10v1__source_stream__loc_unknown_start_0_0_end_0_0(op_10v1)
                                };
                                let op_11v1 = op_10v1.map(|res|{
                                    let(id,b) = res.unwrap();
                                    (id,hydroflow_plus::runtime_support::bincode::deserialize:: <hydroflow_plus_test::__staged::cluster::paxos::P2a>(&b).unwrap())
                                });
                                let op_11v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_11v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_11v1__map__loc_unknown_start_0_0_end_0_0(op_11v1)
                                };
                                let op_12v1 = op_11v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(_,b)|b
                                });
                                let op_12v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_12v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_12v1__map__loc_unknown_start_0_0_end_0_0(op_12v1)
                                };
                                let op_13v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_30v1_send,hoff_34v1_send);
                                let op_13v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_13v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_13v1__tee__loc_unknown_start_0_0_end_0_0(op_13v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_4v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_4v1(op_12v1,op_13v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(3v1)",1,var_expr!(hoff_27v1_recv),var_expr!(hoff_29v1_send,hoff_31v1_send,hoff_35v1_send),false,move|context,var_args!(hoff_27v1_recv),var_args!(hoff_29v1_send,hoff_31v1_send,hoff_35v1_send)|{
                                let mut hoff_27v1_recv = hoff_27v1_recv.borrow_mut_swap();
                                let hoff_27v1_recv = hoff_27v1_recv.drain(..);
                                let hoff_29v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_29v1_send.give(Some(v));
                                });
                                let hoff_31v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_31v1_send.give(Some(v));
                                });
                                let hoff_35v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_35v1_send.give(Some(v));
                                });
                                let op_7v1 = {
                                    let mut sg_3v1_node_7v1_accumulator = context.state_ref(singleton_op_7v1).borrow_mut();
                                    hoff_27v1_recv.for_each(|sg_3v1_node_7v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_3v1_node_7v1_accumulator,sg_3v1_node_7v1_iterator_item,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |max_ballot,p1a|{
                                                if p1a.ballot> *max_ballot {
                                                    *max_ballot = p1a.ballot;
                                                }
                                            }
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_3v1_node_7v1_accumulator))
                                    }
                                };
                                let op_7v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_7v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_7v1__fold__loc_unknown_start_0_0_end_0_0(op_7v1)
                                };
                                let op_8v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_29v1_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_31v1_send,hoff_35v1_send));
                                let op_8v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_8v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_8v1__tee__loc_unknown_start_0_0_end_0_0(op_8v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_3v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_3v1(op_7v1,op_8v1);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(5v1)",1,var_expr!(hoff_30v1_recv,hoff_31v1_recv),var_expr!(hoff_32v1_send),false,move|context,var_args!(hoff_30v1_recv,hoff_31v1_recv),var_args!(hoff_32v1_send)|{
                                let mut hoff_30v1_recv = hoff_30v1_recv.borrow_mut_swap();
                                let hoff_30v1_recv = hoff_30v1_recv.drain(..);
                                let mut hoff_31v1_recv = hoff_31v1_recv.borrow_mut_swap();
                                let hoff_31v1_recv = hoff_31v1_recv.drain(..);
                                let hoff_32v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_32v1_send.give(Some(v));
                                });
                                let hoff_30v1_recv = hoff_30v1_recv.map(|a|((),a));
                                let hoff_31v1_recv = hoff_31v1_recv.map(|b|((),b));
                                let mut sg_5v1_node_14v1_joindata_lhs_borrow = context.state_ref(sg_5v1_node_14v1_joindata_lhs).borrow_mut();
                                let mut sg_5v1_node_14v1_joindata_rhs_borrow = context.state_ref(sg_5v1_node_14v1_joindata_rhs).borrow_mut();
                                let op_14v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_30v1_recv,hoff_31v1_recv, &mut *sg_5v1_node_14v1_joindata_lhs_borrow, &mut *sg_5v1_node_14v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_14v1 = op_14v1.map(|((),(a,b))|(a,b));
                                let op_14v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_14v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_14v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_14v1)
                                };
                                let op_15v1 = op_14v1.filter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(p2a,max_ballot)|p2a.ballot>= *max_ballot
                                });
                                let op_15v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_15v1__filter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_15v1__filter__loc_unknown_start_0_0_end_0_0(op_15v1)
                                };
                                let op_16v1 = op_15v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(p2a,_)|(p2a.slot,p2a)
                                });
                                let op_16v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_16v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_16v1__map__loc_unknown_start_0_0_end_0_0(op_16v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_5v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_5v1(op_16v1,hoff_32v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(6v1)",2,var_expr!(hoff_32v1_recv),var_expr!(hoff_33v1_send),false,move|context,var_args!(hoff_32v1_recv),var_args!(hoff_33v1_send)|{
                                let mut hoff_32v1_recv = hoff_32v1_recv.borrow_mut_swap();
                                let hoff_32v1_recv = hoff_32v1_recv.drain(..);
                                let hoff_33v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_33v1_send.give(Some(v));
                                });
                                let mut sg_6v1_node_17v1_hashtable = context.state_ref(sg_6v1_node_17v1_groupbydata).borrow_mut();
                                {
                                    #[inline(always)]
                                    fn check_input<Iter: ::std::iter::Iterator<Item = (A,B)> ,A: ::std::clone::Clone,B: ::std::clone::Clone>(iter:Iter) -> impl ::std::iter::Iterator<Item = (A,B)>{
                                        iter
                                    }
                                    #[inline(always)]
                                    #[doc = r" A: accumulator type"]
                                    #[doc = r" O: output type"]
                                    fn call_comb_type<A,O>(acc: &mut A,item:A,f:impl Fn(&mut A,A) -> O) -> O {
                                        f(acc,item)
                                    }
                                    for kv in check_input(hoff_32v1_recv){
                                        match sg_6v1_node_17v1_hashtable.entry(kv.0){
                                            ::std::collections::hash_map::Entry::Vacant(vacant) => {
                                                vacant.insert(kv.1);
                                            }
                                            ::std::collections::hash_map::Entry::Occupied(mut occupied) => {
                                                #[allow(clippy::redundant_closure_call)]
                                                call_comb_type(occupied.get_mut(),kv.1,{
                                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                                    |curr_entry,new_entry|{
                                                        if new_entry.ballot>curr_entry.ballot {
                                                            *curr_entry = new_entry;
                                                        }
                                                    }
                                                });
                                            }
                                        
                                            }
                                    }
                                }let op_17v1 = context.is_first_run_this_tick().then_some(sg_6v1_node_17v1_hashtable.iter()).into_iter().flatten().map(#[allow(unknown_lints,suspicious_double_ref_op,clippy::clone_on_copy)]
                                |(k,v)|(::std::clone::Clone::clone(k), ::std::clone::Clone::clone(v),));
                                let op_17v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_17v1__reduce_keyed__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_17v1__reduce_keyed__loc_unknown_start_0_0_end_0_0(op_17v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_6v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_6v1(op_17v1,hoff_33v1_send);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(7v1)",3,var_expr!(hoff_28v1_recv,hoff_29v1_recv,hoff_33v1_recv),var_expr!(),false,move|context,var_args!(hoff_28v1_recv,hoff_29v1_recv,hoff_33v1_recv),var_args!()|{
                                let mut hoff_28v1_recv = hoff_28v1_recv.borrow_mut_swap();
                                let hoff_28v1_recv = hoff_28v1_recv.drain(..);
                                let mut hoff_29v1_recv = hoff_29v1_recv.borrow_mut_swap();
                                let hoff_29v1_recv = hoff_29v1_recv.drain(..);
                                let mut hoff_33v1_recv = hoff_33v1_recv.borrow_mut_swap();
                                let hoff_33v1_recv = hoff_33v1_recv.drain(..);
                                let hoff_28v1_recv = hoff_28v1_recv.map(|a|((),a));
                                let hoff_29v1_recv = hoff_29v1_recv.map(|b|((),b));
                                let mut sg_7v1_node_9v1_joindata_lhs_borrow = context.state_ref(sg_7v1_node_9v1_joindata_lhs).borrow_mut();
                                let mut sg_7v1_node_9v1_joindata_rhs_borrow = context.state_ref(sg_7v1_node_9v1_joindata_rhs).borrow_mut();
                                let op_9v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_28v1_recv,hoff_29v1_recv, &mut *sg_7v1_node_9v1_joindata_lhs_borrow, &mut *sg_7v1_node_9v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_9v1 = op_9v1.map(|((),(a,b))|(a,b));
                                let op_9v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_9v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_9v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_9v1)
                                };
                                let op_18v1 = {
                                    let mut sg_7v1_node_18v1_accumulator = context.state_ref(singleton_op_18v1).borrow_mut();
                                    hoff_33v1_recv.for_each(|sg_7v1_node_18v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_7v1_node_18v1_accumulator,sg_7v1_node_18v1_iterator_item,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |log,(slot,p2a)|{
                                                log.insert(slot,LogValue {
                                                    ballot:p2a.ballot,value:p2a.value
                                                });
                                            }
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_7v1_node_18v1_accumulator))
                                    }
                                };
                                let op_18v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_18v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_18v1__fold__loc_unknown_start_0_0_end_0_0(op_18v1)
                                };
                                let op_9v1 = op_9v1.map(|a|((),a));
                                let op_18v1 = op_18v1.map(|b|((),b));
                                let mut sg_7v1_node_19v1_joindata_lhs_borrow = context.state_ref(sg_7v1_node_19v1_joindata_lhs).borrow_mut();
                                let mut sg_7v1_node_19v1_joindata_rhs_borrow = context.state_ref(sg_7v1_node_19v1_joindata_rhs).borrow_mut();
                                let op_19v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_9v1,op_18v1, &mut *sg_7v1_node_19v1_joindata_lhs_borrow, &mut *sg_7v1_node_19v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_19v1 = op_19v1.map(|((),(a,b))|(a,b));
                                let op_19v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_19v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_19v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_19v1)
                                };
                                let op_20v1 = op_19v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |((p1a,max_ballot),log)|(p1a.ballot.id,P1b {
                                        ballot:p1a.ballot,max_ballot:max_ballot,accepted:log
                                    })
                                });
                                let op_20v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_20v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_20v1__map__loc_unknown_start_0_0_end_0_0(op_20v1)
                                };
                                let op_21v1 = op_20v1.map(|(id,data)|{
                                    (id,hydroflow_plus::runtime_support::bincode::serialize:: <hydroflow_plus_test::__staged::cluster::paxos::P1b>(&data).unwrap().into())
                                });
                                let op_21v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_21v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_21v1__map__loc_unknown_start_0_0_end_0_0(op_21v1)
                                };
                                let op_22v1 = hydroflow_plus::pusherator::for_each::ForEach::new(|item|{
                                    if let Err(err) = sg_7v1_node_22v1_item_send.send(item){
                                        {
                                            //$crate::panicking::panic_fmt($crate::const_format_args!("Failed to send async write item for processing.: {}",err));
                                        };
                                    }
                                });
                                let op_22v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_22v1__dest_sink__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_22v1__dest_sink__loc_unknown_start_0_0_end_0_0(op_22v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_7v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_7v1(op_21v1,op_22v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(8v1)",1,var_expr!(hoff_34v1_recv,hoff_35v1_recv),var_expr!(),false,move|context,var_args!(hoff_34v1_recv,hoff_35v1_recv),var_args!()|{
                                let mut hoff_34v1_recv = hoff_34v1_recv.borrow_mut_swap();
                                let hoff_34v1_recv = hoff_34v1_recv.drain(..);
                                let mut hoff_35v1_recv = hoff_35v1_recv.borrow_mut_swap();
                                let hoff_35v1_recv = hoff_35v1_recv.drain(..);
                                let hoff_34v1_recv = hoff_34v1_recv.map(|a|((),a));
                                let hoff_35v1_recv = hoff_35v1_recv.map(|b|((),b));
                                let mut sg_8v1_node_23v1_joindata_lhs_borrow = context.state_ref(sg_8v1_node_23v1_joindata_lhs).borrow_mut();
                                let mut sg_8v1_node_23v1_joindata_rhs_borrow = context.state_ref(sg_8v1_node_23v1_joindata_rhs).borrow_mut();
                                let op_23v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_34v1_recv,hoff_35v1_recv, &mut *sg_8v1_node_23v1_joindata_lhs_borrow, &mut *sg_8v1_node_23v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_23v1 = op_23v1.map(|((),(a,b))|(a,b));
                                let op_23v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_23v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_23v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_23v1)
                                };
                                let op_24v1 = op_23v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(p2a,max_ballot)|(p2a.ballot.id,P2b {
                                        ballot:p2a.ballot,max_ballot:max_ballot,slot:p2a.slot,value:p2a.value
                                    })
                                });
                                let op_24v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_24v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_24v1__map__loc_unknown_start_0_0_end_0_0(op_24v1)
                                };
                                let op_25v1 = op_24v1.map(|(id,data)|{
                                    (id,hydroflow_plus::runtime_support::bincode::serialize:: <hydroflow_plus_test::__staged::cluster::paxos::P2b>(&data).unwrap().into())
                                });
                                let op_25v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_25v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_25v1__map__loc_unknown_start_0_0_end_0_0(op_25v1)
                                };
                                let op_26v1 = hydroflow_plus::pusherator::for_each::ForEach::new(|item|{
                                    if let Err(err) = sg_8v1_node_26v1_item_send.send(item){
                                        {
                                            //$crate::panicking::panic_fmt($crate::const_format_args!("Failed to send async write item for processing.: {}",err));
                                        };
                                    }
                                });
                                let op_26v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_26v1__dest_sink__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_26v1__dest_sink__loc_unknown_start_0_0_end_0_0(op_26v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_8v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_8v1(op_25v1,op_26v1);
                            },);
                            df
                        }
                    }
                }else if __given_id==2usize {
                    {
                        #[allow(unused_qualifications)]
                        {
                            use hydroflow_plus::{
                                var_expr,var_args
                            };
                            let mut df = hydroflow_plus::scheduled::graph::Hydroflow::new();
                            df.__assign_meta_graph("{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_iter ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let cli = cli ; let self_id = 0usize ; cli . meta . clusters . get (& self_id) . unwrap ()})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | d | d . clone ()})\"},\"version\":1},{\"value\":{\"Operator\":\"source_iter ({use crate :: __staged :: cluster :: paxos :: * ; [ClientPayload {key : 10 , value : \\\"Hello, Berkeley!\\\" . to_string ()} , ClientPayload {key : 10 , value : \\\"Goodbye, Berkeley\\\" . to_string ()} , ClientPayload {key : 20 , value : \\\"Hello, SF\\\" . to_string ()} , ClientPayload {key : 20 , value : \\\"Goodbye, SF\\\" . to_string ()}]})\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'static , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"map (| (id , data) | {(id , hydroflow_plus :: runtime_support :: bincode :: serialize :: < crate :: __staged :: cluster :: paxos :: ClientPayload > (& data) . unwrap () . into ())})\"},\"version\":1},{\"value\":{\"Operator\":\"dest_sink ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_0\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedDemux < ConnectedDirect > > () . into_sink ()}})\"},\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":4,\"version\":1},{\"idx\":5,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1},{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1}],\"node_singleton_references\":[{\"value\":null,\"version\":0},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"stream_48\",\"version\":1},{\"value\":\"stream_49\",\"version\":1},{\"value\":\"stream_50\",\"version\":1},{\"value\":\"stream_51\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}");
                            df.__assign_diagnostics("[]");
                            let mut sg_1v1_node_1v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let cli = cli;
                                    let self_id = 0usize;
                                    cli.meta.clusters.get(&self_id).unwrap()
                                })
                            };
                            let mut sg_1v1_node_3v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    [ClientPayload {
                                        key:10,value:"Hello, Berkeley!".to_string()
                                    },ClientPayload {
                                        key:10,value:"Goodbye, Berkeley".to_string()
                                    },ClientPayload {
                                        key:20,value:"Hello, SF".to_string()
                                    },ClientPayload {
                                        key:20,value:"Goodbye, SF".to_string()
                                    }]
                                })
                            };
                            let sg_1v1_node_4v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            let sg_1v1_node_4v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_1v1_node_4v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let(sg_1v1_node_6v1_item_send,sg_1v1_node_6v1_item_recv) = hydroflow_plus::tokio::sync::mpsc::unbounded_channel();
                            {
                                #[doc = r" Function is needed so `Item` is so no ambiguity for what `Item` is used"]
                                #[doc = r" when calling `.flush()`."]
                                async fn sink_feed_flush<Sink,Item>(mut recv:hydroflow_plus::tokio::sync::mpsc::UnboundedReceiver<Item> ,mut sink:Sink,)where Sink: ::std::marker::Unpin+hydroflow_plus::futures::Sink<Item> ,Sink::Error: ::std::fmt::Debug,{
                                    use hydroflow_plus::futures::SinkExt;
                                    while let Some(item) = recv.recv().await {
                                        sink.feed(item).await.expect("Error processing async sink item.");
                                        while let Ok(item) = recv.try_recv(){
                                            sink.feed(item).await.expect("Error processing async sink item.");
                                        }sink.flush().await.expect("Failed to flush sink.");
                                    }
                                }
                                df.request_task(sink_feed_flush(sg_1v1_node_6v1_item_recv,{
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_0";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedDemux<ConnectedDirect> >().into_sink()
                                    }
                                }));
                            }df.add_subgraph_stratified("Subgraph GraphSubgraphId(1v1)",0,var_expr!(),var_expr!(),false,move|context,var_args!(),var_args!()|{
                                let op_1v1 = sg_1v1_node_1v1_iter.by_ref();
                                let op_1v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_1v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_1v1__source_iter__loc_unknown_start_0_0_end_0_0(op_1v1)
                                };
                                let op_2v1 = op_1v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |d|d.clone()
                                });
                                let op_2v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_2v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_2v1__map__loc_unknown_start_0_0_end_0_0(op_2v1)
                                };
                                let op_3v1 = sg_1v1_node_3v1_iter.by_ref();
                                let op_3v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_3v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_3v1__source_iter__loc_unknown_start_0_0_end_0_0(op_3v1)
                                };
                                let op_2v1 = op_2v1.map(|a|((),a));
                                let op_3v1 = op_3v1.map(|b|((),b));
                                let mut sg_1v1_node_4v1_joindata_lhs_borrow = context.state_ref(sg_1v1_node_4v1_joindata_lhs).borrow_mut();
                                let mut sg_1v1_node_4v1_joindata_rhs_borrow = context.state_ref(sg_1v1_node_4v1_joindata_rhs).borrow_mut();
                                let op_4v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(op_2v1,op_3v1, &mut *sg_1v1_node_4v1_joindata_lhs_borrow, &mut *sg_1v1_node_4v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_4v1 = op_4v1.map(|((),(a,b))|(a,b));
                                let op_4v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_4v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_4v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_4v1)
                                };
                                let op_5v1 = op_4v1.map(|(id,data)|{
                                    (id,hydroflow_plus::runtime_support::bincode::serialize:: <hydroflow_plus_test::__staged::cluster::paxos::ClientPayload>(&data).unwrap().into())
                                });
                                let op_5v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_5v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_5v1__map__loc_unknown_start_0_0_end_0_0(op_5v1)
                                };
                                let op_6v1 = hydroflow_plus::pusherator::for_each::ForEach::new(|item|{
                                    if let Err(err) = sg_1v1_node_6v1_item_send.send(item){
                                        {
                                            //$crate::panicking::panic_fmt($crate::const_format_args!("Failed to send async write item for processing.: {}",err));
                                        };
                                    }
                                });
                                let op_6v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_6v1__dest_sink__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_6v1__dest_sink__loc_unknown_start_0_0_end_0_0(op_6v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_1v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_1v1(op_5v1,op_6v1);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df
                        }
                    }
                }else if __given_id==3usize {
                    {
                        #[allow(unused_qualifications)]
                        {
                            use hydroflow_plus::{
                                var_expr,var_args
                            };
                            let mut df = hydroflow_plus::scheduled::graph::Hydroflow::new();
                            df.__assign_meta_graph("{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_stream ({use hydroflow_plus_cli_integration :: __staged :: runtime :: * ; let port = \\\"port_0\\\" ; let self_cli = cli ; {self_cli . port (port) . connect_local_blocking :: < ConnectedTagged < ConnectedDirect > > () . into_source ()}})\"},\"version\":1},{\"value\":{\"Operator\":\"map (| res | {let (id , b) = res . unwrap () ; (id , hydroflow_plus :: runtime_support :: bincode :: deserialize :: < crate :: __staged :: cluster :: paxos :: ReplicaPayload > (& b) . unwrap ())})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use hydroflow_plus :: __staged :: stream :: * ; | (_ , b) | b})\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"sort ()\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"source_iter ({use crate :: __staged :: cluster :: paxos :: * ; [- 1]})\"},\"version\":1},{\"value\":{\"Operator\":\"union ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'tick > ({use crate :: __staged :: cluster :: paxos :: * ; | | - 1} , {use crate :: __staged :: cluster :: paxos :: * ; | filled_slot , (sorted_payload , highest_seq) | {let next_slot = std :: cmp :: max (* filled_slot , highest_seq) ; * filled_slot = if sorted_payload . seq == next_slot + 1 {sorted_payload . seq} else {* filled_slot} ;}})\"},\"version\":1},{\"value\":{\"Operator\":\"tee ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"filter ({use crate :: __staged :: cluster :: paxos :: * ; | (sorted_payload , highest_seq) | sorted_payload . seq > * highest_seq})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (sorted_payload , _) | sorted_payload})\"},\"version\":1},{\"value\":{\"Operator\":\"defer_tick ()\"},\"version\":1},{\"value\":{\"Operator\":\"cross_join_multiset :: < 'tick , 'tick > ()\"},\"version\":1},{\"value\":{\"Operator\":\"filter ({use crate :: __staged :: cluster :: paxos :: * ; | (sorted_payload , highest_seq) | sorted_payload . seq <= * highest_seq})\"},\"version\":1},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (sorted_payload , _) | sorted_payload})\"},\"version\":1},{\"value\":{\"Operator\":\"fold :: < 'static > ({use crate :: __staged :: cluster :: paxos :: * ; | | (HashMap :: < u32 , String > :: new () , - 1)} , {use crate :: __staged :: cluster :: paxos :: * ; | state , payload | {let ref mut kv_store = state . 0 ; let ref mut last_seq = state . 1 ; kv_store . insert (payload . key , payload . value) ; debug_assert ! (payload . seq == * last_seq + 1 , \\\"Hole in log between seq {} and {}\\\" , * last_seq , payload . seq) ; * last_seq = payload . seq ; println ! (\\\"Replica kv store: {:?}\\\" , kv_store) ;}})\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":3},{\"value\":{\"Operator\":\"map ({use crate :: __staged :: cluster :: paxos :: * ; | (kv_store , highest_seq) | highest_seq})\"},\"version\":1},{\"value\":{\"Operator\":\"defer_tick ()\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":15,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":4,\"version\":1},{\"idx\":20,\"version\":3}],\"version\":3},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":22,\"version\":1},{\"idx\":8,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":7,\"version\":1},{\"idx\":8,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":6,\"version\":1},{\"idx\":23,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":8,\"version\":1},{\"idx\":9,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":24,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":10,\"version\":1},{\"idx\":11,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":6,\"version\":1},{\"idx\":25,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":11,\"version\":1},{\"idx\":26,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":12,\"version\":1},{\"idx\":13,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":13,\"version\":1},{\"idx\":14,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":14,\"version\":1},{\"idx\":27,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":6,\"version\":1},{\"idx\":28,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":11,\"version\":1},{\"idx\":29,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":16,\"version\":1},{\"idx\":17,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":17,\"version\":1},{\"idx\":18,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":18,\"version\":1},{\"idx\":30,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":20,\"version\":3},{\"idx\":5,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":19,\"version\":1},{\"idx\":31,\"version\":1}],\"version\":5},{\"value\":[{\"idx\":21,\"version\":1},{\"idx\":32,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":23,\"version\":1},{\"idx\":9,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":24,\"version\":1},{\"idx\":10,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":25,\"version\":1},{\"idx\":12,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":26,\"version\":1},{\"idx\":12,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":27,\"version\":1},{\"idx\":15,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":28,\"version\":1},{\"idx\":16,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":29,\"version\":1},{\"idx\":16,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":30,\"version\":1},{\"idx\":19,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":31,\"version\":1},{\"idx\":21,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":32,\"version\":1},{\"idx\":22,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":5},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"0\"}],\"version\":1},{\"value\":[\"Elided\",{\"Int\":\"1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":4,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":5,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":6,\"version\":1},\"version\":1},{\"value\":{\"idx\":6,\"version\":1},\"version\":1},{\"value\":{\"idx\":6,\"version\":1},\"version\":1},{\"value\":{\"idx\":7,\"version\":1},\"version\":1},{\"value\":null,\"version\":0},{\"value\":{\"idx\":8,\"version\":1},\"version\":1},{\"value\":{\"idx\":3,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1},{\"idx\":15,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":7,\"version\":1},{\"idx\":22,\"version\":1},{\"idx\":8,\"version\":1},{\"idx\":9,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":10,\"version\":1},{\"idx\":11,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":12,\"version\":1},{\"idx\":13,\"version\":1},{\"idx\":14,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":16,\"version\":1},{\"idx\":17,\"version\":1},{\"idx\":18,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":19,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":21,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1},{\"value\":1,\"version\":1},{\"value\":1,\"version\":1},{\"value\":2,\"version\":1},{\"value\":2,\"version\":1},{\"value\":2,\"version\":1},{\"value\":3,\"version\":1},{\"value\":3,\"version\":1}],\"node_singleton_references\":[{\"value\":null,\"version\":0},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1},{\"value\":[],\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"stream_154\",\"version\":1},{\"value\":\"stream_154\",\"version\":1},{\"value\":\"stream_155\",\"version\":1},{\"value\":\"stream_156\",\"version\":1},{\"value\":\"stream_157\",\"version\":1},{\"value\":\"stream_158\",\"version\":1},{\"value\":\"stream_159\",\"version\":1},{\"value\":\"stream_160\",\"version\":1},{\"value\":\"stream_161\",\"version\":1},{\"value\":\"stream_162\",\"version\":1},{\"value\":\"stream_163\",\"version\":1},{\"value\":\"stream_164\",\"version\":1},{\"value\":\"stream_165\",\"version\":1},{\"value\":\"stream_166\",\"version\":1},{\"value\":\"stream_167\",\"version\":1},{\"value\":\"stream_168\",\"version\":1},{\"value\":\"stream_169\",\"version\":1},{\"value\":\"stream_170\",\"version\":1},{\"value\":\"stream_171\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":\"stream_173\",\"version\":1},{\"value\":\"stream_174\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":2,\"lattice_flow_type\":null},\"version\":3},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":4,\"lattice_flow_type\":null},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":2,\"lattice_flow_type\":null},\"version\":3}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}");
                            df.__assign_diagnostics("[]");
                            let(hoff_20v3_send,hoff_20v3_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(20v3)");
                            let(hoff_23v1_send,hoff_23v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(23v1)");
                            let(hoff_24v1_send,hoff_24v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(24v1)");
                            let(hoff_25v1_send,hoff_25v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(25v1)");
                            let(hoff_26v1_send,hoff_26v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(26v1)");
                            let(hoff_27v1_send,hoff_27v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(27v1)");
                            let(hoff_28v1_send,hoff_28v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(28v1)");
                            let(hoff_29v1_send,hoff_29v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(29v1)");
                            let(hoff_30v1_send,hoff_30v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(30v1)");
                            let(hoff_31v1_send,hoff_31v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(31v1)");
                            let(hoff_32v1_send,hoff_32v1_recv) = df.make_edge:: <_,hydroflow_plus::scheduled::handoff::VecHandoff<_> >("handoff GraphNodeId(32v1)");
                            let mut sg_1v1_node_1v1_stream = {
                                #[inline(always)]
                                fn check_stream<Stream:hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin,Item>(stream:Stream) -> impl hydroflow_plus::futures::stream::Stream<Item = Item> + ::std::marker::Unpin {
                                    stream
                                }
                                check_stream({
                                    use hydroflow_plus_cli_integration::__staged::runtime:: * ;
                                    let port = "port_0";
                                    let self_cli = cli;
                                    {
                                        self_cli.port(port).connect_local_blocking:: <ConnectedTagged<ConnectedDirect> >().into_source()
                                    }
                                })
                            };
                            let mut sg_3v1_node_7v1_iter = {
                                #[inline(always)]
                                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item> ,Item>(into_iter:IntoIter) -> impl ::std::iter::Iterator<Item = Item>{
                                    ::std::iter::IntoIterator::into_iter(into_iter)
                                }
                                check_iter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    [-1]
                                })
                            };
                            let sg_3v1_node_9v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_3v1_node_9v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_3v1_node_9v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_3v1_node_9v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            #[allow(unused_mut)]
                            let mut sg_4v1_node_10v1_initializer_func = {
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                | | -1
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_10v1 = df.add_state(::std::cell::RefCell::new((sg_4v1_node_10v1_initializer_func)()));
                            df.set_state_tick_hook(singleton_op_10v1,move|rcell|{
                                rcell.replace((sg_4v1_node_10v1_initializer_func)());
                            });
                            let sg_5v1_node_12v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_5v1_node_12v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_5v1_node_12v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_5v1_node_12v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_6v1_node_16v1_joindata_lhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_6v1_node_16v1_joindata_lhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            let sg_6v1_node_16v1_joindata_rhs = df.add_state(::std::cell::RefCell::new(hydroflow_plus::compiled::pull::HalfMultisetJoinState::default()));
                            df.set_state_tick_hook(sg_6v1_node_16v1_joindata_rhs, |rcell|hydroflow_plus::util::clear::Clear::clear(rcell.get_mut()));
                            #[allow(unused_mut)]
                            let mut sg_7v1_node_19v1_initializer_func = {
                                use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                | |(HashMap:: <u32,String> ::new(), -1)
                            };
                            #[allow(clippy::redundant_closure_call)]
                            let singleton_op_19v1 = df.add_state(::std::cell::RefCell::new((sg_7v1_node_19v1_initializer_func)()));
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(1v1)",0,var_expr!(hoff_27v1_recv),var_expr!(hoff_20v3_send),false,move|context,var_args!(hoff_27v1_recv),var_args!(hoff_20v3_send)|{
                                let mut hoff_27v1_recv = hoff_27v1_recv.borrow_mut_swap();
                                let hoff_27v1_recv = hoff_27v1_recv.drain(..);
                                let hoff_20v3_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_20v3_send.give(Some(v));
                                });
                                let op_1v1 = std::iter::from_fn(| |{
                                    match hydroflow_plus::futures::stream::Stream::poll_next(::std::pin::Pin::new(&mut sg_1v1_node_1v1_stream), &mut std::task::Context::from_waker(&context.waker())){
                                        std::task::Poll::Ready(maybe) => maybe,
                                        std::task::Poll::Pending => None,
                                    
                                        }
                                });
                                let op_1v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_1v1__source_stream__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_1v1__source_stream__loc_unknown_start_0_0_end_0_0(op_1v1)
                                };
                                let op_2v1 = op_1v1.map(|res|{
                                    let(id,b) = res.unwrap();
                                    (id,hydroflow_plus::runtime_support::bincode::deserialize:: <hydroflow_plus_test::__staged::cluster::paxos::ReplicaPayload>(&b).unwrap())
                                });
                                let op_2v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_2v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_2v1__map__loc_unknown_start_0_0_end_0_0(op_2v1)
                                };
                                let op_3v1 = op_2v1.map({
                                    use hydroflow_plus::__staged::stream:: * ;
                                    |(_,b)|b
                                });
                                let op_3v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_3v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_3v1__map__loc_unknown_start_0_0_end_0_0(op_3v1)
                                };
                                let op_15v1 = {
                                    fn check_input<Iter: ::std::iter::Iterator<Item = Item> ,Item>(iter:Iter) -> impl ::std::iter::Iterator<Item = Item>{
                                        iter
                                    }
                                    check_input:: <_,_>(hoff_27v1_recv)
                                };
                                let op_15v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_15v1__defer_tick__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_15v1__defer_tick__loc_unknown_start_0_0_end_0_0(op_15v1)
                                };
                                let op_4v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(op_3v1,op_15v1)
                                };
                                let op_4v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_4v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_4v1__union__loc_unknown_start_0_0_end_0_0(op_4v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_1v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_1v1(op_4v1,hoff_20v3_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(3v1)",1,var_expr!(hoff_23v1_recv,hoff_32v1_recv),var_expr!(hoff_24v1_send),false,move|context,var_args!(hoff_23v1_recv,hoff_32v1_recv),var_args!(hoff_24v1_send)|{
                                let mut hoff_23v1_recv = hoff_23v1_recv.borrow_mut_swap();
                                let hoff_23v1_recv = hoff_23v1_recv.drain(..);
                                let mut hoff_32v1_recv = hoff_32v1_recv.borrow_mut_swap();
                                let hoff_32v1_recv = hoff_32v1_recv.drain(..);
                                let hoff_24v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_24v1_send.give(Some(v));
                                });
                                let op_7v1 = sg_3v1_node_7v1_iter.by_ref();
                                let op_7v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_7v1__source_iter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_7v1__source_iter__loc_unknown_start_0_0_end_0_0(op_7v1)
                                };
                                let op_22v1 = {
                                    fn check_input<Iter: ::std::iter::Iterator<Item = Item> ,Item>(iter:Iter) -> impl ::std::iter::Iterator<Item = Item>{
                                        iter
                                    }
                                    check_input:: <_,_>(hoff_32v1_recv)
                                };
                                let op_22v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_22v1__defer_tick__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_22v1__defer_tick__loc_unknown_start_0_0_end_0_0(op_22v1)
                                };
                                let op_8v1 = {
                                    #[allow(unused)]
                                    #[inline(always)]
                                    fn check_inputs<A: ::std::iter::Iterator<Item = Item> ,B: ::std::iter::Iterator<Item = Item> ,Item>(a:A,b:B) -> impl ::std::iter::Iterator<Item = Item>{
                                        a.chain(b)
                                    }
                                    check_inputs(op_22v1,op_7v1)
                                };
                                let op_8v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_8v1__union__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_8v1__union__loc_unknown_start_0_0_end_0_0(op_8v1)
                                };
                                let hoff_23v1_recv = hoff_23v1_recv.map(|a|((),a));
                                let op_8v1 = op_8v1.map(|b|((),b));
                                let mut sg_3v1_node_9v1_joindata_lhs_borrow = context.state_ref(sg_3v1_node_9v1_joindata_lhs).borrow_mut();
                                let mut sg_3v1_node_9v1_joindata_rhs_borrow = context.state_ref(sg_3v1_node_9v1_joindata_rhs).borrow_mut();
                                let op_9v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_23v1_recv,op_8v1, &mut *sg_3v1_node_9v1_joindata_lhs_borrow, &mut *sg_3v1_node_9v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_9v1 = op_9v1.map(|((),(a,b))|(a,b));
                                let op_9v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_9v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_9v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_9v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_3v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_3v1(op_9v1,hoff_24v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(2v1)",1,var_expr!(hoff_20v3_recv),var_expr!(hoff_23v1_send,hoff_25v1_send,hoff_28v1_send),false,move|context,var_args!(hoff_20v3_recv),var_args!(hoff_23v1_send,hoff_25v1_send,hoff_28v1_send)|{
                                let mut hoff_20v3_recv = hoff_20v3_recv.borrow_mut_swap();
                                let hoff_20v3_recv = hoff_20v3_recv.drain(..);
                                let hoff_23v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_23v1_send.give(Some(v));
                                });
                                let hoff_25v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_25v1_send.give(Some(v));
                                });
                                let hoff_28v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_28v1_send.give(Some(v));
                                });
                                let op_5v1 = {
                                    let mut v = hoff_20v3_recv.collect:: < ::std::vec::Vec<_> >();
                                    v.sort_unstable();
                                    v.into_iter()
                                };
                                let op_5v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_5v1__sort__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_5v1__sort__loc_unknown_start_0_0_end_0_0(op_5v1)
                                };
                                let op_6v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_23v1_send,hydroflow_plus::pusherator::tee::Tee::new(hoff_25v1_send,hoff_28v1_send));
                                let op_6v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_6v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_6v1__tee__loc_unknown_start_0_0_end_0_0(op_6v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_2v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_2v1(op_5v1,op_6v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(4v1)",2,var_expr!(hoff_24v1_recv),var_expr!(hoff_26v1_send,hoff_29v1_send),false,move|context,var_args!(hoff_24v1_recv),var_args!(hoff_26v1_send,hoff_29v1_send)|{
                                let mut hoff_24v1_recv = hoff_24v1_recv.borrow_mut_swap();
                                let hoff_24v1_recv = hoff_24v1_recv.drain(..);
                                let hoff_26v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_26v1_send.give(Some(v));
                                });
                                let hoff_29v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_29v1_send.give(Some(v));
                                });
                                let op_10v1 = {
                                    let mut sg_4v1_node_10v1_accumulator = context.state_ref(singleton_op_10v1).borrow_mut();
                                    hoff_24v1_recv.for_each(|sg_4v1_node_10v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_4v1_node_10v1_accumulator,sg_4v1_node_10v1_iterator_item,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |filled_slot,(sorted_payload,highest_seq)|{
                                                let next_slot = std::cmp::max(*filled_slot,highest_seq);
                                                *filled_slot = if sorted_payload.seq==next_slot+1 {
                                                    sorted_payload.seq
                                                }else {
                                                    *filled_slot
                                                };
                                            }
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_4v1_node_10v1_accumulator))
                                    }
                                };
                                let op_10v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_10v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_10v1__fold__loc_unknown_start_0_0_end_0_0(op_10v1)
                                };
                                let op_11v1 = hydroflow_plus::pusherator::tee::Tee::new(hoff_26v1_send,hoff_29v1_send);
                                let op_11v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_11v1__tee__loc_unknown_start_0_0_end_0_0<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >(input:Input) -> impl hydroflow_plus::pusherator::Pusherator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Push<Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input:hydroflow_plus::pusherator::Pusherator<Item = Item> >hydroflow_plus::pusherator::Pusherator for Push<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn give(&mut self,item:Self::Item){
                                                self.inner.give(item)
                                            }
                                        
                                            }
                                        Push {
                                            inner:input
                                        }
                                    }
                                    op_11v1__tee__loc_unknown_start_0_0_end_0_0(op_11v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_4v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_4v1(op_10v1,op_11v1);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(5v1)",2,var_expr!(hoff_25v1_recv,hoff_26v1_recv),var_expr!(hoff_27v1_send),false,move|context,var_args!(hoff_25v1_recv,hoff_26v1_recv),var_args!(hoff_27v1_send)|{
                                let mut hoff_25v1_recv = hoff_25v1_recv.borrow_mut_swap();
                                let hoff_25v1_recv = hoff_25v1_recv.drain(..);
                                let mut hoff_26v1_recv = hoff_26v1_recv.borrow_mut_swap();
                                let hoff_26v1_recv = hoff_26v1_recv.drain(..);
                                let hoff_27v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_27v1_send.give(Some(v));
                                });
                                let hoff_25v1_recv = hoff_25v1_recv.map(|a|((),a));
                                let hoff_26v1_recv = hoff_26v1_recv.map(|b|((),b));
                                let mut sg_5v1_node_12v1_joindata_lhs_borrow = context.state_ref(sg_5v1_node_12v1_joindata_lhs).borrow_mut();
                                let mut sg_5v1_node_12v1_joindata_rhs_borrow = context.state_ref(sg_5v1_node_12v1_joindata_rhs).borrow_mut();
                                let op_12v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_25v1_recv,hoff_26v1_recv, &mut *sg_5v1_node_12v1_joindata_lhs_borrow, &mut *sg_5v1_node_12v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_12v1 = op_12v1.map(|((),(a,b))|(a,b));
                                let op_12v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_12v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_12v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_12v1)
                                };
                                let op_13v1 = op_12v1.filter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(sorted_payload,highest_seq)|sorted_payload.seq> *highest_seq
                                });
                                let op_13v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_13v1__filter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_13v1__filter__loc_unknown_start_0_0_end_0_0(op_13v1)
                                };
                                let op_14v1 = op_13v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(sorted_payload,_)|sorted_payload
                                });
                                let op_14v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_14v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_14v1__map__loc_unknown_start_0_0_end_0_0(op_14v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_5v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_5v1(op_14v1,hoff_27v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(6v1)",2,var_expr!(hoff_28v1_recv,hoff_29v1_recv),var_expr!(hoff_30v1_send),false,move|context,var_args!(hoff_28v1_recv,hoff_29v1_recv),var_args!(hoff_30v1_send)|{
                                let mut hoff_28v1_recv = hoff_28v1_recv.borrow_mut_swap();
                                let hoff_28v1_recv = hoff_28v1_recv.drain(..);
                                let mut hoff_29v1_recv = hoff_29v1_recv.borrow_mut_swap();
                                let hoff_29v1_recv = hoff_29v1_recv.drain(..);
                                let hoff_30v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_30v1_send.give(Some(v));
                                });
                                let hoff_28v1_recv = hoff_28v1_recv.map(|a|((),a));
                                let hoff_29v1_recv = hoff_29v1_recv.map(|b|((),b));
                                let mut sg_6v1_node_16v1_joindata_lhs_borrow = context.state_ref(sg_6v1_node_16v1_joindata_lhs).borrow_mut();
                                let mut sg_6v1_node_16v1_joindata_rhs_borrow = context.state_ref(sg_6v1_node_16v1_joindata_rhs).borrow_mut();
                                let op_16v1 = {
                                    #[inline(always)]
                                    fn check_inputs<'a,K,I1,V1,I2,V2>(lhs:I1,rhs:I2,lhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V1,V2> ,rhs_state: &'a mut hydroflow_plus::compiled::pull::HalfMultisetJoinState<K,V2,V1> ,is_new_tick:bool,) -> impl 'a+Iterator<Item = (K,(V1,V2))>where K:Eq+std::hash::Hash+Clone,V1:Clone,V2:Clone,I1:'a+Iterator<Item = (K,V1)> ,I2:'a+Iterator<Item = (K,V2)> ,{
                                        hydroflow_plus::compiled::pull::symmetric_hash_join_into_iter(lhs,rhs,lhs_state,rhs_state,is_new_tick)
                                    }
                                    check_inputs(hoff_28v1_recv,hoff_29v1_recv, &mut *sg_6v1_node_16v1_joindata_lhs_borrow, &mut *sg_6v1_node_16v1_joindata_rhs_borrow,context.is_first_run_this_tick())
                                };
                                let op_16v1 = op_16v1.map(|((),(a,b))|(a,b));
                                let op_16v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_16v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_16v1__cross_join_multiset__loc_unknown_start_0_0_end_0_0(op_16v1)
                                };
                                let op_17v1 = op_16v1.filter({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(sorted_payload,highest_seq)|sorted_payload.seq<= *highest_seq
                                });
                                let op_17v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_17v1__filter__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_17v1__filter__loc_unknown_start_0_0_end_0_0(op_17v1)
                                };
                                let op_18v1 = op_17v1.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(sorted_payload,_)|sorted_payload
                                });
                                let op_18v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_18v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_18v1__map__loc_unknown_start_0_0_end_0_0(op_18v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_6v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_6v1(op_18v1,hoff_30v1_send);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(7v1)",3,var_expr!(hoff_30v1_recv),var_expr!(hoff_31v1_send),false,move|context,var_args!(hoff_30v1_recv),var_args!(hoff_31v1_send)|{
                                let mut hoff_30v1_recv = hoff_30v1_recv.borrow_mut_swap();
                                let hoff_30v1_recv = hoff_30v1_recv.drain(..);
                                let hoff_31v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_31v1_send.give(Some(v));
                                });
                                let op_19v1 = {
                                    let mut sg_7v1_node_19v1_accumulator = context.state_ref(singleton_op_19v1).borrow_mut();
                                    hoff_30v1_recv.for_each(|sg_7v1_node_19v1_iterator_item|{
                                        #[inline(always)]
                                        fn call_comb_type<Accum,Item>(accum: &mut Accum,item:Item,func:impl Fn(&mut Accum,Item),){
                                            (func)(accum,item);
                                        }
                                        #[allow(clippy::redundant_closure_call)]
                                        call_comb_type(&mut *sg_7v1_node_19v1_accumulator,sg_7v1_node_19v1_iterator_item,{
                                            use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                            |state,payload|{
                                                let ref mut kv_store = state.0;
                                                let ref mut last_seq = state.1;
                                                kv_store.insert(payload.key,payload.value);
                                                if true {
                                                    {
                                                        if!(payload.seq== *last_seq+1){
                                                            {
                                                                //$crate::panicking::panic_fmt($crate::const_format_args!("Hole in log between seq {} and {}",*last_seq,payload.seq));
                                                            };
                                                        }
                                                    };
                                                };
                                                *last_seq = payload.seq;
                                                {
                                                    //($crate::format_args_nl!("Replica kv store: {:?}",kv_store));
                                                };
                                            }
                                        });
                                    });
                                    #[allow(clippy::clone_on_copy)]
                                    {
                                        ::std::iter::once(::std::clone::Clone::clone(& *sg_7v1_node_19v1_accumulator))
                                    }
                                };
                                let op_19v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_19v1__fold__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_19v1__fold__loc_unknown_start_0_0_end_0_0(op_19v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_7v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_7v1(op_19v1,hoff_31v1_send);
                                context.schedule_subgraph(context.current_subgraph(),false);
                            },);
                            df.add_subgraph_stratified("Subgraph GraphSubgraphId(8v1)",3,var_expr!(hoff_31v1_recv),var_expr!(hoff_32v1_send),false,move|context,var_args!(hoff_31v1_recv),var_args!(hoff_32v1_send)|{
                                let mut hoff_31v1_recv = hoff_31v1_recv.borrow_mut_swap();
                                let hoff_31v1_recv = hoff_31v1_recv.drain(..);
                                let hoff_32v1_send = hydroflow_plus::pusherator::for_each::ForEach::new(|v|{
                                    hoff_32v1_send.give(Some(v));
                                });
                                let op_21v1 = hoff_31v1_recv.map({
                                    use hydroflow_plus_test::__staged::cluster::paxos:: * ;
                                    |(kv_store,highest_seq)|highest_seq
                                });
                                let op_21v1 = {
                                    #[allow(non_snake_case)]
                                    #[inline(always)]
                                    pub fn op_21v1__map__loc_unknown_start_0_0_end_0_0<Item,Input: ::std::iter::Iterator<Item = Item> >(input:Input) -> impl ::std::iter::Iterator<Item = Item>{
                                        #[repr(transparent)]
                                        struct Pull<Item,Input: ::std::iter::Iterator<Item = Item> >{
                                            inner:Input
                                        }
                                        impl <Item,Input: ::std::iter::Iterator<Item = Item> >Iterator for Pull<Item,Input>{
                                            type Item = Item;
                                            #[inline(always)]
                                            fn next(&mut self) -> Option<Self::Item>{
                                                self.inner.next()
                                            }
                                            #[inline(always)]
                                            fn size_hint(&self) -> (usize,Option<usize>){
                                                self.inner.size_hint()
                                            }
                                        
                                            }
                                        Pull {
                                            inner:input
                                        }
                                    }
                                    op_21v1__map__loc_unknown_start_0_0_end_0_0(op_21v1)
                                };
                                #[inline(always)]
                                fn pivot_run_sg_8v1<Pull: ::std::iter::Iterator<Item = Item> ,Push:hydroflow_plus::pusherator::Pusherator<Item = Item> ,Item>(pull:Pull,push:Push){
                                    hydroflow_plus::pusherator::pivot::Pivot::new(pull,push).run();
                                }
                                pivot_run_sg_8v1(op_21v1,hoff_32v1_send);
                            },);
                            df
                        }
                    }
                }else {
                    {
                        //$crate::panicking::panic_fmt($crate::const_format_args!("Invalid node id: {}",__given_id));
                    };
                }
            }
        }
        expand_staged(ports, &f, &num_clients_per_node, &kv_num_keys, &kv_value_size, &i_am_leader_send_timeout, &i_am_leader_check_timeout)
    })(&ports);
    {
        //($crate::format_args_nl!("ack start"));
    };
    hydroflow_plus::util::cli::launch_flow(flow).await
}.await;
}
