use std::{collections::HashMap, rc::Rc};
use crate::ir::*;
use syn::Ident;

fn monotonic_detection_called_fn_leaf(
    node: HfPlusLeaf,
    cycle_sink_ident_map: &mut HashMap<Ident, bool>,
) {
        
                println!(
                    "node: {:?}, \nmonotonic: {}",
                    node,
                    node.clone().is_my_output_set_monotonic(cycle_sink_ident_map)
                );
}

fn monotonic_detection_called_fn_node(
    node: HfPlusNode,
    cycle_sink_ident_map: &mut HashMap<Ident, bool>,
) {
        
    println!(
        "node: {:?}, \nmonotonic: {}",
        node,
        node.clone().is_my_output_set_monotonic(cycle_sink_ident_map)
    );
}


#[cfg(test)]
mod tests {
    use hydroflow::lattices::cc_traits::MapInsert;
    use stageleft::*;
    use crate::{MultiGraph, __staged::cycle};
    use super::*;

    #[test]
    fn is_my_output_set_monotonic_source_union_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let source1 = flow.source_iter(&process, q!(0..2));
        let source2 = flow.source_iter(&process, q!(3..4)).union(source1).for_each(q!(|n| println!("{}", n)));
        let built = flow.extract();
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        println!("checking each node is monotonic or not: ");
        println!("Expected output: NO");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }

    #[test]
    fn is_my_output_set_monotonic_source_and_map_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());
        // source -> map -> for_each
        let source = flow.source_iter(&process, q!(0..2));
        let ret = source.map(q!(|i| i + 1)).for_each(q!(|n| println!("{}", n)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: NO");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }

    #[test]
    fn is_my_output_set_monotonic_source_and_persist_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());
        // source -> all_ticks -> for_each
        let source = flow.source_iter(&process, q!(0..2));
        let ret = source.all_ticks().for_each(q!(|n| println!("{}", n)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: Yes after Persist");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }
    

    #[test]
    fn is_my_output_set_monotonic_source_and_clone_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let source = flow.source_iter(&process, q!(0..2));
        let ret = source.clone().for_each(q!(|n| println!("{}", n)));
        let ret2 = source.for_each(q!(|n| println!("{}", n)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: Yes after Persist");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }

    #[test]
    fn is_my_output_set_monotonic_source_persist_and_flatmap_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let source = flow.source_iter(&process, q!(vec![0..2]));
        let persisted = source.all_ticks();
        persisted.flat_map(q!(|i| i)).for_each(q!(|n| println!("{}", n)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: Yes after Persist");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }

    
    #[test]
    fn is_my_output_set_monotonic_source_and_cycle_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let (cycle_complete, cycle) = flow.cycle(&process);
        let source = flow.source_iter(&process, q!(0..2));
        cycle_complete.complete(source);
        let ret = cycle.for_each(q!(|n| println!("{}", n)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: NO");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }

    #[test]
    fn is_my_output_set_monotonic_source_and_persist_cycle_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());

        let (cycle_complete, cycle) = flow.cycle(&process);
        let source = flow.source_iter(&process, q!(0..2));
        cycle_complete.complete(source);
        let ret = cycle.all_ticks().for_each(q!(|n| println!("{}", n)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: Yes after persist");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }

    #[test]
    fn is_my_output_set_monotonic_source_filter_fold_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());
        let source = flow.source_iter(&process, q!(0..2));
        let filtered = source.filter(q!(|i| *i < 30));
        let folded = filtered.fold(q!(|| 0), q!(|cur, incoming| *cur += 1));
        folded.for_each(q!(|i| println!("{}",i)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: NO");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }

    #[test]
    fn is_my_output_set_monotonic_source_filter_persist_map_filter_fold_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());
        let source = flow.source_iter(&process, q!(0..15));
        let filtered = source.filter(q!(|i| *i < 30));
        let persisted = filtered.all_ticks();
        let mapped = persisted.map(q!(|x| x+1));
        let filtered_2 = mapped.filter(q!(|x| *x < 10));
        let folded = filtered_2.fold(q!(|| 0), q!(|cur, incoming| *cur += 1));
        folded.for_each(q!(|i| println!("{}",i)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: Yes after persist, No after fold");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }

    #[test]
    fn is_my_output_set_monotonic_source_persist_cycle_test() {
        let flow = crate::builder::FlowBuilder::<MultiGraph>::new();
        let process = flow.process(&());
        let source: crate::Stream<i32, crate::stream::Windowed, crate::location::MultiNode> = flow.source_iter(&process, q!(0..15));
        let persisted = source.all_ticks();
        let (cycle_complte, cycle) = flow.cycle(&process);
        cycle_complte.complete(persisted);
        cycle.for_each(q!(|i| println!("{}",i)));
        let mut cycle_sink_ident_map: HashMap<Ident, bool> = HashMap::new();

        let built = flow.extract();
        println!("checking each node is monotonic or not: ");
        println!("Expected output: Yes after persist");
        for node in built.ir.clone() {
            node.apply_function_to_all_nodes(
                &monotonic_detection_called_fn_leaf,
                &monotonic_detection_called_fn_node,
                &mut cycle_sink_ident_map,
            );
        }
    }


}
