use crate::graph_algorithms::handler::{IndexedGraphEdge, NoneIndexedGraphEdge};
use log::{info, trace, warn};
use std::collections::HashMap;

pub struct RequiredGraphData {
    pub graph_edges: Vec<IndexedGraphEdge>,
    pub graph_vertices_total: usize,
}

pub(crate) fn pre_process_request_data(
    raw_rates: &mut HashMap<String, String>,
) -> RequiredGraphData {
    info!("Removing duplicate tickers to ensure clean node map");
    let mut none_indexed_graph_edges = Vec::new();
    let mut currency_index_store: HashMap<String, usize> = HashMap::new();
    let mut total_graph_vertices_number = 0;
    raw_rates.retain(|key, value| {
        let mut split_key: Vec<&str> = key.split("-").collect();
        split_key.dedup();
        match split_key.len() {
            1 => {
                warn!("Conversions between the same tickers found");
                currency_index_store.insert(split_key[0].to_owned(), total_graph_vertices_number);
                total_graph_vertices_number += 1;
                false
            }
            2 => {
                trace!("Valid pairing found");
                none_indexed_graph_edges.push(NoneIndexedGraphEdge {
                    start_node: split_key[0].to_string(),
                    end_node: split_key[1].to_string(),
                    conversion_rate: value.parse::<f64>().expect("Cannot parse conversion rate"),
                });
                true
            }
            _ => {
                warn!("Invalid input");
                false
            }
        }
    });
    // add error here for no values in index store and data contract changing!
    info!("{:?}", currency_index_store);
    create_indexing_for_currencies(none_indexed_graph_edges, currency_index_store)
}

fn create_indexing_for_currencies(
    none_indexed_graph_edges: Vec<NoneIndexedGraphEdge>,
    index_store: HashMap<String, usize>,
) -> RequiredGraphData {
    let indexed_graph_edges = none_indexed_graph_edges
        .iter()
        .map(|none_indexed_singleton| IndexedGraphEdge {
            start_node: index_store[&none_indexed_singleton.start_node],
            end_node: index_store[&none_indexed_singleton.end_node],
            log_conversion_value: -1.0 * none_indexed_singleton.conversion_rate.ln(),
        })
        .collect();

    RequiredGraphData {
        graph_edges: indexed_graph_edges,
        graph_vertices_total: index_store.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test data the same as what would be pulled from the API
    fn test_duplicates_with_replica_data() {
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

        assert_eq!(exchange_rates.len(), 16);
        let test_vector_return = pre_process_request_data(&mut exchange_rates);
        assert_eq!(test_vector_return.graph_vertices_total, 4);
        assert_eq!(exchange_rates.len(), 12);
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    // Random Test data - all the same should be zero and should panic due to no keys in hash maps
    fn test_duplicates_with_no_replica_data() {
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

        assert_eq!(exchange_rates.len(), 12);
        let test_vector_return = pre_process_request_data(&mut exchange_rates);
        assert_eq!(test_vector_return.graph_vertices_total, 0);
        assert_eq!(exchange_rates.len(), 12);
    }

    #[test]
    // Random data, no duplicates, this will fail due to assumptions - custom error for this?
    fn test_duplicates_with_all_replica_tickers() {
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("BTC-BTC".to_string(), "1.00000000".to_string());
        exchange_rates.insert("BORG-BORG".to_string(), "1.00000000".to_string());
        exchange_rates.insert("EUR-EUR".to_string(), "1.00000000".to_string());
        exchange_rates.insert("DAI-DAI".to_string(), "1.00000000".to_string());

        assert_eq!(exchange_rates.len(), 4);
        let test_vector_return = pre_process_request_data(&mut exchange_rates);
        assert_eq!(test_vector_return.graph_vertices_total, 4);
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
        let test_vector_return = pre_process_request_data(&mut exchange_rates);
        assert_eq!(test_vector_return.graph_vertices_total, 4);
        assert_eq!(test_vector_return.graph_edges.len(), 0);
        assert_eq!(exchange_rates.len(), 0);
    }
}
