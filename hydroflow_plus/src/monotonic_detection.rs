use crate::ir::*;

struct Edge {
    source: HfPlusNode,
    target: HfPlusNode, 
    is_monotonic: bool,
}



fn monotonic_detection_node(node: HfPlusNode, seen_tees: &mut SeenTees) -> HfPlusNode {
    match node.transform_children(monotonic_detection_node, seen_tees) {
        HfPlusNode::Source{
            source: HfPlusSource,
            location_id: usize,
        } => {
            println!("I'm source, not monotonic. All op except Persist behind me are not monotonic.");
            HfPlusNode::Source {
                source: HfPlusSource.clone(),
                location_id: usize,
            }
        },
        HfPlusNode::Persist(inner) => {
            // Create a new HfPlusNode::Persist with the transformed input
            println!("hey, it is persist");
            HfPlusNode::Persist(inner)
        },
        o => {
            println!("current op: {:?}", o);
            o
        },
    }
}

// pub fn monotonic_detection(ir: Vec<HfPlusLeaf>) -> Vec<HfPlusGraphNode> {
//     let mut seen_tees = Default::default();
//     ir.into_iter()
//         .flat_map(|l| l.create_inverted_graph(&mut seen_tees)) // Use `flat_map` to flatten the results
//         .map(|rc_node| Rc::try_unwrap(rc_node).ok().unwrap().into_inner()) // Unwrap the Rc<RefCell<_>> to get the inner HfPlusGraphNode
//         .collect()
// }


#[cfg(test)]
mod tests {
    use stageleft::*;
    use std::rc::Rc;
    use crate::MultiGraph;

    use super::HfPlusGraphNode;

    #[test]
    fn invert_graph_union_and_source_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let source1 = flow.source_iter(&process, q!(0..2));
        let source2 = flow.source_iter(&process, q!(3..4)).union(source1).for_each(q!(|n| println!("{}", n)));

