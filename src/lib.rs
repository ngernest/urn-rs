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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}