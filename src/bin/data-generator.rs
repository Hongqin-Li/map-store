#![feature(with_options)]

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::io::prelude::*;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// Number of samples to generated.
    #[structopt(short, long)]
    n_samples: i32,
}

fn rand_str() -> String {
    let len = 50;
    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    s
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", &opt);

    let mut f = File::with_options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(opt.output)
        .expect("cannot open file");

    let mut sz = 0;
    for i in 0..opt.n_samples {
        let s = rand_str();
        let s = s.as_bytes();
        sz += s.len() + 1;
        f.write(s).expect("write to file error");
        f.write("\n".as_bytes()).expect("write to file error");
    }

    println!("{:#?} GB", (sz as f64) / (2u64.pow(30) as f64));
}
