extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate plotters;
extern crate petgraph;

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use plotters::prelude::*;
use petgraph::Direction;
use petgraph::algo::{connected_components, dijkstra};
use petgraph::visit::NodeCount;
use petgraph::prelude::EdgeRef;

#[derive(Debug, Deserialize)]
struct FlightRecord {
    geo_region: String,
    activity_type: String,
    passenger_count: u32,
}

fn read_csv(file_path: &str) -> Result<Vec<FlightRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let mut record: FlightRecord = result?;
        match &record.activity_type[..] {
            "Enplaned" => {
                record.geo_region = record.geo_region.clone();
            }
            "Deplaned" => {
                record.geo_region = String::from("US");
            }
            "Thru/Transit" => {
                record.geo_region = String::from("US");
            }
            _ => continue, // Skip records with unknown activity type
        };
        records.push(record);
    }
    Ok(records)
}

fn construct_graph(records: &[FlightRecord]) -> DiGraph<String, u32> {
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




fn analyze_centrality(graph: &DiGraph<String, u32>) {
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

    // You can similarly calculate other centrality measures like betweenness and closeness centrality
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Air_Traffic_Passenger_Statistics.csv";
    let flight_records = read_csv(file_path)?;

    let graph = construct_graph(&flight_records);

    // Perform further analysis on the graph...
    // Print out the nodes and their properties
    
    println!("Nodes:");
    for node in graph.node_indices() {
        println!("Node: {}, Region: {}", node.index(), graph[node]);
    }

    // Print out the edges and their properties
    //println!("Edges:");
    //for edge in graph.edge_indices() {
        //let (source, target) = graph.edge_endpoints(edge).unwrap();
        //println!("Edge: {} -> {}, Weight: {}", source.index(), target.index(), graph[edge]);
    //}

    // Analyze centrality
    analyze_centrality(&graph);
    
    Ok(())
}
