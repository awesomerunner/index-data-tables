// Module for handling IDT validation schemes
use crate::idt_errors::IDTError;

use crate::parity::xor_slice;

/**
 * Accepts a byte slice containing a validation block, checks that the size of
 * the validation is correct, and performs validation.
 * After validating, sends the data to a destination slice
 * TODO: Implement single-bit error correction
 */
pub fn read_validation_block<I> (validation_size: usize, block: &[u8], 
    data_destination: &mut[u8]) -> Result<(), IDTError> 
where 
    I: Iterator
{
    // Define useful values for checks and subslices
    let target_block_length = (validation_size + 1) * (validation_size + 1);
    let data_size = validation_size * validation_size;

    // Check block and destination lengths
    if block.len() != target_block_length {
        return Err(IDTError::IDTValidationError(
            "VALIDATOR: INCORRECTLY SIZED BLOCK".to_string()))
    }
    if data_destination.len() != data_size {
        return Err(IDTError::IDTValidationError(
            "VALIDATOR: INCORRECTLY SIZED DATA DESTINATION".to_string()))
    }

    // Initialize parity checks to the value of parity bytes
    let mut row_parity_check = block[data_size
        ..data_size + validation_size].to_vec();
    let mut column_parity_check = block[data_size + validation_size
        ..data_size + (2 * validation_size)].to_vec();
    
    // If the overall parity of the parity bytes aren't 0, there is an error
    if xor_slice(&row_parity_check[..]) != 0_u8
        || xor_slice(&column_parity_check[..]) != 0_u8 {
        return Err(IDTError::IDTValidationError("FAILED TO VALIDATE DATA"
            .to_string()));
        // TODO: Count errors and implement correction
    }
    
    // Loop through data bytes and apply them to their parity checks
    for row in 0..validation_size {
        for column in 0..validation_size {
            let item = block[(row * validation_size) + column];
            row_parity_check[row] ^= item;
            column_parity_check[column] ^= item;
        }
        // row_parity_check[row] ^= block[(validation_size * validation_size) + row];
    }
    
    // Check parity
    for i in 0..validation_size {
        if row_parity_check[i] != 0 || column_parity_check[i] != 0 {
        return Err(IDTError::IDTValidationError("FAILED TO VALIDATE DATA"
            .to_string()));
        }
    }

    // data_destination has already been length-checked
    data_destination.copy_from_slice(&block[0..data_size]);

    Ok(())
}