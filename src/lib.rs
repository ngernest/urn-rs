#![allow(dead_code)]

/// The datatype for weights (`u32` for now)
type Weight = u32;

/// Datatype for indexes (same as weights)
type Index = Weight;

/// Polymorphic binary trees, with a weight at each node/leaf
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
}

/* -------------------------------------------------------------------------- */
/*                             Methods for Tree<T>                            */
/* -------------------------------------------------------------------------- */

impl<T: Clone> Tree<T> {
    /// Retrieves the weight of a tree.
    /// Invariant: `Node(w, l, r).weight() == l.weight() + r.weight()`
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
    use Tree::*;

    /// Example from figure 5 in the paper
    #[test]
    fn sample_example() {
        let tree = node(
            21,
            node(
                9,
                node(5, Leaf(4, 'a'), Leaf(1, 'b')),
                node(4, Leaf(2, 'c'), Leaf(2, 'd')),
            ),
            node(
                12,
                node(7, Leaf(2, 'e'), Leaf(5, 'f')),
                node(5, Leaf(3, 'g'), Leaf(2, 'h')),
            ),
        );
        let expected = 'f';
        let &actual = tree.sample(12);
        assert_eq!(expected, actual);
    }
}
