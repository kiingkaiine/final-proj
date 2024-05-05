extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate plotters;
extern crate petgraph;

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use petgraph::Direction;
use crate::FlightRecord;

pub fn construct_graph(records: &[FlightRecord]) -> DiGraph<String, u32> {
    let mut graph = DiGraph::new();

    let mut edge_weights: HashMap<(String, String), u32> = HashMap::new();

    for record in records {
        match &record.activity_type[..] {
            "Enplaned" => {
                let origin_region = String::from("US");
                let destination_region = record.geo_region.clone();
                let edge_key = (origin_region.clone(), destination_region.clone());

                if let Some(weight) = edge_weights.get_mut(&edge_key) {
                    *weight += record.passenger_count;
                } else {
                    let origin_node = graph.add_node(origin_region.clone());
                    let destination_node = graph.add_node(destination_region.clone());
                    let edge_weight = record.passenger_count;
                    graph.add_edge(origin_node, destination_node, edge_weight);
                    edge_weights.insert(edge_key, edge_weight);
                }
            }
            "Deplaned" => {
                let origin_region = record.geo_region.clone();
                let destination_region = String::from("US");
                let edge_key = (origin_region.clone(), destination_region.clone());

                if let Some(weight) = edge_weights.get_mut(&edge_key) {
                    *weight += record.passenger_count;
                } else {
                    let origin_node = graph.add_node(origin_region.clone());
                    let destination_node = graph.add_node(destination_region.clone());
                    let edge_weight = record.passenger_count;
                    graph.add_edge(origin_node, destination_node, edge_weight);
                    edge_weights.insert(edge_key, edge_weight);
                }
            }
            "Thru/Transit" => {
                let origin_region = String::from("US");
                let destination_region = String::from("US");
                let edge_key = (origin_region.clone(), destination_region.clone());

                if let Some(weight) = edge_weights.get_mut(&edge_key) {
                    *weight += record.passenger_count;
                } else {
                    let origin_node = graph.add_node(origin_region.clone());
                    let destination_node = graph.add_node(destination_region.clone());
                    let edge_weight = record.passenger_count;
                    graph.add_edge(origin_node, destination_node, edge_weight);
                    edge_weights.insert(edge_key, edge_weight);
                }
            }
            _ => continue, // Skip records with unknown activity type
        };
    }

    graph
}




pub fn analyze_centrality(graph: &DiGraph<String, u32>) {
    // Calculate degree centrality for each node
    let degree_centrality: HashMap<NodeIndex, f64> = graph
        .node_indices()
        .map(|node| {
            let sum_weights: u32 = graph
                .edges_directed(node, Direction::Outgoing)
                .map(|edge| *edge.weight())
                .sum();
            (node, sum_weights as f64 / (graph.node_count() - 1) as f64)
        })
        .collect();

    println!("Degree Centrality:");
    for (node, centrality) in &degree_centrality {
        println!("Node {}: {}", node.index(), centrality);
    }

}