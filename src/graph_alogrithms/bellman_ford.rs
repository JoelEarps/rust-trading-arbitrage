use log::{info, trace};

use crate::graph_alogrithms::graph_algorithm_handler::{Graph, SearchAllEdgesAlgorithm, IndexedGraphEdge};

impl SearchAllEdgesAlgorithm for Graph {
    fn search_for_arbitrage(&self) -> () {
        info!("Searching for Arbitrage Opportunities");
        let source_node = 0;
        let mut distances = vec![f64::INFINITY; self.total_vertexes];
        distances[source_node] = 0.0;
        let mut predecessor: Vec<usize> = vec![usize::MAX; self.total_vertexes]; 

        for _ in 0..self.total_vertexes - 1 {
            for edge in &self.edges {
            trace!("Distance to End Node: {}", distances[edge.end_node]);
                if distances[edge.start_node] != f64::INFINITY && distances[edge.start_node] + edge.log_conversion_value < distances[edge.end_node] {
                    distances[edge.end_node] = distances[edge.start_node] + edge.log_conversion_value;
                    trace!("New Edge Node Distance Calculated {}", distances[edge.end_node]);
                    predecessor[edge.end_node] = edge.start_node;
                }
            }
        }

        for edge in &self.edges {
            if distances[edge.start_node] != f64::INFINITY && distances[edge.start_node] + edge.log_conversion_value < distances[edge.end_node] {
            } 
        }

        for vert in predecessor {
            println!("{}", vert);
        }
    }
    
}


// Create a mermaid diagram to show how this all fits together
#[cfg(test)]
mod tests{
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_creation_of_graph() {
        let mut currency_map: HashMap<&str, usize> = HashMap::new();
        currency_map.insert("BORG", 0);
        currency_map.insert("DAI", 1);
        currency_map.insert("EUR", 2);
        currency_map.insert("BTC", 3);

        let test_input = vec![
        IndexedGraphEdge { start_node: currency_map["EUR"], end_node: currency_map["BTC"], conversion_rate: 57157.97132034, log_conversion_value: -1.0 * (57157.97132034f64).ln() },
        IndexedGraphEdge { start_node: currency_map["EUR"], end_node: currency_map["BORG"], conversion_rate: 0.14846482, log_conversion_value: -1.0 * (0.14846482f64).ln() },
        IndexedGraphEdge { start_node: currency_map["BORG"], end_node: currency_map["BTC"], conversion_rate: 370331.49347896, log_conversion_value: -1.0 * (370331.49347896f64).ln() },
        IndexedGraphEdge { start_node: currency_map["DAI"], end_node: currency_map["BORG"], conversion_rate: 0.16276449, log_conversion_value: -1.0 * (0.16276449f64).ln() },
        IndexedGraphEdge { start_node: currency_map["DAI"], end_node: currency_map["BTC"], conversion_rate: 61194.73626107, log_conversion_value: -1.0 * (61194.73626107f64).ln() },
        IndexedGraphEdge { start_node: currency_map["BTC"], end_node: currency_map["DAI"], conversion_rate: 1.586e-5, log_conversion_value: -1.0 * (1.586e-5f64).ln() },
        IndexedGraphEdge { start_node: currency_map["DAI"], end_node: currency_map["EUR"], conversion_rate: 1.10578631, log_conversion_value: -1.0 * (1.10578631f64).ln() },
        IndexedGraphEdge { start_node: currency_map["BORG"], end_node: currency_map["EUR"], conversion_rate: 6.5350502, log_conversion_value: -1.0 * (6.5350502f64).ln() },
        IndexedGraphEdge { start_node: currency_map["BTC"], end_node: currency_map["BORG"], conversion_rate: 2.69e-6, log_conversion_value: -1.0 *(2.69e-6f64).ln() },
        IndexedGraphEdge { start_node: currency_map["EUR"], end_node: currency_map["DAI"], conversion_rate: 0.89846283, log_conversion_value: -1.0 * (0.89846283f64).ln() },
        IndexedGraphEdge { start_node: currency_map["BTC"], end_node: currency_map["EUR"], conversion_rate: 1.739e-5, log_conversion_value: -1.0 * (1.739e-5f64).ln() },
    ];

        let test_bellman_ford = Graph::new(test_input, currency_map.len());
        test_bellman_ford.search_for_arbitrage();
    }
}
