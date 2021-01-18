#![deny(missing_docs)]
#![feature(with_options)]
#![feature(test)]
//! Solutions for top K frequent string.

extern crate test;

use std::{collections::HashMap, path::Path};

mod brute_force;
mod dataset;
mod map_reduce;
mod map_store;
mod mink_set;

pub use crate::map_store::MapStore;
pub use brute_force::BruteForce;
pub use dataset::Generator;
pub use map_reduce::MapReduce;

/// A trait of solutions.
pub trait Solution {
    /// Solve a top k frequent string problem with k of `topk` and path to a input file.
    ///
    /// The file contains lines of strings. This function should return a `HashMap` of size
    /// smaller than `topk`, mapping from top-k strings to their frequency.
    fn solve(&self, topk: u32, path: impl AsRef<Path>) -> HashMap<String, u32>;
}
