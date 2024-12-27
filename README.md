# Random Urns
A Rust implementation of [*Urns*](https://hackage.haskell.org/package/urn-random) [(Lampropulos et al. 2017)](https://dl.acm.org/doi/pdf/10.1145/3122955.3122959), a data structure which allows for random sampling and updating discrete distributions in logarithmic time. 

For details about the design behind urns, we refer the reader to the [original paper](https://dl.acm.org/doi/pdf/10.1145/3122955.3122959), or the [Haskell Symposium '17 talk](https://www.youtube.com/watch?v=O37FMxLxm78&t=1166s).

This implementation has been adapted from:
- Lampropulos et al.'s [original Haskell implementation](https://github.com/antalsz/urn-random)           
- Justin Frank's [OCaml implementation](https://github.com/laelath/ocaml-urn)     
(I've tried to follow the Haskell/OCaml implementations as close as possible.)

## Code overview
To compile, run `cargo build`.        
To run unit tests + QuickCheck tests, run `cargo test`.       

- [`types.rs`](./src/types.rs): Type definitions
- [`urn.rs`](./src/urn.rs): Methods for interacting with urns 
- [`almost_perfect.rs`](./src/almost_perfect.rs): *Almost perfect* trees (used in the construction of urns)
- [`quickcheck_tests.rs`](./src/quickcheck_tests.rs): QuickCheck properties for urns 
