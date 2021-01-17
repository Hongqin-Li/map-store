use std::path::PathBuf;

use structopt::{clap::arg_enum, StructOpt};

use topkstr::{Generator, MapReduce, Solution};

arg_enum! {
    #[derive(Debug)]
    enum Mode {
        Distinct,
        Identical,
        Noraml,
    }
}
/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Output file.
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// Size of samples to generate in GB.
    #[structopt(short, long)]
    size: u64,
    /// Mode of sample distribution.
    #[structopt(possible_values = &Mode::variants(), case_insensitive = true)]
    mode: Mode,
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    println!("{:#?}", &opt);

    let g = match opt.mode {
        Mode::Noraml => Generator::Normal {},
        Mode::Distinct => Generator::Distinct {},
        Mode::Identical => Generator::Identical {},
    };
    g.generate(opt.size * 1024, opt.output);
}
