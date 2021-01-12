use anyhow::Result;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

pub struct BatchWriter {
    max_cache_len: usize,
    cache_len: usize,
    cache: HashMap<PathBuf, String>,
}

impl BatchWriter {
    pub fn new(max_cache_len: usize) -> Self {
        Self {
            max_cache_len,
            cache_len: 0,
            cache: HashMap::default(),
        }
    }

    pub fn write(&mut self, path: &PathBuf, content: &String) {
        if let Some(s) = self.cache.get_mut(path) {
            *s += content;
        } else {
            self.cache.insert(path.into(), content.into());
        }
        self.cache_len += content.len();
        if self.cache_len > self.max_cache_len {
            self.flush();
        }
    }

    pub fn flush(&mut self) {
        for (p, s) in self.cache.iter() {
            let mut f = File::with_options()
                .create(true)
                .write(true)
                .append(true)
                .open(p)
                .unwrap();
            f.write(s.as_bytes()).unwrap();
        }
        self.cache_len = 0;
        self.cache.clear();
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

        let mut writer = BatchWriter::new(10);
        let content1 = String::from("1234567890");
        let content2 = String::from("abcdefg");

        writer.write(&p1, &content1);
        assert!(File::open(&p1).is_err());
        writer.write(&p2, &content2);

        let mut s1 = String::new();
        let mut s2 = String::new();
        File::open(&p1).unwrap().read_to_string(&mut s1).unwrap();
        File::open(&p2).unwrap().read_to_string(&mut s2).unwrap();

        assert_eq!(s1, content1);
        assert_eq!(s2, content2);
    }

    #[test]
    fn test_drop() {
        let dir = tempfile::tempdir().unwrap();

        let p1 = dir.path().join("1");
        let content1 = String::from("1234567890");

        {
            let mut writer = BatchWriter::new(10);

            writer.write(&p1, &content1);
            assert!(File::open(&p1).is_err());
        }

        let mut s1 = String::new();
        File::open(&p1).unwrap().read_to_string(&mut s1).unwrap();

        assert_eq!(s1, content1);
    }
}
