// Contains algorithms to compute graph metrics like shortest paths, closeness, betweenness, and delay ranking

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use ordered_float::NotNan;
use crate::graph::{TransitGraph, Station};
use std::collections::{HashSet, VecDeque};

impl TransitGraph {
    // Returns a set of all unique stations in the graph
    pub fn all_stations(&self) -> HashSet<Station> {
        let mut stations = HashSet::new();
        for (from, neighbors) in &self.nodes {
            stations.insert(from.clone());
            for (to, _) in neighbors {
                stations.insert(to.clone());
            }
        }
        stations
    }

    // Computes the shortest path (by total delay) from start to end station using Dijkstra’s algorithm
    // Input: start and end station names
    // Output: Option<(total delay, path of stations)>
    pub fn shortest_path(&self, start: &Station, end: &Station) -> Option<(f32, Vec<Station>)> {
        let mut distances: HashMap<Station, f32> = HashMap::new();
        let mut previous: HashMap<Station, Station> = HashMap::new();
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((NotNan::new(0.0).unwrap(), start.clone()))); // Initialize heap with starting point
        distances.insert(start.clone(), 0.0);
        while let Some(Reverse((wrapped_dist, station))) = heap.pop() {
            let dist = wrapped_dist.into_inner();
            if &station == end {
                let mut path = vec![end.clone()];
                let mut current = end.clone();
                // Reconstruct the path from end to start
                while let Some(prevstation) = previous.get(&current) {
                    path.push(prevstation.clone());
                    current = prevstation.clone();
                }
                path.reverse();
                return Some((dist, path));
            }

            if let Some(neighbors) = self.nodes.get(&station) {
                for (neighbor, weight) in neighbors {
                    let new_dist = dist + *weight;
                    let is_better = match distances.get(neighbor) {
                        None => true,
                        Some(&current_dist) => new_dist < current_dist,
                    };
                    if is_better {
                        distances.insert(neighbor.clone(), new_dist);
                        previous.insert(neighbor.clone(), station.clone());
                        heap.push(Reverse((NotNan::new(new_dist).unwrap(), neighbor.clone())));
                    }
                }
            }
        }
        None // No path found
    }

    // Calculates closeness centrality for a given station
    // Returns None if station is isolated or unreachable from others
    // Closeness is defined as the number of reachable nodes divided by the sum of shortest-path delays to them
    pub fn closeness_centrality(&self, station: &Station) -> Option<f32> {
        let mut total_delay = 0.0; 
        let mut reachable = 0;    
        let n = self.nodes.len(); 
        // Loop through all other stations in the graph
        for other in self.nodes.keys() {
            if other == station {
                continue; // Skip calculating distance to itself
            }
            // Try computing shortest path from station to `other`
            if let Some((delay, _path)) = self.shortest_path(station, other) {
                total_delay += delay; 
                reachable += 1;      
            }
        }

        // If no reachable nodes or no delay accumulated, return None (undefined closeness)
        if total_delay == 0.0 || reachable == 0 {
            if reachable == 0 { return None } 
            None // Covers cases where delays exist but all are zero
        } else {
            // Compute closeness as the number of reachable nodes divided by total delay
            Some(reachable as f32 / total_delay) // Higher value means more central (lower delay to more stations)
        }
    }

    // Ranks stations by closeness centrality and prints top N
    pub fn rank_stations_by_closeness(&self, top_n: usize) {
        let mut results: Vec<(Station, f32)> = vec![];
        for station in self.nodes.keys() {
            if let Some(score) = self.closeness_centrality(station) {
                results.push((station.clone(), score));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!("Top {} stations by closeness centrality:", top_n);
        for (i, (station, score)) in results.iter().take(top_n).enumerate() {
            println!("{:>2}. {:<30} {:.4}", i + 1, station, score);
        }
    }

    // Computes unweighted betweenness centrality for all stations
    // Betweenness measures how often a station appears on shortest paths between other stations
    // Returns a HashMap mapping each station to its centrality score
    pub fn betweenness_centrality(&self) -> HashMap<Station, f32> {
        let all: Vec<Station> = self.all_stations().into_iter().collect(); // Collect all unique stations
        // Initialize centrality map with zero for each station
        let mut centrality: HashMap<Station, f32> =
            all.iter().map(|v| (v.clone(), 0.0)).collect();
        // Iterate over each station as the source
        for s in &all {
            let mut stack: Vec<Station> = Vec::new(); // Stack for storing visitation order
            let mut preds: HashMap<Station, Vec<Station>> = HashMap::new(); // Predecessors in shortest paths
            let mut sigma: HashMap<Station, f32> = all.iter().map(|v| (v.clone(), 0.0)).collect(); // Num of shortest paths to each node
            let mut dist: HashMap<Station, i32> = all.iter().map(|v| (v.clone(), -1)).collect(); // Distance from source
            let mut queue: VecDeque<Station> = VecDeque::new(); // Queue for BFS
            sigma.insert(s.clone(), 1.0); // There's one path to the source
            dist.insert(s.clone(), 0);    // Distance to self is 0
            queue.push_back(s.clone());   // Start BFS from source
            // BFS traversal from source to discover shortest paths
            while let Some(v) = queue.pop_front() {
                stack.push(v.clone());
                let d_v = dist[&v];
                // For each neighbor of v
                for (w, _) in self.nodes.get(&v).into_iter().flatten() {
                    if dist[w] < 0 {
                        // First time visiting w
                        dist.insert(w.clone(), d_v + 1);
                        queue.push_back(w.clone());
                    }
                    if dist[w] == d_v + 1 {
                        // If w is reachable via shortest path through v
                        let sv = sigma[&v];
                        let entry = sigma.get_mut(w).unwrap();
                        *entry += sv; // Accumulate path counts
                        preds.entry(w.clone()).or_default().push(v.clone());
                    }
                }
            }
            // Dependency accumulation
            let mut delta: HashMap<Station, f32> = all.iter().map(|v| (v.clone(), 0.0)).collect();
            // Back-propagate dependencies from the stack
            while let Some(w) = stack.pop() {
                for v in preds.get(&w).into_iter().flatten() {
                    let sig_w = sigma[&w];
                    if sig_w > 0.0 {
                        // Distribute dependency based on path counts
                        let c = (sigma[v] / sig_w) * (1.0 + delta[&w]);
                        delta.entry(v.clone()).and_modify(|x| *x += c);
                    }
                }
                if w != *s {
                    let contrib = delta[&w];
                    // Only add finite and non-negative contributions
                    if contrib.is_finite() && contrib >= 0.0 {
                        centrality.entry(w.clone()).and_modify(|x| *x += contrib);
                    }
                }
            }
        }

        centrality // Return final centrality map
    }


    // Ranks and prints top N stations by betweenness centrality
    pub fn rank_stations_by_betweenness(&self, top_n: usize) {
        let mut scores: Vec<(Station, f32)> = self.betweenness_centrality().into_iter().collect();
        scores.retain(|(_, sc)| sc.is_finite());
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        println!("Top {} stations (unweighted betweenness):", top_n);
        for (i, (st, sc)) in scores.into_iter().take(top_n).enumerate() {
            println!("{:>2}. {:<30} {:.4}", i + 1, st, sc);
        }
    }

    // Computes average delay per route in the network
    // Output: Vec of ((from, to), avg_delay, trip_count)
    pub fn get_route_average_delays(&self) -> Vec<((Station, Station), f32, usize)> {
        let mut totalroutes: HashMap<(Station, Station), (f32, usize)> = HashMap::new();
        for (from, neighbors) in &self.nodes {
            for (to, delay) in neighbors {
                let entry = totalroutes.entry((from.clone(), to.clone())).or_insert((0.0, 0));
                entry.0 += *delay; // Accumulate delay
                entry.1 += 1;      // Count trips
            }
        }
        totalroutes.into_iter().map(|((from, to), (total_delay, count))| {
                ((from, to), total_delay / count as f32, count) // Compute average
            })
            .collect()
    }

    // Prints top N routes with highest average delay
    pub fn rank_routes_by_average_delay(&self, n: usize) {
        let mut averages = self.get_route_average_delays();
        let mut averages = self.get_route_average_delays().into_iter().filter(|(_, _, count)| *count >= 5).collect::<Vec<_>>(); // Filter routes with at least 5 trips
        averages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        println!("Top {} routes by average delay:", n);
        for (i, ((from, to), avg, count)) in averages.into_iter().take(n).enumerate() {
            println!(
                "{:>2}. {} → {} : {:.2} minutes ({} trips)", i + 1, from, to, avg, count
            );
        }
    }

    // Prints top N routes with the lowest average delay
    pub fn rank_routes_by_lowest_delay(&self, n: usize) {
        let mut averages = self.get_route_average_delays().into_iter().filter(|(_, _, count)| *count >= 5).collect::<Vec<_>>(); // Filter routes with at least 5 trips
        averages.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        println!("Top {} routes by **lowest** average delay:", n);
        for (i, ((from, to), avg, count)) in averages.into_iter().take(n).enumerate() {
            println!(
                "{:>2}. {} → {} : {:.2} minutes ({} trips)", i + 1, from, to, avg, count
            );
        }
    }
}
