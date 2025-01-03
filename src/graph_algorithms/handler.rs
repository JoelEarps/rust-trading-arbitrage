use log::{info, error};
use std::{borrow::Borrow, sync::{Arc, RwLock, TryLockError}};
use anyhow::Result;
use anyhow::anyhow;


use crate::{data_processing::data_pre_processing::pre_process_request_data, errors::data_processing_errors::DataPreProcessingResult, fetch_rates::RatesResponse};

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
    match shared_response_data.try_read() {
        Ok(data) => {
            info!("Data read successfully, {:?}", data);
            let mut latest_rates = data.rates.clone();
            let graph_data = pre_process_request_data(&mut latest_rates)?;
            let bellman_ford_graph = Graph::new(graph_data.graph_edges, graph_data.graph_vertices_total);
            info!("Checking for the shortest path for each vertex");
            bellman_ford_graph.search_for_arbitrage(0);
            Ok(())
        }
        Err(err) => {
            error!("Failed to access Data in via Mutex");
            match err {
                TryLockError::WouldBlock => {
                    error!("Failed to access data: lock would block");
                    return Err(anyhow!("Failed to access data: lock would block"));
                }
                TryLockError::Poisoned(_) => {
                    error!("Failed to access data: lock is poisoned");
                    return Err(anyhow!("Failed to access data: lock is poisoned"));
                }
            }
        }
     }
    }
}

pub trait SearchAllEdgesAlgorithm {
    fn search_for_arbitrage(&self, start: usize) -> ();
}
