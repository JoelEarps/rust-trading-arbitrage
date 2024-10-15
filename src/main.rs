use log::info;
use reqwest::Error;
mod data_pre_processing;
mod graph_algorithms;
use data_pre_processing::pre_process_request_data;
use graph_algorithms::handler::{Graph, SearchAllEdgesAlgorithm};
mod fetch_rates;
use fetch_rates::fetch_rates;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Staring arbitrage application");
    let mut rates_response = fetch_rates().await?;
    info!("Successfully pulled rates, {:#?}", rates_response);
    let graph_data = pre_process_request_data(&mut rates_response.rates);
    let bellman_ford_graph = Graph::new(graph_data.graph_edges, graph_data.graph_vertices_total);
    info!("Checking for the shortest path for each vertex");
    bellman_ford_graph.search_for_arbitrage(0);
    Ok(())
}
