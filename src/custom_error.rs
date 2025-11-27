use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Table not found: {0}")]
    TableNotFound(String),

    #[error("Table already exists: {0}")]
    TableAlreadyExists(String),
    #[error("Record already exists: {0}")]
    RecordAlreadyExists(String),
    #[error("{0} is empty")]
    InvalidKey(String),
    #[error("Invalid COmmand Screen Combination: {0} {1}")]
    ScreenInvalidForCommand(String, String),

    #[error("Failed to parse integer: {0}")]
    ParseError(#[from] ParseIntError),

    #[error("Failed to parse field: {0}")]
    FieldParseError(String),

    #[error("Failed to parse value: {0}")]
    ValueParseError(String),

    #[error("Failed to parse condition: {0}")]
    ConditionParseError(String),
    #[error("Missing keyword: {0}")]
    MissingKeyword(String),
    #[error("{0} with index {1} not found")]
    ItemWithIndexNotFound(String, String),

    #[error("{0} is empty")]
    MissingField(String),
    #[error("You somehow got lost")]
    YouGotLost(),
    #[error("Unknown Command: {0}")]
    UnknownCommand(String),
    #[error("Unknown Field: {0}")]
    UnknownField(String),
    #[error("Invalid index: {0}")]
    InvalidIndex(usize),
    #[error("Invalid Key Type")]
    WrongKeyType(),
}
