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

impl IDTError {
    /**
     * Returns an IDT Format Error with the given message
     * IDT Format Errors do not have source errors
    */
    pub fn idt_format_error(message: String) -> IDTError {
        IDTError { 
            variant: IDTErrorType::IDTFormatError,
            message: message,
            source: None,
        }
    }

    /**
     * Returns an IDT Validation Error with the given message
     * IDT Format Errors do not have source errors
    */
    pub fn idt_validation_error(message: String) -> IDTError {
        IDTError { 
            variant: IDTErrorType::IDTValidationError,
            message: message,
            source: None,
        }
    }

    /**
     * Returns an IDT Metadata Error with the given message
     * IDT Format Errors do not have source errors
    */
    pub fn idt_metadata_error(message: String) -> IDTError {
        IDTError { 
            variant: IDTErrorType::IDTMetadataError,
            message: message,
            source: None,
        }
    }

    /**
     * Returns an IDT Validation Error with the given message
     * IDT Format Errors do not have source errors
    */
    pub fn idt_parsing_error(message: String) -> IDTError {
        IDTError { 
            variant: IDTErrorType::IDTParsingError,
            message: message,
            source: None,
        }
    }
}