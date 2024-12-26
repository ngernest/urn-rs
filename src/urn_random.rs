use crate::types::Weight;
use crate::urn_deterministic::*;
use rand::distributions::Uniform;
use rand::prelude::*;

/// Produces a value uniformly at random from the range `[0, w]`
fn sample_weight(w: Weight) -> Weight {
    let mut rng = thread_rng();
    rng.sample(Uniform::new_inclusive(0, w))
}
