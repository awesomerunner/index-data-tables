use crate::merge_bytes::*;
use std::ops::*;
use std::fmt::Debug;

// Generic test runner – validates both success and error paths.
fn assert_merge<T>(bytes: &[u8], endianness: Endianness, expected: Result<T, String>)
where
    T: BitOr<Output = T>
        + Shl<usize, Output = T>
        + From<u8>
        + PartialEq
        + Debug
        + Copy,
{
    let result = merge_bytes::<T>(bytes, endianness);
    match expected {
        Ok(v) => assert_eq!(result.unwrap(), v),
        Err(msg) => assert_eq!(result.unwrap_err(), msg),
    }
}

#[test]
fn merge_u16_big_endian() {
    let bytes = [0x12, 0x34];
    let expected = 0x1234u16;
    assert_merge(&bytes, Endianness::BigEndian, Ok(expected));
}

#[test]
fn merge_u16_small_endian() {
    let bytes = [0x78, 0x56];
    let expected = 0x5678u16; // low byte first -> 0x56 0x78 in memory
    assert_merge(&bytes, Endianness::SmallEndian, Ok(expected));
}

#[test]
fn merge_u32_big_endian() {
    let bytes = [0xDE, 0xAD, 0xBE, 0xEF];
    let expected = 0xDEADBEEFu32;
    // let merge: u32 = merge_bytes(&bytes, Endianness::BigEndian).unwrap();
    // println!("{:x}", merge);
    assert_merge(&bytes, Endianness::BigEndian, Ok(expected));
}

#[test]
fn merge_u32_small_endian() {
    let bytes = [0xEF, 0xBE, 0xAD, 0xDE];
    let expected = 0xDEADBEEFu32;
    assert_merge(&bytes, Endianness::SmallEndian, Ok(expected));
}

#[test]
fn merge_u64_big_endian() {
    let bytes = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let expected = 0x0123456789ABCDEFu64;
    assert_merge(&bytes, Endianness::BigEndian, Ok(expected));
}

#[test]
fn merge_u64_small_endian() {
    let bytes = [0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01];
    let expected = 0x0123456789ABCDEFu64;
    assert_merge(&bytes, Endianness::SmallEndian, Ok(expected));
}