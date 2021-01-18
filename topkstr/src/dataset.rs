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

/// Dataset generator.
pub enum Generator {
    /// All strings are different.
    Distinct,
    /// Distribution of strings follows [pareto distribution](https://en.wikipedia.org/wiki/Pareto_distribution)
    Normal,
    /// All strings are same.
    Identical,
}

impl Generator {
    /// Generate dataset of `size` MB.
    pub fn generate(&self, size: u64, path: impl AsRef<Path>) {
        let mut f = File::with_options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path.as_ref())
            .expect("cannot open file");

        let len = 50;
        let mut sz = 0;
        let mb = 2u64.pow(20);
        {
            let pareto = Pareto::new((size * mb) as f64, 3.0).unwrap();
            let mut rng = thread_rng();

            let paths = vec![PathBuf::from(path.as_ref())];
            let mut writer = BatchWriter::new(10000000, paths);
            while sz / mb < size {
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

        println!("{:#?} GB", (sz as f64) / (mb as f64) / 1024.);
    }
}

#[cfg(test)]
mod tests {
    use statrs::assert_almost_eq;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_generated_size() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tmp");

        let g = Generator::Normal {};
        g.generate(3, &path);
        assert!(path.exists());
        assert_almost_eq!(
            path.metadata().unwrap().len() as f64 / 1024. / 1024.,
            3.,
            1.
        );
    }
}
