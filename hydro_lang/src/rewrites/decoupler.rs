use std::cell::RefCell;
use std::collections::HashSet;

use crate::ir::*;
use crate::location::LocationId;

pub struct Decoupler {
    pub nodes_to_decouple: Vec<usize>,
    pub num_machines: usize,
    pub parent_location: LocationId,
    pub location: LocationId,

    pub curr_node_id: usize,
}

fn decouple_node(
    node: &mut HydroNode,
    decoupler: &mut Decoupler,
) {
    
}

impl Decoupler {
    pub fn decouple(&mut self, ir: Vec<HydroLeaf>) -> Vec<HydroLeaf> {
        ir.into_iter()
            .map(|l| {
                l.transform_children(
                    |n, s| n.transform_bottom_up(decouple_node, s, self),
                    &mut Default::default(),
                )
            })
            .collect()
    }
}
