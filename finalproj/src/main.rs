extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate plotters;
extern crate petgraph;

use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use serde::Deserialize;
mod georeg;
mod predcount;

//The structure for each flight record (line) in the CSV
#[derive(Debug, Deserialize)]
struct FlightRecord {
    geo_region: String,
    activity_type: String,
    passenger_count: u32,
    activity_period: String,
}
//Reads my CSV file passed from main
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


fn main() -> Result<(), Box<dyn Error>> {

    //Read CSV
    let file_path = "Air_Traffic_Passenger_Statistics.csv";
    let flight_records = read_csv(file_path)?;

    let graph = georeg::construct_graph(&flight_records);
    //Analyze centrality
    georeg::analyze_centrality(&graph);

    //Fit linear regression model
    let predicted_passenger_counts = predcount::predict_passenger_counts_by_month(&flight_records);

    //Print the results of linear regression
    println!("Predicted Passenger Counts by Month:");
    for (month, count) in &predicted_passenger_counts {
        println!("{}: {}", month, count);
    }
    Ok(())
    
}
