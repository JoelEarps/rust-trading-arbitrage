use log::{info, error};
use std::sync::{Arc, RwLock};
use anyhow::Result;

use crate::fetch_rates::RatesResponse;

#[derive(Clone, Copy, Debug)]
pub struct IndexedGraphEdge {
    pub start_node: usize,
    pub end_node: usize,
    pub log_conversion_value: f64,
}

pub struct NoneIndexedGraphEdge {
    pub start_node: String,
    pub end_node: String,
    pub conversion_rate: f64,
}

pub struct Graph {
    pub edges: Vec<IndexedGraphEdge>,
    pub total_vertices: usize,
}

impl Graph {
    pub fn new(graph_edges: Vec<IndexedGraphEdge>, calculated_total_vertices: usize) -> Self {
        info!("Creating new nodes for edges");
        Self {
            edges: graph_edges,
            total_vertices: calculated_total_vertices,
        }
    }

    pub async fn re_calculate_values(shared_response_data: Arc<RwLock<RatesResponse>>) -> Result<()>{
    // Check for change in values, this is where maybe you could do a sorting and checking alogrithm
    match shared_response_data.try_read() {
        Ok(data) => {
            info!("Data read successfully, {:?}", data);
            Ok(())
        }
        Err(_) => {
            error!("Could not return data, adding to failure list and then bubbling up error via custom at some point");
            Ok(())
        }
     }
    }

    fn search_for_rate_changes() {}

    fn validate_results(){}

    fn pretty_print_results(){}
}

pub trait SearchAllEdgesAlgorithm {
    fn search_for_arbitrage(&self, start: usize) -> ();
}
