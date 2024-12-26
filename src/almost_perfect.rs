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
        // Case `|n| < |x|` : n=3, x=14 (0b1110) -> 0b111
        // Leftmost 1 in `0b1110` is ignored
        assert_eq!(reverse_bits(3, 0b1110), 0b011);

        // Case `|n| = |x|` : n=4, x=12 (0b1100) -> 0b0011
        // Reverses the entire binary representation of `x`
        assert_eq!(reverse_bits(4, 0b1100), 0b0011);

        // Case `|n| > |x|` : n=5, x=3 (0b11) -> 0b11000
        // `reverse_bits` adds trailing zeroes in this case
        assert_eq!(reverse_bits(5, 0b11), 0b11000);

        // Case `x = 0` : n=3, x=0 (0b000) -> 0b000
        // Ensures that 000 is handled correctly
        assert_eq!(reverse_bits(3, 0b000), 0b000);

        // Case `|n| = |x| = 1`: n=1, x=1 (0b1) -> 0b1
        // Reversing one bit does nothing
        assert_eq!(reverse_bits(1, 0b1), 0b1);
    }
}
