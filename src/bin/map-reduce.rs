#![feature(with_options)]

use log::info;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self, prelude::*, BufReader};
use std::{
    collections::{BinaryHeap, HashMap},
    env::temp_dir,
    fs::File,
    path::PathBuf,
};
use structopt::StructOpt;

use anyhow::Result;
use solution::BatchWriter;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,

    /// Number of intermediate fils of MapReduce algorithm.
    #[structopt(long)]
    nmaps: u64,
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    println!("{:#?}", &opt);
    let (file, nmaps) = (opt.file, opt.nmaps);

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    // let mut cnt = HashMap::new();

    let dir = tempfile::TempDir::new().unwrap();
    let intermediate_path = move |i| dir.path().join(format!("mr-{}", i));

    let mut nentries = 0;
    {
        let mut writer = BatchWriter::new(10000000);

        // Map phase.
        for line in reader.lines() {
            let key = line?;
            nentries += 1;

            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let i = hasher.finish() % nmaps;
            let p = intermediate_path(i);

            // let mut f = File::with_options()
            //     .create(true)
            //     .write(true)
            //     .append(true)
            //     .open(p)?;
            // f.write(key.as_bytes())?;
            // f.write("\n".as_bytes())?;
            writer.write(&p, &(key + "\n"));
        }
    }
    println!("number of entries: {}", nentries);

    let mut heap: BinaryHeap<(i32, String)> = BinaryHeap::new();
    let maxk = 10;

    // Reduce phase.
    for i in 0..nmaps {
        let p = intermediate_path(i);
        let mut cnt = HashMap::new();
        if let Ok(file) = File::open(&p) {
            println!("file {:?} of size {:?}", p, file.metadata().unwrap().len());
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let key = line?;
                if let Some(v) = cnt.get_mut(&key) {
                    *v += 1;
                } else {
                    cnt.insert(key, 1);
                }
            }
        }

        for (k, v) in cnt {
            heap.push((-v, k.to_owned()));

            // FIXME: same count?
            if heap.len() > maxk {
                heap.pop();
            }
        }
    }

    println!("{:#?}", heap.into_sorted_vec());
    Ok(())
}
