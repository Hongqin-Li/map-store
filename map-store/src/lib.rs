#![deny(missing_docs)]
#![feature(num_as_ne_bytes)]
#![feature(with_options)]
//! MapReduce-based key-value storage.

mod batch_writer;
mod map_store;
mod operator;

pub use crate::map_store::MapStore;
pub use batch_writer::BatchWriter;
pub use operator::Operator;
