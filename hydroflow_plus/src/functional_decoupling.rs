use std::cell::RefCell;

use hydroflow::futures::channel::mpsc::UnboundedSender;
use hydroflow::futures::stream::ForEach;
use stageleft::*;
use syn::meta;

use crate as hydroflow_plus;
use crate::ir::*;
use crate::RuntimeContext;

#[derive(Debug)]
struct FunctionalDecouplingMetadata {
    can_be_decoupled: bool,
    required_codecouplings: Vec<u32>, // IDs of other nodes that must also be decoupled
}

fn functional_analysis_node<'a>(
    node: HfPlusNode,
    _context: RuntimeContext<'a>,
    id: &mut u32,
    seen_tees: &mut SeenTees,
    func_dec_metadata_map: &mut HashMap<u32, FunctionalDecouplingMetadata>,
    prev_node_metadata: FunctionalDecouplingMetadata,
) -> HfPlusNode {
    let my_id = *id;
    *id += 1;

    let metadata = FunctionalDecouplingMetadata {
        
    };

    let child = node.transform_children(
        |node, seen_tees| {
            functional_analysis_node(node, _context, id, seen_tees, func_dec_metadata_map, metadata)
        },
        seen_tees,
    );
}

/// Count the cardinality of each input and periodically output to a file
pub fn functional_analysis<'a>(
    ir: Vec<HfPlusLeaf>,
    context: RuntimeContext<'a>,
    func_dec_metadata_map: &mut HashMap<u32, FunctionalDecouplingMetadata>,
) -> Vec<HfPlusLeaf> {
    let mut id = 0;
    let mut seen_tees = Default::default();
    ir.into_iter()
        .map(|l| {
            // Each leaf has a different ID
            id += 1;
            let metadata = FunctionalDecouplingMetadata {
                num_children: 0,
                reduced_load: 0,
            };
            func_dec_metadata_map.insert(id, metadata);
            // Recurse through children
            l.transform_children(
                |node, seen_tees| {
                    functional_analysis_node(node, context, &mut id, seen_tees, func_dec_metadata_map, metadata)
                },
                &mut seen_tees,
            )
        })
        .collect()
}

#[stageleft::runtime]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use stageleft::*;

    use crate::{MultiGraph, __staged::functional_decoupling::FunctionalDecouplingMetadata};

    #[test]
    fn functional_decoupling_broadcast() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let sender = flow.process(&());
        let receiver  = flow.cluster(&());

        // Cycle (can't be decoupled)
        let (new_nums_complete_cycle, new_nums) = flow.cycle(&sender);
        let nums = flow.source_iter(&sender, q!(0..10))
            .union(new_nums)
            .all_ticks();
        let nums_plus_one = nums.map(q!(|v| v + 1));
        new_nums_complete_cycle.complete(nums_plus_one);
        // Functional stuff after cycle, should be decoupled
        let nums_times_two = nums.map(q!(|v| v * 2))
            .filter(q!(|v| v % 3 == 0))
            .broadcast_bincode(&receiver);

        let runtime_context = flow.runtime_context();
        let built = flow.extract();

        insta::assert_debug_snapshot!(&built.ir);

        // Build map of which nodes can be functionally decoupled
        let mut func_dec_metadata_map = HashMap::<u32, FunctionalDecouplingMetadata>::new();
        built.optimize_with(|ir| super::functional_analysis(ir, runtime_context, &mut func_dec_metadata_map));

        println!("{:?}", func_dec_metadata_map);
    }
}
