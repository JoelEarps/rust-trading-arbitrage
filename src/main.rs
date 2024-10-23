use std::{error, sync::{Arc, RwLock}};

use log::{info, error};
use reqwest::Error;
mod data_processing;
mod graph_algorithms;
use data_processing::data_pre_processing::pre_process_request_data;
use graph_algorithms::handler::{Graph, SearchAllEdgesAlgorithm};
mod fetch_rates;
use fetch_rates::fetch_rates;
use tokio::task;


/* POA:
1. Bubble up custom errors
2. Configurable BE address and thread request time
    a. Loop errors
3. Tokio multi loop
4. Binary search algorithm for live graph
5. Graceful shutdown
 */

// Add a config value that passes this periodically
// Create Arc that shares data
async fn fetch_rates_periodically(shared_response_data: Arc<RwLock<i32>>){
    loop {
        // Create arc to share between threads
        // Trigger update of graph if the rates change
        // or update graph for rates that have changed and trigger function - trigger new thread
        // Assumption - no rate is expected to stay exactly the same given f32
        let rates_response = fetch_rates().await;
        info!("Successfully pulled rates, {:#?}", rates_response);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        if let Ok(mut number) = shared_response_data.try_write() {
        *number += 2;
        println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!")
    };
    }
    
}

async fn re_calculate_values(shared_response_data: Arc<RwLock<i32>>){
    // Check for change in values, this is where maybe you could do a sorting and checking alogrithm
    match shared_response_data.try_read() {
        Ok(data) => {
            info!("Data read successfully, {}", data);
        }
        Err(_) => {
            error!("Could not return data, adding to failure list and then bubbling up error via custom at some point");
        }
    }

}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Staring arbitrage application");
    let shared_response_data = Arc::new(RwLock::new(0));
    let fetch_handle = task::spawn(fetch_rates_periodically(shared_response_data.clone()));
    let generate_opportunities = task::spawn(re_calculate_values(shared_response_data));
    let _ = tokio::join!(fetch_handle, generate_opportunities);
    // let mut rates_response = fetch_rates().await?;
    // let graph_data = pre_process_request_data(&mut rates_response.rates);
    // let bellman_ford_graph = Graph::new(graph_data.graph_edges, graph_data.graph_vertices_total);
    // info!("Checking for the shortest path for each vertex");
    // bellman_ford_graph.search_for_arbitrage(0);
    Ok(())
}
