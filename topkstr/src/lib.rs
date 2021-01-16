#![feature(with_options)]

use std::{collections::HashMap, path::Path};

mod brute_force;
mod dataset;
mod map_reduce;
mod map_store;
mod mink_set;

pub use dataset::Generator;

pub trait Solution {
    fn solve(&self, topk: u32, path: impl AsRef<Path>) -> HashMap<String, u32>;
}
