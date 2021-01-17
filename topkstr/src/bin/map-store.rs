use std::path::PathBuf;

use structopt::StructOpt;

use topkstr::{MapReduce, MapStore, Solution};

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,

    /// Number of intermediate fils of MapReduce algorithm.
    #[structopt(long)]
    nmaps: u32,

    /// Output k most common string.
    #[structopt(short = "k", long)]
    topk: u32,
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    println!("{:#?}", &opt);

    // let mut cnt = HashMap::new();
    let s = MapStore { nmaps: opt.nmaps };
    let res = s.solve(opt.topk, opt.file);
    println!("{:#?}", res);
}
