use std::{error::Error, fmt::Display};


#[derive(Debug, Clone, PartialEq, Eq)]
enum IDTErrorType {
    IDTFormatError,
    IDTValidationError,
    IDTMetadataError,
    IDTParsingError,
}

#[derive(Debug)]
pub struct IDTError {
    variant: IDTErrorType,
    message: String,
    source: Option<Box<dyn Error + 'static>>
}

impl Display for IDTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.variant {
            IDTErrorType::IDTFormatError =>
                write!(f, "IDTFormatError: {}", self.message),
            IDTErrorType::IDTValidationError =>
                write!(f, "IDTFormatError: {}", self.message),
            IDTErrorType::IDTMetadataError =>
                write!(f, "IDTFormatError: {}", self.message),
            IDTErrorType::IDTParsingError =>
                write!(f, "IDTFormatError: {}", self.message),
        }
    }
}

impl Error for IDTError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}