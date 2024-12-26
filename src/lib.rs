#![allow(dead_code)]

/// The datatype for weights (`u32` for now)
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

/// Smart constructor for `Node`s
/// (automatically wraps the two subtrees in `Box`es)
fn node<T: Clone>(w: Weight, l: Tree<T>, r: Tree<T>) -> Tree<T> {
    Tree::Node(w, Box::new(l), Box::new(r))
}

/// Alias for the `Leaf` constructor
fn leaf<T: Clone>(w: Weight, a: T) -> Tree<T> {
    Tree::Leaf(w, a)
}

/// Tests whether the `n`-th bit of the `input` is set or not
fn test_bit(input: u32, n: u32) -> bool {
    (input & (1 << n)) != 0
}

/// An `Urn` is a `Tree`, along with its `size`
struct Urn<T: Clone> {
    size: u32,
    tree: Tree<T>,
}

/* -------------------------------------------------------------------------- */
/*                             Methods for Urn<T>                             */
/* -------------------------------------------------------------------------- */

impl<T: Clone> Urn<T> {
    /// Same as the `weight` method for `Tree<T>`
    fn weight(&self) -> Weight {
        self.tree.weight()
    }

    /// Same as the `sample` method for `Tree<T>`
    fn sample(&self, i: u32) -> &T {
        self.tree.sample(i)
    }

    /// Same as the `update` method for `Tree<T>`
    fn update<F>(&self, upd: F, i: Index) -> ((Weight, &T), (Weight, &T), Self)
    where
        F: for<'a> Fn(Weight, &'a T) -> (Weight, &'a T),
    {
        let (old, new, new_tree) = self.tree.update(upd, i);
        (
            old,
            new,
            Urn {
                size: self.size,
                tree: new_tree,
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
                Tree::Leaf(w, a) => {
                    node(w + w_outer, leaf(w, a), leaf(w_outer, a_outer))
                }
                Tree::Node(w, l, r) => {
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
}

/* -------------------------------------------------------------------------- */
/*                             Methods for Tree<T>                            */
/* -------------------------------------------------------------------------- */

impl<T: Clone> Tree<T> {
    /// Retrieves the weight of a tree
    fn weight(&self) -> Weight {
        match self {
            Tree::Leaf(w, _) => *w,
            Tree::Node(w, _, _) => *w,
        }
    }

    /// Samples the value at index `i` from a `tree`
    fn sample(&self, i: u32) -> &T {
        match self {
            Tree::Leaf(_, a) => a,
            Tree::Node(_, l, r) => {
                let wl = l.weight();
                if i < wl {
                    l.sample(i)
                } else {
                    r.sample(i - wl)
                }
            }
        }
    }

    /// Updates the element at index `i` using the functino `upd`, returning
    /// the old and new (weight, element) pairs, as wel as the updated tree
    /// (`upd` needs to implement the `Fn` trait, since we want to be able
    /// to call it repeatedly without mutating state)
    fn update<F>(&self, upd: F, i: Index) -> ((Weight, &T), (Weight, &T), Self)
    where
        F: for<'a> Fn(Weight, &'a T) -> (Weight, &'a T),
    {
        match self {
            Tree::Leaf(w, a) => {
                let (w_new, a_new) = upd(*w, a);
                ((*w, a), (w_new, a_new), Tree::Leaf(w_new, a_new.clone()))
            }
            Tree::Node(w, l, r) => {
                let wl = l.weight();
                if i < wl {
                    let (old, new, l_new) = l.update(upd, i);
                    (
                        old,
                        new,
                        Tree::Node(
                            w - old.0 + new.0,
                            Box::new(l_new),
                            r.clone(),
                        ),
                    )
                } else {
                    let (old, new, r_new) = r.update(upd, i - wl);
                    (
                        old,
                        new,
                        Tree::Node(
                            w - old.0 + new.0,
                            l.clone(),
                            Box::new(r_new),
                        ),
                    )
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