        let built = flow.extract();
        println!("Original Graph: [");
        for node in built.ir.clone() {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
        // insta::assert_debug_snapshot!(built.ir());
        let mut seen_tees = Default::default();
        let source: Vec<HfPlusGraphNode> = built.ir.into_iter()
        .flat_map(|l| l.create_inverted_graph(&mut seen_tees)) // Use `flat_map` to flatten the results
        .map(|rc_node| Rc::try_unwrap(rc_node).ok().unwrap().into_inner()) // Unwrap the Rc<RefCell<_>> to get the inner HfPlusGraphNode
        .collect(); // Now the compiler knows to collect into Vec<HfPlusGraphNode>
        println!("Result in Graph: [");
        // Debug print the resulting graph
        for node in &source {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
    }

    #[test]
    fn invert_graph_source_and_map_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let source = flow.source_iter(&process, q!(0..2));
        let ret = source.map(q!(|i| i + 1)).for_each(q!(|n| println!("{}", n)));

        let built = flow.extract();
        println!("Original Graph: [");
        for node in built.ir.clone() {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
        let mut seen_tees = Default::default();
        let source: Vec<HfPlusGraphNode> = built.ir.into_iter()
        .flat_map(|l| l.create_inverted_graph(&mut seen_tees)) // Use `flat_map` to flatten the results
        .map(|rc_node| Rc::try_unwrap(rc_node).ok().unwrap().into_inner()) // Unwrap the Rc<RefCell<_>> to get the inner HfPlusGraphNode
        .collect(); // Now the compiler knows to collect into Vec<HfPlusGraphNode>
        println!("Result in Graph: [");
        // Debug print the resulting graph
        for node in &source {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
    }

    #[test]
    fn invert_graph_source_and_flatmap_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let source = flow.source_iter(&process, q!(vec![0..2]));
        let ret = source.flat_map(q!(|i| i)).for_each(q!(|n| println!("{}", n)));

        let built = flow.extract();
        println!("Original Graph: [");
        for node in built.ir.clone() {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
        let mut seen_tees = Default::default();
        let source: Vec<HfPlusGraphNode> = built.ir.into_iter()
        .flat_map(|l| l.create_inverted_graph(&mut seen_tees)) // Use `flat_map` to flatten the results
        .map(|rc_node| Rc::try_unwrap(rc_node).ok().unwrap().into_inner()) // Unwrap the Rc<RefCell<_>> to get the inner HfPlusGraphNode
        .collect(); // Now the compiler knows to collect into Vec<HfPlusGraphNode>
    
        // Debug print the resulting graph
        println!("Result in Graph: [");
        // Debug print the resulting graph
        for node in &source {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
    }

    #[test]
    fn invert_graph_source_and_clone_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let source = flow.source_iter(&process, q!(0..2));
        let ret = source.clone().for_each(q!(|n| println!("{}", n)));
        let ret2 = source.for_each(q!(|n| println!("{}", n)));

        let built = flow.extract();
        println!("Original Graph: [");
        for node in built.ir.clone() {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
        let mut seen_tees = Default::default();
        let source: Vec<HfPlusGraphNode> = built.ir.into_iter()
        .flat_map(|l| l.create_inverted_graph(&mut seen_tees)) // Use `flat_map` to flatten the results
        .map(|rc_node| Rc::try_unwrap(rc_node).ok().unwrap().into_inner()) // Unwrap the Rc<RefCell<_>> to get the inner HfPlusGraphNode
        .collect(); // Now the compiler knows to collect into Vec<HfPlusGraphNode>
    
        println!("Result in Graph: [");
        // Debug print the resulting graph
        for node in &source {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
    }

    #[test]
    fn invert_graph_source_and_persist_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let source = flow.source_iter(&process, q!(0..2));
        let ret = source.all_ticks().for_each(q!(|n| println!("{}", n)));

        let built = flow.extract();
        println!("Original Graph: [");
        for node in built.ir.clone() {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
        let mut seen_tees = Default::default();
        let source: Vec<HfPlusGraphNode> = built.ir.into_iter()
        .flat_map(|l| l.create_inverted_graph(&mut seen_tees)) // Use `flat_map` to flatten the results
        .map(|rc_node| Rc::try_unwrap(rc_node).ok().unwrap().into_inner()) // Unwrap the Rc<RefCell<_>> to get the inner HfPlusGraphNode
        .collect(); // Now the compiler knows to collect into Vec<HfPlusGraphNode>
    
        println!("Result in Graph: [");
        // Debug print the resulting graph
        for node in &source {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
    }

    #[test]
    fn invert_graph_source_and_cycle_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let (cycle_complete, cycle) = flow.cycle(&process);
        let source = flow.source_iter(&process, q!(0..2));
        cycle_complete.complete(source);
        let ret = cycle.all_ticks().for_each(q!(|n| println!("{}", n)));

        let built = flow.extract();
        println!("Original Graph: [");
        for node in built.ir.clone() {
            println!("{}", node);
            println!(" ");
        }
        println!("]");        let mut seen_tees = Default::default();
        let source: Vec<HfPlusGraphNode> = built.ir.into_iter()
        .flat_map(|l| l.create_inverted_graph(&mut seen_tees)) // Use `flat_map` to flatten the results
        .map(|rc_node| Rc::try_unwrap(rc_node).ok().unwrap().into_inner()) // Unwrap the Rc<RefCell<_>> to get the inner HfPlusGraphNode
        .collect(); // Now the compiler knows to collect into Vec<HfPlusGraphNode>
    
        println!("Result in Graph: [");
        // Debug print the resulting graph
        for node in &source {
            println!("{}", node);
            println!(" ");
        }
        println!("]");
    }

    #[test]
    fn invert_graph_source_filter_fold_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());
        let source = flow.source_iter(&process, q!(0..2));
        let filtered = source.filter(q!(|i| *i < 30));
        let folded = filtered.fold(q!(|| 0), q!(|cur, incoming| *cur += 1));
        folded.for_each(q!(|i| println!("{}",i)));

        let built = flow.extract();
        println!("Graph did not inverted: {:?}", built.ir);
        let mut seen_tees = Default::default();
        let source: Vec<HfPlusGraphNode> = built.ir.into_iter()
        .flat_map(|l| l.create_inverted_graph(&mut seen_tees)) // Use `flat_map` to flatten the results
        .map(|rc_node| Rc::try_unwrap(rc_node).ok().unwrap().into_inner()) // Unwrap the Rc<RefCell<_>> to get the inner HfPlusGraphNode
        .collect(); // Now the compiler knows to collect into Vec<HfPlusGraphNode>
    
        // Debug print the resulting graph
        for node in &source {
            println!("Result in graph: {}", node);
        }
    }


    // #[test]
    // fn monotonic_detection_through_map() {
    //     let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
    //     let process = flow.process(&());

    //     let source1 = flow.source_iter(&process, q!(0..2));
    //     let source2 = flow.source_iter(&process, q!(3..4)).union(source1).for_each(q!(|n| println!("{}", n)));

    //     let built = flow.extract();

    //     // insta::assert_debug_snapshot!(built.ir());
    //     let optimized = built.optimize_with(super::monotonic_detection);
    //     // insta::assert_debug_snapshot!(optimized.ir());
    //     // for (id, graph) in optimized.no_optimize().hydroflow_ir() {
    //     //     insta::with_settings!({snapshot_suffix => format!("surface_graph_{id}")}, {
    //     //         insta::assert_display_snapshot!(graph.surface_syntax_string());
    //     //     });
    //     // }
    // }

    // #[test]
    // fn monotonic_detection_behind_tee() {
    //     let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
    //     let process = flow.process(&());

    //     let before_tee = flow
    //         .source_iter(&process, q!(0..10))
    //         .all_ticks()
    //         .map(q!(|v| v + 1));

    //     before_tee.clone().for_each(q!(|n| println!("{}", n)));

    //     before_tee.for_each(q!(|n| println!("{}", n)));

    //     let built = flow.extract();

    //     insta::assert_debug_snapshot!(built.ir());

    //     let optimized = built.optimize_with(super::monotonic_detection);

    //     insta::assert_debug_snapshot!(optimized.ir());

    //     for (id, graph) in optimized.no_optimize().hydroflow_ir() {
    //         insta::with_settings!({snapshot_suffix => format!("surface_graph_{id}")}, {
    //             insta::assert_display_snapshot!(graph.surface_syntax_string());
    //         });
    //     }
    // }
}
