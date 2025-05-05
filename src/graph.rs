// Defines the transit graph structure and builds it from the records.
use std::collections::HashMap;
use crate::load::TrainRecord;
// Type alias for station name
pub type Station = String;
// Type alias for a weighted edge between stations with delay as weight
pub type WeightedEdge = (Station, Station, f32);
// Represents a transit network graph with stations and delays as weighted edges
#[derive(Debug)]
pub struct TransitGraph {
    pub nodes: HashMap<Station, Vec<(Station, f32)>>, // Map from station to list of destination stations with delay
}
impl TransitGraph {
    // Constructs a TransitGraph from a slice of TrainRecords
    // Input: slice of TrainRecord structs
    // Output: TransitGraph with nodes populated by delay-weighted edges
    // Logic: Filter records with delay data, then insert edges into graph map
    pub fn from_records(records: &[TrainRecord]) -> Self {
        let mut nodes: HashMap<Station, Vec<(Station, f32)>> = HashMap::new(); // Initialize graph
        // Iterate over records with valid delay data
        for r in records.iter().filter(|r| r.delay_minutes.is_some()) {
            let from = r.from.clone(); // Source station
            let to = r.to.clone();     // Destination station
            let delay = r.delay_minutes.unwrap(); // Extract delay value
            // Insert or update edge from -> to with delay
            nodes.entry(from.clone()).or_default().push((to.clone(), delay));
        }

        Self { nodes } // Return constructed graph
    }
}
