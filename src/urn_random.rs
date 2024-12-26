#![allow(dead_code, unused_imports)]

use crate::types::{Index, Tree, Tree::*, Urn, Weight};
use crate::urn_deterministic::*;
use rand::prelude::*;

/// Produces a value uniformly at random from the range `[0, w]`
fn sample_weight(w: Weight) -> Weight {
    let mut rng = thread_rng();
    rng.gen_range(0..=w)
}

fn sampler<F, T>(f: F, urn: Urn<T>) -> T
where
    T: Clone,
    F: FnOnce(&Urn<T>, Weight) -> T,
{
    f(&urn, sample_weight(urn.weight()))
}

fn sample_index<T: Clone>(urn: Urn<T>, i: Index) -> T {
    urn.tree.sample(i)
}
