use std::sync::{Arc, RwLock};

use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use log::{info};
use anyhow::Result;
#[derive(Deserialize, Debug)]
pub struct RatesResponse {
    pub rates: HashMap<String, String>,
}

pub async fn fetch_rates_periodically(shared_response_data: Arc<RwLock<RatesResponse>>) -> Result<()>{
    let mut attempt_counter = 0;
    loop {
        let rates_response = fetch_rates().await?;
        info!("Successfully pulled rates, {:#?}", rates_response);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        if let Ok(mut return_data) = shared_response_data.try_write() {
        // Implement Binary Search for updated rates here? Custom error for failure to assign?
        return_data.rates = rates_response.rates;
        println!("Now the number is {:?}", return_data.rates);
        attempt_counter = 0;
    } else {
        println!("Couldn't get write access, sorry!");
        };
        attempt_counter+=1;
        if attempt_counter > 5 {
            return Err(anyhow::Error::msg("Test Error"))
        }
    }  
}


async fn fetch_rates() -> Result<RatesResponse> {
    let url = "https://api.swissborg.io/v1/challenge/rates";
    // Custom error here for failing - demonstrate ability to do this?
    let response = reqwest::get(url).await?.json::<RatesResponse>().await?;
    Ok(response)
}
