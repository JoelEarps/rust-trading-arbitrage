
use std::sync::TryLockError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchingErrors {
    #[error("Failed to Pre process the data")]
    MutexError(String),

    #[error("Failed to fetch the data from the API")]
    FetchError(#[from] reqwest::Error)
}