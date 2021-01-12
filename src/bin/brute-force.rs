#![feature(with_options)]

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::io::{self, prelude::*, BufReader};
use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    path::PathBuf,
};
use structopt::StructOpt;

use anyhow::Result;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{:#?}", &opt);

    let file = File::open(opt.file)?;
    let reader = BufReader::new(file);

    let mut cnt = HashMap::new();

    for line in reader.lines() {
        let key = line?;
        if let Some(c) = cnt.get_mut(&key) {
            *c += 1;
        } else {
            cnt.insert(key, 1);
        }
    }

    let maxk = 10;
    let mut heap: BinaryHeap<(i32, String)> = BinaryHeap::new();
    for (k, cnt) in cnt.iter() {
        heap.push((-cnt, k.to_owned()));
        if heap.len() > maxk {
            heap.pop();
        }
    }

    println!("{:#?}", heap);

    Ok(())
}
