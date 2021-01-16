use anyhow::Result;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

/// Writing to disk in batch by caching in memory for efficiency.
#[derive(Default)]
pub struct BatchWriter {
    max_cache_len: usize,
    cache_len: usize,
    file: Vec<File>,
    cache: Vec<Vec<u8>>,
}

impl BatchWriter {
    /// Create a new [BatchWriter] with maximum cache size in memory and
    /// paths to write to.
    pub fn new(max_cache_len: usize, path: Vec<PathBuf>) -> Self {
        let cache = vec![vec![]; path.len()];
        let mut file = vec![];
        for p in path {
            file.push(
                File::with_options()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(p)
                    .unwrap(),
            );
        }
        debug_assert!(cache.len() == file.len());
        Self {
            max_cache_len,
            file,
            cache,
            cache_len: 0,
        }
    }

    /// Write content in `buf` to file of path by `path_id`.
    ///
    /// `buf` will be cleared after writing.
    pub fn write(&mut self, path_id: usize, buf: &mut Vec<u8>) {
        self.cache_len += buf.len();

        self.cache[path_id].append(buf);
        if self.cache_len > self.max_cache_len {
            self.flush();
        }
    }

    /// Flush all in-memory cache to disk.
    pub fn flush(&mut self) {
        for (path_id, buf) in self.cache.iter().enumerate() {
            let f = &mut self.file[path_id];
            f.write(buf.as_slice()).unwrap();
        }
        self.cache_len = 0;
        for s in self.cache.iter_mut() {
            s.clear();
        }
        // println!("flush");
    }
}

impl Drop for BatchWriter {
    fn drop(&mut self) {
        self.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    use crate::BatchWriter;

    #[test]
    fn test_flush() {
        let dir = tempfile::tempdir().unwrap();

        let p1 = dir.path().join("1");
        let p2 = dir.path().join("2");
        let paths = vec![p1.clone(), p2.clone()];

        let mut writer = BatchWriter::new(10, paths);
        let content1 = vec![1; 10];
        let content2 = vec![0, 1, 2, 3];

        writer.write(0, &mut (content1.clone()));
        if let Ok(mut f) = File::open(&p1) {
            let mut buf = vec![];
            assert_eq!(f.read(&mut buf).unwrap(), 0);
        }
        writer.write(1, &mut (content2.clone()));

        let mut s1 = vec![];
        let mut s2 = vec![];
        File::open(&p1).unwrap().read_to_end(&mut s1).unwrap();
        File::open(&p2).unwrap().read_to_end(&mut s2).unwrap();

        assert_eq!(s1, content1);
        assert_eq!(s2, content2);
    }

    #[test]
    fn test_drop() {
        let dir = tempfile::tempdir().unwrap();

        let p1 = dir.path().join("1");
        let paths = vec![p1.clone()];
        let content1 = String::from("12345");

        {
            let mut writer = BatchWriter::new(10, paths);

            writer.write(0, &mut (content1.as_bytes().to_vec()));
            if let Ok(mut f) = File::open(&p1) {
                let mut buf = vec![];
                assert_eq!(f.read(&mut buf).unwrap(), 0);
            }
        }

        let mut s1 = String::new();
        File::open(&p1).unwrap().read_to_string(&mut s1).unwrap();

        assert_eq!(s1, content1);
    }
}
