use serde::Deserialize;
use std::collections::HashMap;
use reqwest::{Error, Response};
mod data_cleaning;
mod bellman_ford;
use bellman_ford::EdgeGraph;
use data_cleaning::remove_duplicate_tickers;

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
    let graph_edges = remove_duplicate_tickers(&mut rates_response.rates);

    Ok(())
}
