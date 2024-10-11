use std::collections::HashMap;

use crate::bellman_ford::GraphEdge;

pub struct RequiredGraphData{
    graph_edges: Vec<GraphEdge>,
    graph_vertex_total: i32
}

// Data will be consistent here, always of length two and therefore we can do something a bit more hard coded
pub(crate) fn remove_duplicate_tickers(raw_rates: &mut HashMap<String, String>) ->  RequiredGraphData {
    println!("Removing duplicate tickers to ensure clean node map");
    let mut graph_connections = Vec::new();
    let mut total_graph_vertex_number = 0;
    raw_rates.retain(|key, value| {
        let mut split_key: Vec<&str> = key.split("-").collect();
        split_key.dedup();
        println!("{:?}", &split_key.len());
        match split_key.len() {
            // For now you can use this to count the number of edges, this assumes the data stays uniform
            1 => { println!("Conversions between the same tickers found");
                    total_graph_vertex_number+=1;
                   false
                }
            2 => {
                println!("Valid pairing found");
                graph_connections.push(GraphEdge{ start_node: split_key[0].to_string(), end_node: split_key[1].to_string(), conversion_rate: value.parse::<f64>().expect("Hahahah unlucky") });
                true
            }
            _ => {
                println!("Invalid input");
                false
            }
        }
    
    });
    RequiredGraphData {
        graph_edges: graph_connections,
        graph_vertex_total: total_graph_vertex_number
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_duplicates_with_replica_data() {
        // Test data the same as what would be pulled from the API
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("BORG-BTC".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("BTC-BTC".to_string(), "1.00000000".to_string());
        exchange_rates.insert("DAI-BTC".to_string(), "61194.73626107".to_string());
        exchange_rates.insert("EUR-BORG".to_string(), "0.14846482".to_string());
        exchange_rates.insert("BORG-DAI".to_string(), "6.10395983".to_string());
        exchange_rates.insert("DAI-EUR".to_string(), "1.10578631".to_string());
        exchange_rates.insert("EUR-DAI".to_string(), "0.89846283".to_string());
        exchange_rates.insert("BTC-BORG".to_string(), "0.00000269".to_string());
        exchange_rates.insert("EUR-BTC".to_string(), "57157.97132034".to_string());
        exchange_rates.insert("BTC-EUR".to_string(), "0.00001739".to_string());
        exchange_rates.insert("BTC-DAI".to_string(), "0.00001586".to_string());
        exchange_rates.insert("BORG-BORG".to_string(), "1.00000000".to_string());
        exchange_rates.insert("EUR-EUR".to_string(), "1.00000000".to_string());
        exchange_rates.insert("BORG-EUR".to_string(), "6.53505020".to_string());
        exchange_rates.insert("DAI-BORG".to_string(), "0.16276449".to_string());
        exchange_rates.insert("DAI-DAI".to_string(), "1.00000000".to_string());

        let mut expected_graph_edges = vec![
        GraphEdge { start_node: "BORG".to_string(), end_node: "DAI".to_string(), conversion_rate: 6.10395983 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "BTC".to_string(), conversion_rate: 57157.97132034 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "BORG".to_string(), conversion_rate: 0.14846482 },
        GraphEdge { start_node: "BORG".to_string(), end_node: "BTC".to_string(), conversion_rate: 370331.49347896 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "BORG".to_string(), conversion_rate: 0.16276449 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "BTC".to_string(), conversion_rate: 61194.73626107 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "DAI".to_string(), conversion_rate: 1.586e-5 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "EUR".to_string(), conversion_rate: 1.10578631 },
        GraphEdge { start_node: "BORG".to_string(), end_node: "EUR".to_string(), conversion_rate: 6.5350502 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "BORG".to_string(), conversion_rate: 2.69e-6 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "DAI".to_string(), conversion_rate: 0.89846283 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "EUR".to_string(), conversion_rate: 1.739e-5 },
        ];

        assert_eq!(exchange_rates.len(), 16);
        let mut test_vector_return = remove_duplicate_tickers(&mut exchange_rates);
        assert_eq!(test_vector_return.graph_vertex_total, 4);
        test_vector_return.graph_edges.sort_by(|graph_edge_1, graph_edge_2| graph_edge_1.start_node.cmp(&graph_edge_2.start_node));
        expected_graph_edges.sort_by(|graph_edge_1, graph_edge_2| graph_edge_1.start_node.cmp(&graph_edge_2.start_node));
        assert_eq!(test_vector_return.graph_edges.len(), expected_graph_edges.len());
        assert!(test_vector_return.graph_edges.iter().all(|v| expected_graph_edges.contains(v)));
        assert_eq!(exchange_rates.len(), 12);
    }
    #[test]
    // Random Test data - all the same should be zero
    fn test_duplicates_with_no_replica_data() {
        // Test data the same as what would be pulled from the API
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("BORG-BTC".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("DAI-BTC".to_string(), "61194.73626107".to_string());
        exchange_rates.insert("EUR-BORG".to_string(), "0.14846482".to_string());
        exchange_rates.insert("BORG-DAI".to_string(), "6.10395983".to_string());
        exchange_rates.insert("DAI-EUR".to_string(), "1.10578631".to_string());
        exchange_rates.insert("EUR-DAI".to_string(), "0.89846283".to_string());
        exchange_rates.insert("BTC-BORG".to_string(), "0.00000269".to_string());
        exchange_rates.insert("EUR-BTC".to_string(), "57157.97132034".to_string());
        exchange_rates.insert("BTC-EUR".to_string(), "0.00001739".to_string());
        exchange_rates.insert("BTC-DAI".to_string(), "0.00001586".to_string());
        exchange_rates.insert("BORG-EUR".to_string(), "6.53505020".to_string());
        exchange_rates.insert("DAI-BORG".to_string(), "0.16276449".to_string());

        let mut expected_graph_edges = vec![
        GraphEdge { start_node: "BORG".to_string(), end_node: "DAI".to_string(), conversion_rate: 6.10395983 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "BTC".to_string(), conversion_rate: 57157.97132034 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "BORG".to_string(), conversion_rate: 0.14846482 },
        GraphEdge { start_node: "BORG".to_string(), end_node: "BTC".to_string(), conversion_rate: 370331.49347896 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "BORG".to_string(), conversion_rate: 0.16276449 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "BTC".to_string(), conversion_rate: 61194.73626107 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "DAI".to_string(), conversion_rate: 1.586e-5 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "EUR".to_string(), conversion_rate: 1.10578631 },
        GraphEdge { start_node: "BORG".to_string(), end_node: "EUR".to_string(), conversion_rate: 6.5350502 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "BORG".to_string(), conversion_rate: 2.69e-6 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "DAI".to_string(), conversion_rate: 0.89846283 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "EUR".to_string(), conversion_rate: 1.739e-5 },
        ];

        assert_eq!(exchange_rates.len(), 12);
        let mut test_vector_return = remove_duplicate_tickers(&mut exchange_rates);
        assert_eq!(test_vector_return.graph_vertex_total, 0);
        test_vector_return.graph_edges.sort_by(|graph_edge_1, graph_edge_2| graph_edge_1.start_node.cmp(&graph_edge_2.start_node));
        expected_graph_edges.sort_by(|graph_edge_1, graph_edge_2| graph_edge_1.start_node.cmp(&graph_edge_2.start_node));
        assert_eq!(test_vector_return.graph_edges.len(), expected_graph_edges.len());
        assert!(test_vector_return.graph_edges.iter().all(|v| expected_graph_edges.contains(v)));
        assert_eq!(exchange_rates.len(), 12);
    }
    #[test]
    // Random data, no duplicates
    fn test_duplicates_with_all_replica_tickers() {
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("BTC-BTC".to_string(), "1.00000000".to_string());
        exchange_rates.insert("BORG-BORG".to_string(), "1.00000000".to_string());
        exchange_rates.insert("EUR-EUR".to_string(), "1.00000000".to_string());
        exchange_rates.insert("DAI-DAI".to_string(), "1.00000000".to_string());

        assert_eq!(exchange_rates.len(), 4);
        let test_vector_return = remove_duplicate_tickers(&mut exchange_rates);
        assert_eq!(test_vector_return.graph_vertex_total, 4);
        assert_eq!(test_vector_return.graph_edges.len(), 0);
        assert_eq!(exchange_rates.len(), 0);
    }
    #[test]
    // Random data none ticker related - no splitting custom error?
    fn test_duplicates_with_random_data() {
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("Test1".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("Test2".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("Test3".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("Test4".to_string(), "370331.49347896".to_string());
        assert_eq!(exchange_rates.len(), 4);
        let test_vector_return = remove_duplicate_tickers(&mut exchange_rates);
        // Temp solution for now, for random data you will need to have a custom error here, or handle based on the return from the function
        assert_eq!(test_vector_return.graph_vertex_total, 4);
        assert_eq!(test_vector_return.graph_edges.len(), 0);
        assert_eq!(exchange_rates.len(), 0);
    }
}
