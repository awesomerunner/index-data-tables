use crate::{idt_errors::IDTError, validation::xor_slice};

/**
 * Header Data of a valid IDT file
 */
pub struct IDTHeader {
    version: u16,
    common_size: u8,
    validation_size: u8,
}

pub fn read_idt_header(header: &[u8; 8]) -> Result<IDTHeader, IDTError> {
    // Header reading errors
    if xor_slice(header) != 0 {
        return Err(IDTError::IDTFormatError("HEADER VALIDATION FAILURE".to_string()));
    }
    if &header[0..2] != &[0x49, 0x44, 0x54] {
        return Err(IDTError::IDTFormatError("INVALID MAGIC".to_string()));
    }
    // Errors in header contents handled upstream in the parser
    Ok(IDTHeader {
        version: ((header[3] as u16) << 8) | header[4] as u16,
        common_size: header[5],
        validation_size: header[6],
    })
}