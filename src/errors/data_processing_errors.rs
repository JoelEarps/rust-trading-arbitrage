// When defining custom errors, there are two approaches given by the Rust Handbook
// Use a struct or an enum
// A struct would represent a single error and therefore reduce code reuse whereas an enum, as used here would represent multiple errors of a certain type
// Box can also be used here, but why is this not always a good idea?
use std::fmt;

use log::trace;

pub type DataPreProcessingResult<T> = std::result::Result<T,DataProcessingErrors>;

#[derive(Debug, Clone)]
pub enum DataProcessingErrors{
    InvalidGraphPair(String),
    ParsingError(String),
    NoValidIndexingData, 
    MultiErrorDump(Vec<DataProcessingErrors>)
}

impl fmt::Display for DataProcessingErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataProcessingErrors::InvalidGraphPair(base_currency_pair) => {
                write!(f, "Invalid Graph Pair for {}", base_currency_pair)
            },
            DataProcessingErrors::ParsingError(value_to_be_parsed) => {
                write!(f, "Data Parsing Error for stuff {}", value_to_be_parsed)
            },
            DataProcessingErrors::NoValidIndexingData => {
                write!(f, "No Key Value Pairs Exist to make valid indexing for Edges and Vertexes")
            }
            DataProcessingErrors::MultiErrorDump(generated_errors) => {
                // Mimic generating error dump
                for processing_error in generated_errors {
                    trace!("{}", processing_error);
                }
                write!(f, "Multiple Errors Occurred, report will be generated and accessible at random_file_name.txt")
            }
        }
    }
}

impl std::error::Error for DataProcessingErrors{}


// Implementing the From Trait - why was it needed?
/*
To use the ? operator and handle different error types that need to be converted into DataProcessingErrors. 
Without implementing From, the compiler would complain about mismatched types when trying to propagate errors with ?.

The issue here is whilst DataProcessingErrors implements the required traits for Anyhow to handle things,
a vector of DataProcessingErrors does not and therefore anyhow cannot convert them into/ from an error into anyhow error, hence the error message shown here:

the trait bound `Vec<DataProcessingErrors>: StdError` is not satisfied
the following other types implement trait `FromResidual<R>`:
  <Result<T, F> as FromResidual<Result<Infallible, E>>>
  <Result<T, F> as FromResidual<Yeet<E>>>
required for `anyhow::Error` to implement `From<Vec<DataProcessingErrors>>`
required for `Result<(), anyhow::Error>` to implement `FromResidual<Result<Infallible, Vec<DataProcessingErrors>>>

However there are two ways to handle this:
1. Implement the From and Into traits Vec<DataProcessingErrors> - this way you can create a multi error scenario that allows you to handle and generate an error report
2. Create an Error type that is the vector, returning a single error that represents several errors

// impl From<Vec<DataProcessingErrors>> for DataProcessingErrors {
//     fn from(data_errors: Vec<DataProcessingErrors>) -> Self {
        
//     }   
// }

Given the current implementation I am going to go with 2, this way I can generate a report that can be pushed like an error/ crash dump whilst being able to represent a collection of errors
as a single cohesive one without needing to implement further boiler plate

 */
