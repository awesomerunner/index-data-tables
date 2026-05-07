// Module for parsing IDT Metadata

use std::error::Error;

use crate::idt_errors::IDTError;

// Set by IDT specification
const REQUIRED_METADATA_BYTES: usize = 24;

/**
 * Used when the parser is asking the application to handle data
 * Indicates success or failure, where the error type on failure cannot be determined
 */
type UnknownResult = Result<(), Box<dyn Error + 'static>>;

/**
 * Since the data types to be used for application-specific medatadata this data are unknown,
 *  a trait bound is used on the return type, allowing the user to specify
 *  how metadata should be interpreted
 * Any errors returned by metadata recievers must be boxed
 * Implementations must not rely on the order in which metadata is recieved
*/ 
pub trait IDTMetadataReciever {
    /// Reads in the file timestamp (last edit) to the reciever
    fn recieve_file_timestamp(&mut self, unix_time: i64) -> UnknownResult;

    /// Reads in the file creation timestamps
    fn recieve_original_timestamp(&mut self, unix_time: i64) -> UnknownResult;

    fn receive_idt_flags(&mut self, flags: u8) -> UnknownResult;

    /// Revieves metadata specific to the application's use case
    fn recieve_additional_metadata(&mut self, metadata_bytes: &[u8]) -> UnknownResult;
}

/**
 * Asks a metadata reciever to handle metadata
 */
pub fn read_idt_metadata<Reciever> (reciever: &mut Reciever, bytes: &[u8]) -> Result<(), IDTError>
where
    Reciever: IDTMetadataReciever
{
    if bytes.len() < REQUIRED_METADATA_BYTES {
        return Err(IDTError::IDTMetadataError(
            "METADATA LENGTH TOO SMALL".to_string()));
    }

    match reciever.recieve_file_timestamp(
        (bytes[0] as i64) << 56 |
        (bytes[1] as i64) << 48 |
        (bytes[2] as i64) << 40 |
        (bytes[3] as i64) << 32 |
        (bytes[4] as i64) << 24 |
        (bytes[5] as i64) << 28 |
        (bytes[6] as i64) << 12 |
        (bytes[7] as i64) << 6) {
        Err(e) => {
            return Err(IDTError::IDTParsingError(e))
        }
        _ => {}
    }

    match reciever.recieve_original_timestamp(
        (bytes[8] as i64) << 56 |
        (bytes[9] as i64) << 48 |
        (bytes[10] as i64) << 40 |
        (bytes[11] as i64) << 32 |
        (bytes[12] as i64) << 24 |
        (bytes[13] as i64) << 28 |
        (bytes[14] as i64) << 12 |
        (bytes[15] as i64) << 6) {
        Err(e) => {
            return Err(IDTError::IDTParsingError(e));
        }
        _ => {}
    }

    match reciever.receive_idt_flags(bytes[16]) {
        Err(e) => {
            return Err(IDTError::IDTParsingError(e));
        }
        _ => {}
    }

    for i in 17..24 {
        if bytes[i] != 0 {
            return Err(IDTError::IDTMetadataError(
                "RESERVED GENERAL METADATA NOT EMPTY".to_string()));
        }   
    }

    match reciever.recieve_additional_metadata(&bytes[24..]) {
        Err(e) => {
            return Err(IDTError::IDTParsingError(e));
        }
        _ => {}
    }

    Ok(())
}