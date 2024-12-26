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
pub struct Urn<T: Clone> {
    pub size: u32,
    pub tree: Tree<T>,
}
