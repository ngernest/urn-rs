/* -------------------------------------------------------------------------- */
/*                              Type Definitions                              */
/* -------------------------------------------------------------------------- */

/// Weights are guaranteed to be non-negative since they're `u32`s
pub type Weight = u32;

/// Datatype for indexes (same as weights)
pub type Index = Weight;

/// Polymorphic binary trees, with a weight at each node/leaf.      
/// Invariant: `Node(w, l, r).weight() == l.weight() + r.weight()`
#[derive(Debug, PartialEq, Clone)]
pub enum Tree<T: Clone> {
    Leaf(Weight, T),
    Node(Weight, Box<Tree<T>>, Box<Tree<T>>),
}

/// An `Urn` is a `Tree`, along with its `size`
#[derive(Debug, PartialEq)]
pub struct Urn<T: Clone> {
    pub size: u32,
    pub tree: Tree<T>,
}

/* -------------------------------------------------------------------------- */
/*                             Methods for Tree<T>                            */
/* -------------------------------------------------------------------------- */

use Tree::*;

impl<T: Clone> Tree<T> {
    /// Retrieves the weight of a tree
    pub fn weight(&self) -> Weight {
        match self {
            Leaf(w, _) => *w,
            Node(w, _, _) => *w,
        }
    }

    /// Samples the value at index `i` from a `tree`
    pub fn sample(self, i: u32) -> T {
        match self {
            Leaf(_, a) => a.clone(),
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
    pub fn update<F>(
        &self,
        f: F,
        i: Index,
    ) -> ((Weight, &T), (Weight, &T), Self)
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
    pub fn replace(
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
