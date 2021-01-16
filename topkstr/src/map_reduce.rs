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

use map_store::{BatchWriter};

use crate::Solution;
use crate::mink_set::MinkSet;

struct MapReduce {
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
                assert_eq!(buf.len(), 0);
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
mod test {


    #[test]
    fn test_map_reduce() {


    }
}
