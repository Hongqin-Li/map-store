use std::path::PathBuf;
use structopt::StructOpt;
use topkstr::{BruteForce, Solution};

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,

    /// Output k most common string.
    #[structopt(short = "k", long)]
    topk: u32,
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    println!("{:#?}", &opt);

    // let mut cnt = HashMap::new();
    let s = BruteForce {};
    let res = s.solve(opt.topk, opt.file);

    println!("{:#?}", res);
}
