use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerErrors {
    #[error("Failed to Pre process the data")]
    DataPreProcessError(),

    #[error("Failed to fetch the data from the API")]
    FetchError()

}