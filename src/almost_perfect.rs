// use crate::types::{Tree, Tree::*};

#![allow(dead_code)]

/// Reverses the lowest `n` bits of the number `x`
fn reverse_bits(n: u32, x: u32) -> u32 {
    /// Helper function, where `r` is the accumulator
    /// that builds up the reversed bits (the result).     
    /// In each recursive call, we:
    /// - Shift `r` left to make room for a new bit
    /// - Extract the LSB from `x` and adds it to `r`
    /// - Decrements our counter `n` (terminating when `n = 0`)
    /// - Shift `x` right to set it up for the next bit
    fn go(r: u32, n: u32, x: u32) -> u32 {
        if n == 0 {
            r
        } else {
            go(r << 1 | x & 1, n - 1, x >> 1)
        }
    }
    go(0, n, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        let expected = 0b011;
        let actual = reverse_bits(3, 0b110);
        assert_eq!(expected, actual)
    }
}
