use std::ops::{BitOr, Shl};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Endianness {
    BigEndian,
    SmallEndian
}

pub fn merge_bytes<T> (bytes: &[u8], endianness: Endianness) -> Result<T, String>
where
    T: BitOr<Output = T> + Shl<usize, Output = T> + From<u8>
{
    if bytes.len() > size_of::<T>() {
        return Err("Byte sequence cannot be merged".to_string());
    }

    let mut combined: T = 0_u8.into();
    let mut next_shift: usize = if endianness == Endianness::BigEndian {
        size_of::<T>()
    } else {
        0
    };
    
    for byte in bytes {
        if endianness == Endianness::BigEndian {
            next_shift -= 1;
        }
        combined = (T::from(byte.clone()) << (next_shift * 8)) | combined;
        if endianness == Endianness::SmallEndian {
            next_shift += 1
        }
    }

    Ok(combined)
}

