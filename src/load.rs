//  Loads and deserializes the dataset

use serde::Deserialize;
use std::error::Error;
use csv::ReaderBuilder;

// Represents a single train record from the dataset with metadata including delay and routing
#[derive(Debug, Deserialize)]
pub struct TrainRecord {
    pub date: String,// Date of the train record
    pub train_id: String,// Identifier for the train
    pub stop_sequence: String, // Stop sequence number
    pub from: String, // Departure station name
    pub from_id: String,// Departure station ID
    pub to: String,// Arrival station name
    pub to_id: String,// Arrival station ID
    pub scheduled_time: String,// Scheduled time for the trip
    pub actual_time: String,// Actual arrival/departure time
    pub delay_minutes: Option<f32>, // Delay in minutes, optional
    pub status: String,// Status of the train
    pub line: String,// Line name
    pub r#type: String,// Train type (e.g. Local, Express)
    pub month: String,// Month of the record
    pub year: String,// Year of the record
}

// Loads and parses CSV data into a vector of TrainRecord structs
// Input: path to CSV file as &str
// Output: Result with either vector of TrainRecord or error
// Logic: Build CSV reader, iterate through records, deserialize each line into TrainRecord and collect
pub fn load_data(path: &str) -> Result<Vec<TrainRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?; 
    let mut records = Vec::new(); 
    for result in rdr.deserialize(){ 
        let record: TrainRecord = result?; // Deserialize line into TrainRecord struct
        records.push(record) // Append to records vector
    }
    Ok(records) 
}
