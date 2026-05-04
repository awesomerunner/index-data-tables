use crate::idt_errors::IDTError;

/**
 * Header Data of a valid IDT file
 */
pub struct IDTHeader {
    version: u16,
    field_size: u8,
    validation_size: u8,
}

pub fn read_idt_header(header: &[u8]) -> Result<IDTHeader, IDTError> {
    todo!()
}