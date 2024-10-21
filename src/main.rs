use log::info;
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
2. Configurable BE address
    a. Loop errors
3. Tokio multi loop
4. Binary search algorithm for live graph
5. Graceful shutdown
 */

// Add a config value that passes this periodically
// Create Arc that shares data
async fn fetch_rates_periodically(){
    loop {
        // Create arc to share between threads
        // Trigger update of graph if the rates change
        // or update graph for rates that have changed and trigger function - trigger new thread
        // Assumption - no rate is expected to stay exactly the same given f32
        let mut rates_response = fetch_rates().await;
        info!("Successfully pulled rates, {:#?}", rates_response);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
    
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Staring arbitrage application");
    let fetch_handle = task::spawn(fetch_rates_periodically());

    // Wait for both tasks to complete
    // Create custom error and handle
    let _ = tokio::join!(fetch_handle);

    let mut rates_response = fetch_rates().await?;
    info!("Successfully pulled rates, {:#?}", rates_response);
    let graph_data = pre_process_request_data(&mut rates_response.rates);
    let bellman_ford_graph = Graph::new(graph_data.graph_edges, graph_data.graph_vertices_total);
    info!("Checking for the shortest path for each vertex");
    bellman_ford_graph.search_for_arbitrage(0);
    Ok(())
}
