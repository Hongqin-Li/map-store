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
use serde::{Deserialize, Serialize};

use map_store::{BatchWriter, Operator};

use crate::mink_set::MinkSet;
use crate::Solution;

#[derive(Serialize, Deserialize)]
struct Increment1 {}

impl Operator<i32> for Increment1 {
    fn apply(&self, value: &mut i32) {
        *value += 1;
    }
}

/// MapStore method based on MapReduce with `M = nmaps`.
pub struct MapStore {
    /// Number of splits on keys.
    pub nmaps: u32,
}

impl Solution for MapStore {
    fn solve(&self, topk: u32, path: impl AsRef<Path>) -> HashMap<String, u32> {
        let dir = tempfile::TempDir::new().unwrap();

        let mut store = map_store::MapStore::new(self.nmaps as u64, dir.path());
        let inc1 = Increment1 {};

        let mut reader = BufReader::new(File::open(path).expect("failed to open input file"));
        let mut buf = vec![];

        while reader
            .read_until(b'\n', &mut buf)
            .expect("failed to read file")
            > 0
        {
            let last = buf.pop().expect("buffer should not be empty");
            assert_eq!(last, b'\n');
            store.apply(&buf, &inc1);
            buf.clear();
        }

        let mut set = MinkSet::new(topk as usize);
        for (k, v) in store.iter_without_compaction() {
            set.insert((-v, k))
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
mod test {
    use serde_json::Map;
    use tempfile::tempdir;

    use crate::{brute_force::BruteForce, Generator, Solution};

    use super::MapStore;

    #[test]
    fn test_map_store() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tmp");

        let g = Generator::Normal {};
        g.generate(1, &path);

        let solver = BruteForce {};
        let ans1 = solver.solve(10, &path);
        let solver = MapStore { nmaps: 10 };
        let ans2 = solver.solve(10, &path);
        assert_eq!(ans1, ans2);
    }

    #[bench]
    fn bench_map_store(b: &mut test::Bencher) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tmp");

        let g = Generator::Normal {};
        g.generate(10, &path);

        b.iter(|| {
            let solver = MapStore { nmaps: 10 };
            let ans = solver.solve(10, &path);
        });
    }
}
