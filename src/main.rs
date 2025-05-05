// loads data, builds the graph, computes metrics, prints results, and tests metrics
mod load;     // Module for loading and deserializing train data from CSV
mod graph;    // Module for defining and constructing the transit graph
mod metrics;  // Module for centrality and route delay metrics

use load::load_data; // Function to read CSV data into TrainRecords
use graph::TransitGraph; // Transit network graph implementation

fn main() {
    let path = "src/stations_filtered.csv"; 
    let records = load_data(path).expect("Failed to load data"); 
    let graph = TransitGraph::from_records(&records); 
    // Print ranked stations by closeness centrality (top 10)
    graph.rank_stations_by_closeness(10);
    // Print ranked stations by betweenness centrality (top 10)
    graph.rank_stations_by_betweenness(10);
    // Print top 10 routes with highest average delay
    graph.rank_routes_by_average_delay(10);
    // Print top 10 routes with lowest average delay
    graph.rank_routes_by_lowest_delay(10);
}

// Unit test: ensure real data loads and contains a large number of records
#[test]
fn test_load_real_data() {
    let path = "src/stations_filtered.csv";
    let records = load_data(path).expect("Could not load data");
    assert!(records.len() > 1000);
}

// Unit test: ensure a valid shortest path exists between two key stations
#[test]
fn test_real_shortest_path_exists() {
    let path = "src/stations_filtered.csv";
    let records = load_data(path).expect("Could not load data");
    let graph = TransitGraph::from_records(&records);
    let from = "New York Penn Station".to_string();
    let to = "Newark Broad Street".to_string();
    let result = graph.shortest_path(&from, &to);
    assert!(result.is_some());
    if let Some((delay, path)) = result {
        assert!(delay >= 0.0);
        assert!(path.contains(&from));
        assert!(path.contains(&to));
    }
}

// Unit test: check that closeness centrality for a major station is valid and finite
#[test]
fn test_closeness_is_finite_for_main_station() {
    let path = "src/stations_filtered.csv";
    let records = load_data(path).expect("Could not load data");
    let graph = TransitGraph::from_records(&records);
    let station = "Walnut Street".to_string(); 
    let score = graph.closeness_centrality(&station);
    assert!(score.is_some());
    assert!(score.unwrap().is_finite());
}

// Unit test: verify that all betweenness scores are non-negative
#[test]
fn test_betweenness_non_negative() {
    let records = load_data("src/stations_filtered.csv").expect("Failed to load CSV");
    let graph = TransitGraph::from_records(&records);
    let centrality = graph.betweenness_centrality();
    for (station, score) in centrality {
        assert!(score >= 0.0, "{} has negative betweenness score", station);
    }
}

// Unit test: ensure that route delays are sorted in descending order by average delay
#[test]
fn test_rank_routes_by_average_delay_sorted_descending() {
    let records = load_data("src/stations_filtered.csv").expect("Failed to load CSV");
    let graph = TransitGraph::from_records(&records);
    let mut averages = graph.get_route_average_delays();
    averages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for i in 1..averages.len() {
        assert!(
            averages[i - 1].1 >= averages[i].1,
            "Route delay not sorted descending at index {}: {} < {}",
            i,
            averages[i - 1].1,
            averages[i].1
        );
    }
}
// end of main.rs