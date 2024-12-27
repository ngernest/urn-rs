use crate::types::{Tree, Tree::*, Urn, Weight};
use crate::urn;
use quickcheck::*;

/* -------------------------------------------------------------------------- */
/*                                 Generators                                 */
/* -------------------------------------------------------------------------- */

/// QuickCheck generator for `Urn`s with at most size 10
impl Arbitrary for Urn<char> {
    fn arbitrary(_g: &mut Gen) -> Self {
        let elems = Vec::<(Weight, char)>::arbitrary(&mut Gen::new(10));
        let default = urn::singleton(1, 'a');
        urn::from_list(elems).unwrap_or(default)
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Properties                                 */
/* -------------------------------------------------------------------------- */

// Properties are adapted from the Coq lemmas in
// https://github.com/antalsz/urn-random/blob/master/coq/urn.v

impl<T: Clone> Tree<T> {
    /// Counts the no. of nodes in the tree
    fn tree_count(&self) -> u32 {
        match self {
            Leaf(_, _) => 1,
            Node(_, l, r) => l.tree_count().wrapping_add(r.tree_count()),
        }
    }

    /// Sums the weights at all the leaves
    fn sum_leaf_weights(&self) -> Weight {
        match self {
            Leaf(w, _) => *w,
            Node(_, l, r) => {
                l.sum_leaf_weights().wrapping_add(r.sum_leaf_weights())
            }
        }
    }

    /// Checks whether the weight at each node matches the sum of
    /// the subtrees' leaf weights
    fn weights_match(&self) -> bool {
        match self {
            Leaf(_, _) => true,
            Node(w, l, r) => {
                *w == l.sum_leaf_weights().wrapping_add(r.sum_leaf_weights())
                    && l.weights_match()
                    && r.weights_match()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    // Ensure that all urns produced using `from_list` are well-formed
    #[quickcheck]
    fn well_formed_urn(urn: Urn<char>) -> bool {
        urn.tree.tree_count() == urn.size() && urn.tree.weights_match()
    }

    // Ensure that `from_list` produces equivalent urns as `from_list_naive`
    // (i.e. the urns have the same size & same cumulative weight)
    #[quickcheck]
    fn from_list_equivalent_to_from_list_naive(
        elems: Vec<(Weight, char)>,
    ) -> bool {
        let default = urn::singleton(1, 'a');
        let urn = urn::from_list(elems.clone()).unwrap_or(default.clone());
        let naive_urn = urn::from_list_naive(elems).unwrap_or(default);

        urn.size() == naive_urn.size() && urn.weight() == naive_urn.weight()
    }

    // TODO: add some other properties:
    // - `insert` preserves well-formedness
    // - `insert` increases `size` by 1
    // - `uninsert` preserves well-formedness
    // - `uninsert` decreases `size` by 1
    // - `replace` preserves well-formedness
}
