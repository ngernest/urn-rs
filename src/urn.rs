#![allow(dead_code)]

use crate::{
    almost_perfect::almost_perfect,
    types::{
        Index,
        Tree::{self, *},
        Urn, Weight,
    },
};
use rand::prelude::*;

/* -------------------------------------------------------------------------- */
/*                                   Helpers                                  */
/* -------------------------------------------------------------------------- */

/// Smart constructor for `Node`s
/// (automatically wraps the two subtrees in `Box`es)
fn node<T: Clone>(w: Weight, l: Tree<T>, r: Tree<T>) -> Tree<T> {
    Node(w, Box::new(l), Box::new(r))
}

/// Alias for the `Leaf` constructor
fn leaf<T: Clone>(w: Weight, a: T) -> Tree<T> {
    Leaf(w, a)
}

/// Tests whether the `n`-th bit of the `input` is set,
/// returning `true` if so and `false` otherwise
fn test_bit(input: u32, n: u32) -> bool {
    (input & (1 << n)) != 0
}

/// Produces a value uniformly at random from the range `[0, w]`
fn sample_weight(w: Weight) -> Weight {
    let mut rng = thread_rng();
    rng.gen_range(0..=w)
}

/* -------------------------------------------------------------------------- */
/*                             Methods for Urn<T>                             */
/* -------------------------------------------------------------------------- */

/// Creates a singleton urn containing element `a` with weight `w`
pub fn singleton<T: Clone>(w: Weight, a: T) -> Urn<T> {
    Urn {
        size: 1,
        tree: Leaf(w, a),
    }
}

/// Naive implementation of `from_list`, which just folds `insert` over a
/// vector of (weight, element) pairs.
/// TODO: add QC property that says `from_list` behaves the same as `from_list_naive`
pub fn from_list_naive<T: Clone>(elems: Vec<(Weight, T)>) -> Option<Urn<T>> {
    match elems.as_slice() {
        [] => None,
        [(w, a), ws @ ..] => Some(
            ws.iter()
                .fold(singleton(*w, a.clone()), |acc, (w_new, a_new)| {
                    acc.insert(*w_new, a_new.clone())
                }),
        ),
    }
}

/// An optimized version of `from_list`, which builds an almost perfect tree
/// in linear time (see `almost_perfect.rs`)
pub fn from_list<T: Clone>(elems: Vec<(Weight, T)>) -> Option<Urn<T>> {
    if elems.is_empty() {
        None
    } else {
        Some(Urn {
            size: elems.len() as u32,
            tree: almost_perfect(elems),
        })
    }
}

/* -------------------------------------------------------------------------- */
/*                Deterministic (index-based) methods for Urns                */
/* -------------------------------------------------------------------------- */

impl<T: Clone> Urn<T> {
    /// Fetches the `size` of the urn
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Same as the `weight` method for `Tree<T>`
    pub fn weight(&self) -> Weight {
        self.tree.weight()
    }

    /// Same as the `sample_index` method for `Tree<T>`
    fn sample_index(self, i: Index) -> T {
        self.tree.sample_index(i)
    }

    /// Same as the `update_index` method for `Tree<T>`
    fn update_index<F>(
        &self,
        f: F,
        i: Index,
    ) -> ((Weight, &T), (Weight, &T), Self)
    where
        F: Fn(Weight, &T) -> (Weight, &T),
    {
        let (old, new, new_tree) = self.tree.update_index(f, i);
        (
            old,
            new,
            Urn {
                tree: new_tree,
                ..*self
            },
        )
    }

    /// Same as the `replace` method for `Tree<T>`
    fn replace_index(
        &self,
        w: Weight,
        a: &T,
        i: Index,
    ) -> ((Weight, &T), Self) {
        let (old, new_tree) = self.tree.replace_index(w, a, i);
        (
            old,
            Urn {
                tree: new_tree,
                ..*self
            },
        )
    }

    /// Inserts a new element `a` with weight `w` into the `Urn`
    pub fn insert(self, w_outer: Weight, a_outer: T) -> Self {
        /// Helper function which updates the weights on all the
        /// nodes encountered on a `path` through the `tree`.              
        /// (The `path` is the binary representation of an integer,
        /// where 0 is Left and 1 is right. We toggle the direction every time
        /// we insert a new node to ensure that the tree is almost balanced.
        /// See section 3.4-3.5 of the paper for details.)              
        /// Note: since recursive closures aren't really possible
        /// in Rust, and since nested functions can't access outer variables,
        /// we need to supply the `w_outer` and `a_outer` arguments explicitly.
        fn go<T: Clone>(
            w_outer: Weight,
            a_outer: T,
            path: u32,
            tree: Tree<T>,
        ) -> Tree<T> {
            match tree {
                Leaf(w, a) => node(
                    w.wrapping_add(w_outer),
                    leaf(w, a),
                    leaf(w_outer, a_outer),
                ),
                Node(w, l, r) => {
                    let new_path = path >> 1;
                    if test_bit(path, 0) {
                        node(
                            w.wrapping_add(w_outer),
                            *l,
                            go(w_outer, a_outer, new_path, *r),
                        )
                    } else {
                        node(
                            w.wrapping_add(w_outer),
                            go(w_outer, a_outer, new_path, *l),
                            *r,
                        )
                    }
                }
            }
        }

        Urn {
            size: self.size + 1,
            tree: go(w_outer, a_outer, self.size, self.tree),
        }
    }

