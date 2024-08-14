use crate::ir::*;


fn invert_graph()

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

pub fn monotonic_detection(ir: Vec<HfPlusLeaf>) -> Vec<HfPlusLeaf> {
    let mut seen_tees = Default::default();
    ir.into_iter()
        .map(|l| l.transform_children(monotonic_detection_node, &mut seen_tees))
        .collect()

}


#[cfg(test)]
mod tests {
    use stageleft::*;

    use crate::MultiGraph;

    #[test]
    fn monotonic_detection_through_map() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        flow.source_iter(&process, q!(0..10))
            .all_ticks()
            .map(q!(|v| v + 1))
            .for_each(q!(|n| println!("{}", n)));

        let built = flow.extract();

        // insta::assert_debug_snapshot!(built.ir());
        let optimized = built.optimize_with(super::monotonic_detection);
        // insta::assert_debug_snapshot!(optimized.ir());
        // for (id, graph) in optimized.no_optimize().hydroflow_ir() {
        //     insta::with_settings!({snapshot_suffix => format!("surface_graph_{id}")}, {
        //         insta::assert_display_snapshot!(graph.surface_syntax_string());
        //     });
        // }
    }

    #[test]
    fn monotonic_detection_behind_tee() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let before_tee = flow
            .source_iter(&process, q!(0..10))
            .all_ticks()
            .map(q!(|v| v + 1));

        before_tee.clone().for_each(q!(|n| println!("{}", n)));

        before_tee.for_each(q!(|n| println!("{}", n)));

        let built = flow.extract();

        insta::assert_debug_snapshot!(built.ir());

        let optimized = built.optimize_with(super::monotonic_detection);

        insta::assert_debug_snapshot!(optimized.ir());

        for (id, graph) in optimized.no_optimize().hydroflow_ir() {
            insta::with_settings!({snapshot_suffix => format!("surface_graph_{id}")}, {
                insta::assert_display_snapshot!(graph.surface_syntax_string());
            });
        }
    }
}
