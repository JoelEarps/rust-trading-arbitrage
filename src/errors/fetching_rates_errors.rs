
use std::sync::TryLockError;

use thiserror::Error;

/* Notes on Using thisError
Really good article on when and why to use: https://medium.com/rustaceans/a-comprehensive-guide-to-robust-code-with-thiserror-for-rust-43778b1b3906
Requirements for usage here:
1. Use ThisError to Create custom errors
2. Allow various error types e.g. all responses for fetching data from the API to be handled by a single error definition
3. Display messages relevant to each error

Notes:
ThisError provides a convenient attribute std::error::Error reducing boiler plate and this improving code readability and maintainability.
Developed by David Tolnay.

Features:
1. Provide custom error messages using #[error("..")]
2. Use #[from] to generate a From implementation for each variant containing a source error


Note:
If you are sat there thinking, where is the error handling, the ? operator will automatically convert the error into the a type declared in this enum

If there is not a variant that matches the error and you are not using anyhow then you need to do one of the following:
Adding a New Variant: Add a specific variant to the CustomError enum that can wrap the error type.
Mapping the Error: Manually map the error to an existing variant of CustomError or a custom error message.
Generic Catch-All: Add a catch-all Other variant or similar to handle unexpected error types.

*/
#[derive(Error, Debug)]
pub enum FetchingErrors {
    #[error("Failed to Pre process the data")]
    MutexError(String),

    #[error("Failed to fetch the data from the API")]
    FetchError(#[from] reqwest::Error)
}