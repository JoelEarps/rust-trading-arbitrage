use serde::Deserialize;
use std::collections::HashMap;
use reqwest::{Error, Response};
mod data_preprocessing;
mod graph_alogrithms;
use graph_alogrithms::graph_algorithm_handler::{Graph, SearchAllEdgesAlgorithm};
use data_preprocessing::preprocess_request_data;

#[derive(Deserialize, Debug)]
struct RatesResponse {
    rates: HashMap<String, String>,
}

async fn fetch_rates() -> Result<RatesResponse, Error> {
    let url = "https://api.swissborg.io/v1/challenge/rates";
    // Custom error here for failing - demonstrate ability to do this?
    let response = reqwest::get(url).await?.json::<RatesResponse>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut rates_response = fetch_rates().await?;
    println!("{:#?}", rates_response);
    let graph_data = preprocess_request_data(&mut rates_response.rates);
    let bellman_ford_graph = Graph::new(graph_data.graph_edges, graph_data.graph_vertex_total);
    bellman_ford_graph.search_for_arbitrage();
    Ok(())
}
