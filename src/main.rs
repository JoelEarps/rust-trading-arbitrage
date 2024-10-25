use std::{collections::HashMap, sync::{Arc, RwLock}};
use log::{info, error};
use reqwest::Error;
mod data_processing;
mod graph_algorithms;
use data_processing::data_pre_processing::pre_process_request_data;
use graph_algorithms::handler::{Graph, SearchAllEdgesAlgorithm};
mod fetch_rates;
use fetch_rates::{fetch_rates_periodically, RatesResponse};
use tokio::task;
use anyhow::Result;


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

// Error that require bubbling up and kill application
/*
1. Writing/ Reading from Mutexes fails
2. Tasks joining errors and seeing if any thread finished properly or all failed - if we fail to calculate/ pull data
More than x amount times, stop application
 */

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
