#![allow(dead_code)]

/// The datatype for weights (`u32` for now)
type Weight = u32;

/// Datatype for indexes (same as weights)
type Index = Weight;

/// Polymorphic binary trees, with a weight at each node/leaf
#[derive(Debug, PartialEq)]
enum Tree<T> {
    Leaf(Weight, T),
    Node(Weight, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T> Tree<T> {
    /// Retrieves the weight of a tree
    fn weight(&self) -> &Weight {
        match self {
            Tree::Leaf(w, _) => w,
            Tree::Node(w, _, _) => w,
        }
    }

    /// Samples the value at index `i` from a `tree`
    fn sample(&self, i: u32) -> &T {
        match self {
            Tree::Leaf(_, a) => a,
            Tree::Node(_, l, r) => {
                let wl = l.weight();
                if i < *wl {
                    l.sample(i)
                } else {
                    r.sample(i - wl)
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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
