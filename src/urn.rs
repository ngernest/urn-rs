#![allow(dead_code)]

/* -------------------------------------------------------------------------- */
/*                              Type Definitions                              */
/* -------------------------------------------------------------------------- */

/// Weights are guaranteed to be non-negative since they're `u32`s
type Weight = u32;

/// Datatype for indexes (same as weights)
type Index = Weight;

/// Polymorphic binary trees, with a weight at each node/leaf.      
/// Invariant: `Node(w, l, r).weight() == l.weight() + r.weight()`
#[derive(Debug, PartialEq, Clone)]
enum Tree<T: Clone> {
    Leaf(Weight, T),
    Node(Weight, Box<Tree<T>>, Box<Tree<T>>),
}

/// An `Urn` is a `Tree`, along with its `size`
struct Urn<T: Clone> {
    size: u32,
    tree: Tree<T>,
}

use Tree::*;

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

/* -------------------------------------------------------------------------- */
/*                             Methods for Urn<T>                             */
/* -------------------------------------------------------------------------- */

/// Creates a singleton urn containing element `a` with weight `w`
fn singleton<T: Clone>(w: Weight, a: T) -> Urn<T> {
    Urn {
        size: 1,
        tree: Leaf(w, a),
    }
}

/// Naive implementation of `from_list`, which just folds `insert` over a
/// vector of (weight, element) pairs.
/// TODO: add QC property that says `from_list` behaves the same as `from_list_naive`
fn from_list_naive<T: Clone>(elts: Vec<(Weight, T)>) -> Option<Urn<T>> {
    match elts.as_slice() {
        [] => None,
        [(w, a), ws @ ..] => Some(
            ws.iter()
                .fold(singleton(*w, a.clone()), |acc, (w_new, a_new)| {
                    acc.insert(*w_new, a_new.clone())
                }),
        ),
    }
}

impl<T: Clone> Urn<T> {
    /// Fetches the `size` of the urn
    fn size(&self) -> u32 {
        self.size
    }

    /// Same as the `weight` method for `Tree<T>`
    fn weight(&self) -> Weight {
        self.tree.weight()
    }

    /// Same as the `sample` method for `Tree<T>`
    fn sample(&self, i: u32) -> &T {
        self.tree.sample(i)
    }

    /// Same as the `update` method for `Tree<T>`
    fn update<F>(&self, f: F, i: Index) -> ((Weight, &T), (Weight, &T), Self)
    where
        F: Fn(Weight, &T) -> (Weight, &T),
    {
        let (old, new, new_tree) = self.tree.update(f, i);
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
    fn replace(&self, w: Weight, a: &T, i: Index) -> ((Weight, &T), Self) {
        let (old, new_tree) = self.tree.replace(w, a, i);
        (
            old,
            Urn {
                tree: new_tree,
                ..*self
            },
        )
    }

    /// Inserts a new element `a` with weight `w` into the `Urn`
    fn insert(self, w_outer: Weight, a_outer: T) -> Self {
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
                Leaf(w, a) => {
                    node(w + w_outer, leaf(w, a), leaf(w_outer, a_outer))
                }
                Node(w, l, r) => {
                    let new_path = path >> 1;
                    if test_bit(path, 0) {
                        node(
                            w + w_outer,
                            *l,
                            go(w_outer, a_outer, new_path, *r),
                        )
                    } else {
                        node(
                            w + w_outer,
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
    fn remove(self, i: Index) -> ((Weight, T), Option<Self>) {
        let ((w, a), lb, urn_opt) = self.uninsert();
        match urn_opt {
            None => ((w, a), None),
            Some(new_urn) => {
                if i < lb {
                    let ((w_new, a_new), final_urn) = new_urn.replace(w, &a, i);
                    ((w_new, a_new.clone()), Some(final_urn))
                } else if i < lb + w {
                    ((w, a), Some(new_urn))
                } else {
                    let ((w_new, a_new), final_urn) =
                        new_urn.replace(w, &a, i - w);
                    ((w_new, a_new.clone()), Some(final_urn))
                }
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                             Methods for Tree<T>                            */
/* -------------------------------------------------------------------------- */

impl<T: Clone> Tree<T> {
    /// Retrieves the weight of a tree
    fn weight(&self) -> Weight {
        match self {
            Leaf(w, _) => *w,
            Node(w, _, _) => *w,
        }
    }

    /// Samples the value at index `i` from a `tree`
    fn sample(&self, i: u32) -> &T {
        match self {
            Leaf(_, a) => a,
            Node(_, l, r) => {
                let wl = l.weight();
                if i < wl {
                    l.sample(i)
                } else {
                    r.sample(i - wl)
                }
            }
        }
    }

    /// `t.update(f, i)` samples an element from the tree `t`, then replaces the
    /// chosen element `a` and its weight `w` by a new element `a_new`
    /// with weight `w_new`, where `(w_new, a_new) = f(w, a)`.    
    /// This function returns a triple `((w, a), (w_new, a_new), t_new)`,
    /// where `t_new` is the same tree as `t`,
    /// but with `(w, a)` replaced by `(w_new, a_new)`.
    fn update<F>(&self, f: F, i: Index) -> ((Weight, &T), (Weight, &T), Self)
    where
        F: FnOnce(Weight, &T) -> (Weight, &T),
    {
        match self {
            Leaf(w, a) => {
                let (w_new, a_new) = f(*w, a);
                ((*w, a), (w_new, a_new), Leaf(w_new, a_new.clone()))
            }
            Node(w, l, r) => {
                let wl = l.weight();
                if i < wl {
                    let (old, new, l_new) = l.update(f, i);
                    (
                        old,
                        new,
                        Node(w - old.0 + new.0, Box::new(l_new), r.clone()),
                    )
                } else {
                    let (old, new, r_new) = r.update(f, i - wl);
                    (
                        old,
                        new,
                        Node(w - old.0 + new.0, l.clone(), Box::new(r_new)),
                    )
                }
            }
        }
    }

    /// Samples from the tree, and returns the sampled element and its weight,
    /// along with a new tree with the sampled elements removed and a new element
    /// `a` with weight `w` added.
    fn replace(
        &self,
        w_outer: Weight,
        a_outer: &T,
        i: Index,
    ) -> ((Weight, &T), Self) {
        match self {
            Leaf(w, a) => ((*w, a), Leaf(w_outer, a_outer.clone())),
            Node(w, l, r) => {
                let wl = l.weight();
                if i < wl {
                    let (old, l_new) = l.replace(w_outer, a_outer, i);
                    (old, Node(w - old.0 + w_outer, Box::new(l_new), r.clone()))
                } else {
                    let (old, r_new) = r.replace(w_outer, a_outer, i - wl);
                    (old, Node(w - old.0 + w_outer, l.clone(), Box::new(r_new)))
                }
            }
        }
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
        let &actual = tree.sample(12);
        assert_eq!(expected, actual);
    }
}
