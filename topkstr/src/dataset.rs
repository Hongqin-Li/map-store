use rand::distributions::Alphanumeric;
use rand::distributions::Distribution;
use rand::{thread_rng, Rng, RngCore};

use map_store::BatchWriter;

use statrs::distribution::Normal;
use statrs::distribution::{Continuous, Pareto};
use statrs::prec;
use statrs::statistics::Mean;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

fn int2u8(x: u64, len: u64) -> Vec<u8> {
    let mut vec = x.to_string().as_bytes().to_vec();
    while vec.len() < len as usize {
        vec.push(b'$');
    }
    vec
}

fn rand_u8() -> Vec<u8> {
    let len = 50;
    let s: Vec<u8> = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(|c| c as u8)
        .collect();
    s
}

pub enum Generator {
    Distinct,
    Normal,
    Identical,
}

impl Generator {
    /// Generate dataset of `size` GB.
    pub fn generate(&self, size: u64, path: impl AsRef<Path>) {
        let mut f = File::with_options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path.as_ref())
            .expect("cannot open file");

        let len = 50;
        let mut sz = 0;
        let gb = 2u64.pow(30);
        {
            let pareto = Pareto::new((size * gb) as f64 / 1000.0, 2.0).unwrap();
            let mut rng = thread_rng();

            let paths = vec![PathBuf::from(path.as_ref())];
            let mut writer = BatchWriter::new(10000000, paths);
            while sz / gb < size {
                let mut s = match self {
                    Self::Distinct => rand_u8(),
                    Self::Normal => {
                        let x = pareto.sample(&mut rng) as u64;
                        int2u8(x, len)
                    }
                    Self::Identical => vec![b'x'; len as usize],
                };
                s.push(b'\n');
                sz += s.len() as u64;
                writer.write(0, &mut s);
            }
        }

        println!("{:#?} GB", (sz as f64) / (gb as f64));
    }
}
