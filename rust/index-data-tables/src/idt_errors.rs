// Module for the IDT error type

use std::{error::Error, fmt::{Display}};

/**
 * Type aliases for Errors and Results external to this crate
 */
pub type ExternalError = Box<dyn Error + 'static>;
pub type ExternalResult = Result<(), ExternalError>;

pub type IDTResult<T> = Result<T, IDTError>;

#[derive(Debug)]
pub enum IDTError {
    IDTFormatError(String),
    IDTValidationError(String),
    IDTMetadataError(String),
    IDTParsingError(ExternalError),
    IDTApplicationError(ExternalError),
}

impl Display for IDTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IDTError::IDTFormatError(message) =>
                write!(f, "IDT FORMAT ERROR: {}", message),
            IDTError::IDTValidationError(message) =>
                write!(f, "IDT VALIDATION ERROR: {}", message),
            IDTError::IDTMetadataError(message) =>
                write!(f, "IDT METADATA ERROR: {}", message),
            IDTError::IDTParsingError(source) =>
                write!(f, "IDT Parser failed with external error:\n{}", source),
            IDTError::IDTApplicationError(source) =>
                write!(f, "IDT Parser terminated on application error:\n{}", source)
        }
    }
}

impl Error for IDTError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            IDTError::IDTParsingError(source) =>
                Some(source.as_ref()),
            _ => 
                None,
        }
    }
}