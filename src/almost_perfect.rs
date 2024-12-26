#![allow(dead_code)]

use core::panic;

use crate::types::{Tree, Tree::*, Weight};

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

/// Smart constructor: builds a `Node` whose weight is the
/// sum of the two subtree's weights
fn node<T: Clone>(l: Tree<T>, r: Tree<T>) -> Tree<T> {
    Node(l.weight() + r.weight(), Box::new(l), Box::new(r))
}

/// Alias for the `Leaf` constructor
fn leaf<T: Clone>(w: Weight, a: T) -> Tree<T> {
    Leaf(w, a)
}

/// Builds an almost perfect tree using the weights and values in `elems`
pub fn almost_perfect<T: Clone>(elems: Vec<(Weight, T)>) -> Tree<T> {
    /// Helper function: recurses on the current `depth` of the tree
    /// and the array `elem`s, either inserting two elements at a time
    /// or one at a time
    fn go<T: Clone>(
        depth: u32,
        index: u32,
        elems: &[(Weight, T)],
        og_size: u32,
        perfect_depth: u32,
        remainder: u32,
    ) -> (Tree<T>, &[(Weight, T)], u32) {
        if depth == 0 {
            if reverse_bits(perfect_depth, index) < remainder {
                match elems {
                    [(wl, tl), (wr, tr), tail @ ..] => (
                        node(leaf(*wl, tl.clone()), leaf(*wr, tr.clone())),
                        tail,
                        index + 1,
                    ),
                    _ => panic!(
                        "Expected size {} but got input of length {} instead",
                        og_size,
                        elems.len()
                    ),
                }
            } else {
                match elems {
                    [(w, x), tail @ ..] => {
                        (leaf(*w, x.clone()), tail, index + 1)
                    }
                    _ => panic!(
                        "Expected size {} but got input of length {} instead",
                        og_size,
                        elems.len()
                    ),
                }
            }
        } else {
            let (l, l_elems, l_index) =
                go(depth - 1, index, elems, og_size, perfect_depth, remainder);
            let (r, r_elems, r_index) = go(
                depth - 1,
                l_index,
                l_elems,
                og_size,
                perfect_depth,
                remainder,
            );
            (node(l, r), r_elems, r_index)
        }
    }

    let original_size = elems.len() as u32;

    // `ilog2` computes the floor of `size.log2()`
    let perfect_depth = original_size.ilog2();
    let remainder = original_size - (perfect_depth << 1);
    let depth = perfect_depth;
    let index = 0;
    let (tree, _, _) = go(
        depth,
        index,
        elems.as_slice(),
        original_size,
        perfect_depth,
        remainder,
    );
    tree
}

/* -------------------------------------------------------------------------- */
/*                                    Tests                                   */
/* -------------------------------------------------------------------------- */

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