    /// `uninsert`s (deletes) the most-recently-inserted weighted value `(w, a)`
    /// from the urn, returning `(w, a)`, the lower bound `lb` for the bucket
    /// that previously contained `a`, and an optional new urn
    /// (since `uninsert`-ing from an `Urn` of size 1 produces `None`)
    fn uninsert(self) -> ((Weight, T), Weight, Option<Self>) {
        fn go<T: Clone>(
            path: u32,
            tree: Tree<T>,
        ) -> ((Weight, T), Weight, Option<Tree<T>>) {
            match tree {
                Leaf(w, a) => ((w, a), 0, None),
                Node(w, l, r) => {
                    let new_path = path >> 1;
                    if test_bit(path, 0) {
                        let ((w_new, a_new), lb, r_opt) = go(new_path, *r);
                        let new_tree = r_opt.map_or(*l.clone(), |r_new| {
                            Node(w - w_new, l, Box::new(r_new))
                        });
                        ((w_new, a_new), lb, Some(new_tree))
                    } else {
                        let ((w_new, a_new), lb, l_opt) = go(new_path, *l);
                        let new_tree = l_opt.map_or(*r.clone(), |l_new| {
                            Node(w - w_new, Box::new(l_new), r)
                        });
                        ((w_new, a_new), lb, Some(new_tree))
                    }
                }
            }
        }

        let ((w, a), lb, tree_opt) = go(self.size - 1, self.tree);
        (
            (w, a),
            lb,
            tree_opt.map(|tree| Self {
                size: self.size - 1,
                tree,
            }),
        )
    }

    /// Removes the element at index `i` in the urn, returning the element,
    /// its weight, and an optional new urn
    fn remove_index(self, i: Index) -> ((Weight, T), Option<Self>) {
        let ((w, a), lb, urn_opt) = self.uninsert();
        match urn_opt {
            None => ((w, a), None),
            Some(new_urn) => {
                if i < lb {
                    let ((w_new, a_new), final_urn) =
                        new_urn.replace_index(w, &a, i);
                    ((w_new, a_new.clone()), Some(final_urn))
                } else if i < lb + w {
                    ((w, a), Some(new_urn))
                } else {
                    let ((w_new, a_new), final_urn) =
                        new_urn.replace_index(w, &a, i - w);
                    ((w_new, a_new.clone()), Some(final_urn))
                }
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                           Random methods for Urns                          */
/* -------------------------------------------------------------------------- */

impl<T: Clone> Urn<T> {
    pub fn sample(self) -> T {
        let i = sample_weight(self.weight());
        self.sample_index(i)
    }

    pub fn update<F>(&self, f: F) -> ((Weight, &T), (Weight, &T), Self)
    where
        F: Fn(Weight, &T) -> (Weight, &T),
    {
        let i = sample_weight(self.weight());
        self.update_index(f, i)
    }

    pub fn replace(&self, w: Weight, a: &T) -> ((Weight, &T), Self) {
        let i = sample_weight(self.weight());
        self.replace_index(w, a, i)
    }

    pub fn remove(self) -> ((Weight, T), Option<Self>) {
        let i = sample_weight(self.weight());
        self.remove_index(i)
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Tests                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    /// Example from figure 5 in the paper
    #[test]
    fn sample_example() {
        let tree = node(
            21,
            node(
                9,
                node(5, leaf(4, 'a'), leaf(1, 'b')),
                node(4, leaf(2, 'c'), leaf(2, 'd')),
            ),
            node(
                12,
                node(7, leaf(2, 'e'), leaf(5, 'f')),
                node(5, leaf(3, 'g'), leaf(2, 'h')),
            ),
        );
        let expected = 'f';
        let actual = tree.sample_index(12);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_list_equiv_small() {
        let elems = vec![(2, 'R'), (4, 'G'), (3, 'B')];
        let naive_urn = from_list_naive(elems.clone()).unwrap();
        let urn = from_list(elems).unwrap();
        assert_eq!(naive_urn.size(), urn.size());
        assert_eq!(naive_urn.weight(), urn.weight());
    }

    #[test]
    fn from_list_equiv_big() {
        let elems = vec![
            (1, 'a'),
            (2, 'b'),
            (3, 'c'),
            (4, 'd'),
            (5, 'e'),
            (6, 'f'),
            (7, 'g'),
            (8, 'h'),
        ];
        let naive_urn = from_list_naive(elems.clone()).unwrap();
        let urn = from_list(elems).unwrap();
        assert_eq!(naive_urn.size(), urn.size());
        assert_eq!(naive_urn.weight(), urn.weight());
    }
}
