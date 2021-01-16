use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

use std::{collections::BinaryHeap, fs::File};

use super::Solution;

pub struct BruteForce {}

impl Solution for BruteForce {
    fn solve(&self, topk: u32, path: impl AsRef<Path>) -> HashMap<String, u32> {
        let file = File::open(path).expect("failed to open input file");
        let reader = BufReader::new(file);

        let mut cnt = HashMap::new();

        for line in reader.lines() {
            let key = line.expect("failed to read line");
            if let Some(c) = cnt.get_mut(&key) {
                *c += 1;
            } else {
                cnt.insert(key, 1);
            }
        }

        let mut heap: BinaryHeap<(i32, String)> = BinaryHeap::new();
        for (k, cnt) in cnt.iter() {
            heap.push((-cnt, k.to_owned()));
            if heap.len() > (topk as usize) {
                heap.pop();
            }
        }
        let mut result = HashMap::new();
        for (cnt, s) in heap {
            result.insert(s, (-cnt) as u32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_brute_force() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tmp");
        let strs = vec!["3", "3", "3", "2", "2", "2x", "2x", "a", "b", "c"];

        fs::write(&path, strs.join("\n")).expect("failed to write to file");

        let solver = BruteForce {};
        let x = solver.solve(3, &path);
        assert_eq!(x.len(), 3);
        assert_eq!(x.get("3").unwrap(), &3);
        assert_eq!(x.get("2").unwrap(), &2);
        assert_eq!(x.get("2x").unwrap(), &2);
    }
}
