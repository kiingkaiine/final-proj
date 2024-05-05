extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate plotters;
extern crate petgraph;

use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::collections::HashSet;

use petgraph::Direction;
use crate::FlightRecord;

pub fn construct_graph(records: &[FlightRecord]) -> DiGraph<String, u32> {
    let mut graph = DiGraph::new();

    let _edge_weights: HashMap<(String, String), u32> = HashMap::new();

    for record in records {
        let origin_region = record.geo_region.clone();
        let destination_region = match &record.activity_type[..] {
            "Enplaned" => String::from("US"),
            "Deplaned" => String::from("US"),
            "Thru/Transit" => String::from("US"),
            _ => continue, // Skip records with unknown activity type
        };
    
        let origin_node = graph.add_node(origin_region.clone());
        let destination_node = graph.add_node(destination_region.clone());
        let edge_weight = record.passenger_count;
        graph.add_edge(origin_node, destination_node, edge_weight);
    }

    graph
}




pub fn analyze_centrality(graph: &DiGraph<String, u32>) {
    // Calculate degree centrality for each node
    let mut centrality_scores: HashMap<String, f64> = HashMap::new();
    let mut processed_regions: HashSet<String> = HashSet::new();

    for node in graph.node_indices() {
        let region = &graph[node];
        if processed_regions.contains(region) {
            continue;
        }

        let total_weight: f64 = graph
            .edges_directed(node, Direction::Incoming)
            .chain(graph.edges_directed(node, Direction::Outgoing))
            .map(|edge| *edge.weight() as f64)
            .sum();

        let mut count: f64 = 1.0; // Start with 1 to include the node itself
        for other_node in graph.node_indices() {
            if graph[other_node] == *region && other_node != node {
                count += 1.0;
            }
        }

        let centrality = total_weight / (count - 1.0); // Subtract 1 to exclude the node itself
        centrality_scores.insert(region.clone(), centrality);
        processed_regions.insert(region.clone());
    }

    println!("Centrality Scores:");
    for (region, centrality) in &centrality_scores {
        println!("Region {}: Centrality: {}", region, centrality);
    }
}