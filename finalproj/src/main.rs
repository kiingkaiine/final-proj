extern crate csv;
extern crate serde;
extern crate serde_json;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct FlightRecord {
    #[serde(rename = "operating_airline")]
    operating_airline: String,
    #[serde(rename = "geo_region")]
    geo_region: String,
    #[serde(rename = "activity_type")]
    activity_type: String,
    #[serde(rename = "terminal")]
    terminal: String,
    #[serde(rename = "passenger_count")]
    passenger_count: u32,
}

fn read_csv(file_path: &str) -> Result<Vec<FlightRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: FlightRecord = result?;
        records.push(record);
    }

    Ok(records)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Air_Traffic_Passenger_Statistics.csv";
    let flight_records = read_csv(file_path)?;

    // Now you have flight_records containing relevant fields from the CSV
    println!("{:?}", flight_records);

    Ok(())
}
