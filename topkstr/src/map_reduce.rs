use log::{info, warn};

use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    path::PathBuf,
};
use std::{
    io::{prelude::*, BufReader},
    path::Path,
};

use anyhow::Result;

use map_store::BatchWriter;

use crate::mink_set::MinkSet;
use crate::Solution;

/// MapReduce method with `M = nmaps`.
pub struct MapReduce {
    /// Number of splits on keys.
    pub nmaps: u32,
}

impl Solution for MapReduce {
    fn solve(&self, topk: u32, path: impl AsRef<Path>) -> HashMap<String, u32> {
        // let mut cnt = HashMap::new();

        let dir = tempfile::TempDir::new().unwrap();
        let intermediate_path = move |i| dir.path().join(format!("mr-{}", i));
        let mut paths = Vec::new();
        for i in 0..self.nmaps {
            paths.push(intermediate_path(i));
        }

        println!("paths: {:?}", paths);

        let mut nentries = 0;
        {
            let mut writer = BatchWriter::new(1000000, paths.clone());
            let mut reader = BufReader::new(File::open(path).expect("failed to open input file"));
            let mut buf = vec![];

            // Map phase.
            while reader
                .read_until(b'\n', &mut buf)
                .expect("failed to read file")
                > 0
            {
                nentries += 1;

                // let mut hasher = DefaultHasher::new();
                // buf.hash(&mut hasher);
                // let i = hasher.finish() % nmaps;
                let i = seahash::hash(&buf) % (self.nmaps as u64);
                let last = buf.last().expect("buffer should not be empty");
                if last != &b'\n' {
                    buf.push(b'\n');
                    warn!("not end with newline");
                }
                writer.write(i as usize, &mut buf);
                debug_assert_eq!(buf.len(), 0);
            }
        }
        println!("number of entries: {}", nentries);

        let mut set = MinkSet::new(topk as usize);

        // Reduce phase.
        for p in paths.iter() {
            let mut cnt: HashMap<Vec<u8>, i32> = HashMap::new();
            if let Ok(file) = File::open(p) {
                println!("file {:?} of size {:?}", p, file.metadata().unwrap().len());
                let mut reader = BufReader::with_capacity(1000000, file);

                let mut buf = vec![];
                while reader
                    .read_until(b'\n', &mut buf)
                    .expect("failed to read from file")
                    > 0
                {
                    buf.pop();
                    if let Some(v) = cnt.get_mut(&buf) {
                        *v += 1;
                    } else {
                        cnt.insert(buf.clone(), 1);
                    }
                    buf.clear();
                }
            }

            for (k, v) in cnt {
                set.insert((-v, k));
            }
        }

        let mut result = HashMap::new();
        for (neg_cnt, buf) in set.into_sorted_vec() {
            result.insert(
                String::from_utf8(buf).expect("failed to convert to string"),
                (-neg_cnt) as u32,
            );
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use tempfile::tempdir;

    use crate::{brute_force::BruteForce, Generator, Solution};

    use super::MapReduce;

    #[test]
    fn test_map_reduce() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tmp");

        let g = Generator::Normal {};
        g.generate(1, &path);

        let solver = BruteForce {};
        let ans1 = solver.solve(10, &path);
        let solver = MapReduce { nmaps: 10 };
        let ans2 = solver.solve(10, &path);
        assert_eq!(ans1, ans2);
    }

    #[bench]
    fn bench_map_reduce(b: &mut test::Bencher) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tmp");

        let g = Generator::Normal {};
        g.generate(10, &path);

        b.iter(|| {
            let solver = MapReduce { nmaps: 10 };
            let ans = solver.solve(10, &path);
        });
    }
}
