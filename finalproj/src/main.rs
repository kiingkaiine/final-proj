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

    for record in records {
        let origin_region = String::from("US"); // Origin is always US
        let destination_region = record.geo_region.clone();

        let origin_node = graph.add_node(origin_region);
        let destination_node = graph.add_node(destination_region);

        // Add edge representing the flight
        let edge_weight = record.passenger_count;
        graph.add_edge(origin_node, destination_node, edge_weight);
    }

    graph
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
    println!("Edges:");
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        println!("Edge: {} -> {}, Weight: {}", source.index(), target.index(), graph[edge]);
    }

    Ok(())
}
