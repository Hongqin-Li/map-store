#![feature(with_options)]
#![feature(test)]
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

pub trait Solution {
    fn solve(&self, topk: u32, path: impl AsRef<Path>) -> HashMap<String, u32>;
}
