use crate::types::{Tree, Tree::*, Urn};
use quickcheck::*;

/* -------------------------------------------------------------------------- */
/*                                 Generators                                 */
/* -------------------------------------------------------------------------- */

impl Arbitrary for Tree<char> {
    fn arbitrary(g: &mut Gen) -> Self {
        todo!()
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Properties                                 */
/* -------------------------------------------------------------------------- */

impl<T: Clone> Tree<T> {
    /// Counts the no. of nodes in the tree
    fn tree_count(&self) -> u32 {
        match self {
            Leaf(_, _) => 1,
            Node(_, l, r) => l.tree_count() + r.tree_count(),
        }
    }

    /// Sums the weights at all the leaves
    fn sum_leaf_weights(&self) -> u32 {
        match self {
            Leaf(w, _) => *w,
            Node(_, l, r) => l.sum_leaf_weights() + r.sum_leaf_weights(),
        }
    }

    /// Checks whether the weight at each node matches the sum of
    /// the subtrees' leaf weights
    fn weights_match(&self) -> bool {
        match self {
            Leaf(_, _) => true,
            Node(w, l, r) => {
                *w == l.sum_leaf_weights() + r.sum_leaf_weights()
                    && l.weights_match()
                    && r.weights_match()
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[quickcheck]
//     fn well_formed_urn(urn: Urn<char>) -> bool {
//         urn.tree.tree_count() == urn.size() && urn.tree.weights_match()
//     }
// }
