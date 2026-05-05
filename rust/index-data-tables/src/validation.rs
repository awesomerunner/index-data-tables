/**
 * Takes in a slice and returns the culumative XOR (parity) of its elements
 * Used for parity validations
 * Requires that the elements implement Clone and BitXorAssign
 */
pub fn xor_slice<T> (elements: &[T]) -> T 
where
    T: std::ops::BitXorAssign + Clone
{
    let mut cumulative_xor: T = elements[0].clone();
    for element in &elements[1..] {
        cumulative_xor ^= element.clone();
    }
    cumulative_xor
}