use std::{error::Error, fmt::Display};


#[derive(Debug)]
pub enum IDTError {
    IDTFormatError(String),
    IDTValidationError(String),
    IDTMetadataError(String),
    IDTParsingError(Box<dyn Error + 'static>)
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
                write!(f, "IDT Parser File Read failed with:\n{}", source),
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