use std::{collections::HashMap, sync::{Arc, RwLock}};
use log::info;
mod data_processing;
mod graph_algorithms;
use graph_algorithms::handler::Graph;
mod fetch_rates;
use fetch_rates::{fetch_rates_periodically, RatesResponse};
use tokio::task;
use anyhow::Result;
mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Staring arbitrage application");
    let shared_response_data: Arc<RwLock<RatesResponse>> = Arc::new(RwLock::new(RatesResponse{rates: HashMap::new()}));
    let fetch_handle = task::spawn(fetch_rates_periodically(shared_response_data.clone()));
    let generate_opportunities = task::spawn(Graph::re_calculate_values(shared_response_data.clone()));
    let application_result = tokio::try_join!(fetch_handle, generate_opportunities);
    match application_result {
         Ok((first, second)) => {
             first?;
             second?;
         }
         Err(err) => {
            println!("processing failed; error = {}", err);
            return Err(anyhow::Error::new(err));
         }
    }
    Ok(())
}
