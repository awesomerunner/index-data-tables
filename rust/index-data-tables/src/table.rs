// Module for parsing and creating IDT Tables and Table Entries

use std::sync::mpsc::Receiver;

use crate::idt_errors::{IDTError, UnknownResult};

/**
 * A trait for types able reads in all IDT index-entry pairs in a table
 * Recommended practice is for the implementing type to own a data structure
 *  for each table expected in the file and allow destructuring to obtain
 *  those data structures
 */
pub trait IDTEntryReciever {
    /// Recieves the index and data of a table entry
    fn recieve_idt_entry(&mut self, table_id: u32, index: u64, entry: &[u8]) -> UnknownResult;

    fn validate_data_type(&self, table_id: u32, table_data_id: u32) -> UnknownResult;
}

struct IDTTable<'a , R>
where
    R: IDTEntryReciever
{
    table_id: u32,
    data_type_id: u32,
    reciever: &'a mut R,
}

impl<'a, R> IDTTable<'a, R> 
where
    R: IDTEntryReciever
{
    fn new(table_id: u32, data_type_id: u32, receiver: &'a mut R) -> () {
        ()
    }

    fn new_entry(&self, index: u64, data: &[u8]) -> () {
        ()
    }
}