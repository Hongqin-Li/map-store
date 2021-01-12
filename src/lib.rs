#![feature(with_options)]

pub mod hashkv;

mod batch_writer;
mod merkle;

pub use batch_writer::BatchWriter;
