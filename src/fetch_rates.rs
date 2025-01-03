use std::sync::{Arc, RwLock};
use serde::Deserialize;
use std::collections::HashMap;
use log::{info, warn};
use crate::errors::fetching_rates_errors::FetchingErrors;

#[derive(Deserialize, Debug)]
pub struct RatesResponse {
    pub rates: HashMap<String, String>,
}

pub async fn fetch_rates_periodically(shared_response_data: Arc<RwLock<RatesResponse>>) -> Result<(), FetchingErrors>{
    let mut attempt_counter = 0;
    loop {
        let rates_response = fetch_rates().await?;
        info!("Successfully pulled rates, {:#?}", rates_response);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        if let Ok(mut return_data) = shared_response_data.try_write() {
        return_data.rates = rates_response.rates;
        info!("Now the number is {:?}", return_data.rates);
        attempt_counter = 0;
        } else {
            warn!("Couldn't get write access, sorry!");
        };
        attempt_counter+=1;
        if attempt_counter > 5 {
                return Err(FetchingErrors::MutexError("Retry Limit Exceeded 5 times".to_string()));
        }
    }  
}


async fn fetch_rates() -> Result<RatesResponse, FetchingErrors> {
    let url = "https://api.swissborg.io/v1/challenge/rates";
    let response = reqwest::get(url).await?.json::<RatesResponse>().await?;
    Ok(response)
}
