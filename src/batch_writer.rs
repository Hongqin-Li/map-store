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
        println!("flush")
    }
}

impl Drop for BatchWriter {
    fn drop(&mut self) {
        self.flush();
    }
}
