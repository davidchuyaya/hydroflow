use stageleft::q;

use crate::{ir::*, ClusterId};
use crate::location::LocationId;
use crate::location::cluster::CLUSTER_SELF_ID;
use crate::stream::{deserialize_bincode_with_type, serialize_bincode_with_type};

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
    let metadata = node.metadata();
    if let Some(id) = metadata.id {
        if decoupler.nodes_to_decouple.contains(&id) {
            if metadata.location_kind != decoupler.parent_location {
                panic!("decouple_node expected parent location {:?}, got {:?}", decoupler.parent_location, metadata.location_kind);
            }
            let output_type = metadata.output_type.clone().unwrap();
            let is_demux = decoupler.num_machines > 1;

            // If parent is a cluster, find the ID and send the message to the decoupled node with the same ID
            if is_demux {
                if let LocationId::Cluster(parent_id) = metadata.location_kind {
                    let map_metadata = HydroNodeMetadata {
                        id: None, // TODO: Pick a random new ID
                        location_kind: decoupler.parent_location.clone(),
                        output_type: Some(output_type.clone()),
                    };
                    // TODO: Fixes to typecheck
                    let f = q!(move |b| (
                        ClusterId::from_raw(CLUSTER_SELF_ID.raw_id),
                        b.clone()
                    )).splice_fn1_ctx(&decoupler.parent_location).into();

                    let node_content = std::mem::replace(node, HydroNode::Placeholder);
                    let mapped_node = HydroNode::Map { 
                        f,
                        input: Box::new(node_content), 
                        metadata: map_metadata,
                    };
                    *node = mapped_node;
                }
                else {
                    panic!("Expected parent location to be a cluster, got {:?}", metadata.location_kind);
                }
            }

            // Set up the network node
            let network_metadata = HydroNodeMetadata {
                id: None, // TODO: Pick a random new ID
                location_kind: decoupler.location.clone(),
                output_type: Some(output_type.clone()),
            };
            let node_content = std::mem::replace(node, HydroNode::Placeholder);
            let network_node = HydroNode::Network {
                from_location: metadata.location_kind,
                from_key: None,
                to_location: decoupler.location.clone(),
                to_key: None,
                serialize_fn: Some(serialize_bincode_with_type(is_demux, output_type.clone())).map(|e| e.into()),
                instantiate_fn: DebugInstantiate::Building(),
                deserialize_fn: Some(deserialize_bincode_with_type(None, output_type.clone())).map(|e| e.into()),
                input: Box::new(node_content),
                metadata: network_metadata,
            };

            *node = network_node;
        }
    }
}

impl Decoupler {
    pub fn decouple(&mut self, ir: Vec<HydroLeaf>) -> Vec<HydroLeaf> {
        // TODO: Create the new location?
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
