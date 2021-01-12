use anyhow::Result;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

#[derive(Default)]
pub struct BatchWriter {
    max_cache_len: usize,
    cache_len: usize,
    path: Vec<PathBuf>,
    cache: Vec<Vec<u8>>,
}

impl BatchWriter {
    pub fn new(max_cache_len: usize, path: Vec<PathBuf>) -> Self {
        let cache = vec![vec![]; path.len()];
        debug_assert!(cache.len() == path.len());
        Self {
            max_cache_len,
            path,
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

    pub fn flush(&mut self) {
        for (path_id, buf) in self.cache.iter().enumerate() {
            let mut f = File::with_options()
                .create(true)
                .write(true)
                .append(true)
                .open(&self.path[path_id])
                .unwrap();
            f.write(buf.as_slice()).unwrap();
        }
        self.cache_len = 0;
        for s in self.cache.iter_mut() {
            s.clear();
        }
        println!("flush");
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
        assert!(File::open(&p1).is_err());
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
            assert!(File::open(&p1).is_err());
        }

        let mut s1 = String::new();
        File::open(&p1).unwrap().read_to_string(&mut s1).unwrap();

        assert_eq!(s1, content1);
    }
}
