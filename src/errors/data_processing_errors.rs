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

