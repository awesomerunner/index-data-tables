// Module for parsing and creating IDT Tables and Table Entries

use std::sync::mpsc::Receiver;

use crate::idt_errors::{ExternalResult, IDTError, IDTResult};

/**
 * A trait for types able to read in all IDT index-entry pairs in a table
 * Recommended practice is for the implementing type to own a data structure
 *  for each table expected in the file and allow destructuring to obtain
 *  those data structures
 */
pub trait IDTTableReciever {
    /// Recieves the index and data of a table entry
    fn recieve_idt_entry(&mut self, index: u64, entry: &[u8]) -> ExternalResult;

    fn validate_table_header(&self, table_id: u32, data_type_id: u32) -> ExternalResult;
}

struct IDTTable<'a , R>
where
    R: IDTTableReciever
{
    table_id: u32,
    data_type_id: u32,
    receiver: &'a mut R,
}

impl<'a, R> IDTTable<'a, R> 
where
    R: IDTTableReciever
{
    fn new_table(table_id: u32, data_type_id: u32, receiver: &'a mut R) -> IDTResult<Self> {
        match receiver.validate_table_header(table_id, data_type_id) {
            Err(e) => {
                return Err(IDTError::IDTApplicationError(e))
            }
            _ => {}
        }

        Ok(IDTTable { table_id, data_type_id, receiver })
    }

    fn new_entry(&mut self, index: u64, data: &[u8]) -> IDTResult<()> {
        match self.receiver.recieve_idt_entry(index, data) {
            Err(e) => {
                return Err(IDTError::IDTApplicationError(e))
            }
            _ => {}
        }
        
        Ok(())
    }
}