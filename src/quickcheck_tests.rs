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

impl<T: Clone> Urn<T> {
    /// Checks whether an urn is well-formed (WF)
    fn is_wf(&self) -> bool {
        self.tree.tree_count() == self.size() && self.tree.weights_match()
    }
}

#[cfg(test)]
mod qc_tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    // Ensure that all urns produced using `from_list` are well-formed
    #[quickcheck]
    fn from_list_produces_wf_urns(elems: Vec<(Weight, char)>) -> bool {
        urn::from_list(elems).map_or(true, |urn| urn.is_wf())
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

    #[quickcheck]
    fn insert_preserves_wf(urn: Urn<char>, w: Weight, a: char) -> bool {
        urn.is_wf() && urn.insert(w, a).is_wf()
    }

    #[quickcheck]
    fn insert_increments_size(urn: Urn<char>, w: Weight, a: char) -> bool {
        urn.size() + 1 == urn.clone().insert(w, a).size()
    }

    #[quickcheck]
    fn uninsert_preserves_wf(urn: Urn<char>) -> bool {
        let (_, _, new_urn) = urn.clone().uninsert();
        urn.is_wf() && new_urn.map_or(true, |u| u.is_wf())
    }

    #[quickcheck]
    fn uninsert_decrements_size(urn: Urn<char>) -> bool {
        let (_, _, new_urn) = urn.clone().uninsert();
        urn.size() - 1 == new_urn.map_or(0, |u| u.size())
    }

    #[quickcheck]
    fn replace_preserves_wf(urn: Urn<char>, w: Weight, a: char) -> bool {
        let (_, new_urn) = urn.replace(w, &a);
        urn.is_wf() && new_urn.is_wf()
    }

    #[quickcheck]
    fn replace_preserves_size(urn: Urn<char>, w: Weight, a: char) -> bool {
        let (_, new_urn) = urn.replace(w, &a);
        urn.size() == new_urn.size()
    }

    #[quickcheck]
    fn remove_preserves_wf(urn: Urn<char>) -> bool {
        let (_, new_urn) = urn.clone().remove();
        urn.is_wf() && new_urn.map_or(true, |u| u.is_wf())
    }

    #[quickcheck]
    fn remove_decrements_size(urn: Urn<char>) -> bool {
        let ((_, _), new_urn) = urn.clone().remove();
        urn.size() - 1 == new_urn.map_or(0, |u| u.size())
    }

    // `uninsert` retrieves the most recently inserted (weight, element) pair
    #[quickcheck]
    fn insert_uninsert(urn: Urn<char>, w: Weight, a: char) -> bool {
        let new_urn = urn.clone().insert(w, a);
        let ((w_new, a_new), _, u_opt) = new_urn.uninsert();
        urn.is_wf() && (w_new, a_new, u_opt) == (w, a, Some(urn))
    }
}
