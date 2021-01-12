#![feature(with_options)]
#![feature(binary_heap_into_iter_sorted)]

use log::{info, warn};
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

    // let mut cnt = HashMap::new();

    let dir = tempfile::TempDir::new().unwrap();
    let intermediate_path = move |i| dir.path().join(format!("mr-{}", i));
    let mut paths = Vec::new();
    for i in 0..nmaps {
        paths.push(intermediate_path(i));
    }

    println!("paths: {:?}", paths);

    let mut nentries = 0;
    {
        let mut writer = BatchWriter::new(10000000, paths.clone());
        let mut reader = BufReader::new(File::open(file)?);
        let mut buf = vec![];

        // Map phase.
        while reader.read_until(b'\n', &mut buf)? > 0 {
            nentries += 1;

            let mut hasher = DefaultHasher::new();
            buf.hash(&mut hasher);
            let i = hasher.finish() % nmaps;
            let last = buf.last().expect("buffer should not be empty");
            if last != &b'\n' {
                buf.push(b'\n');
            } else {
                warn!("not end with newline");
            }
            writer.write(i as usize, &mut buf);
            assert_eq!(buf.len(), 0);
        }
    }
    println!("number of entries: {}", nentries);

    let mut heap: BinaryHeap<(i32, Vec<u8>)> = BinaryHeap::new();
    let maxk = 10;

    // Reduce phase.
    for p in paths {
        let mut cnt = HashMap::new();
        if let Ok(file) = File::open(&p) {
            println!("file {:?} of size {:?}", p, file.metadata().unwrap().len());
            let mut reader = BufReader::new(file);
            let mut buf = vec![];
            while reader.read_until(b'\n', &mut buf)? > 0 {
                if let Some(v) = cnt.get_mut(&buf) {
                    *v += 1;
                } else {
                    cnt.insert(buf.clone(), 1);
                }
                buf.clear();
            }
        }

        for (k, v) in cnt {
            heap.push((-v, k));

            // FIXME: same count?
            if heap.len() > maxk {
                heap.pop();
            }
        }
    }


    let mut result = Vec::new();
    for (neg_cnt, buf) in heap.into_iter_sorted() {
        result.push((String::from_utf8(buf)?, -neg_cnt));
    }
    println!("{:#?}", result);
    Ok(())
}
