use std::{collections::hash_map::Iter, io::prelude::*, iter, path::Path};
use std::{
    collections::HashMap,
    fs::{self, File},
    hash::Hash,
    io::BufReader,
    marker::PhantomData,
    mem,
    path::PathBuf,
};

use anyhow::Result;
use bincode::Serializer;
use serde::{de::DeserializeOwned, Serialize};

use crate::{batch_writer, operator::Operator, BatchWriter};

/// MapReduce-based key-value storage.
pub struct MapStore<V, O> {
    nmaps: u64,
    dir: PathBuf,
    map_path: Vec<PathBuf>,
    kv_path: Vec<PathBuf>,
    writer: BatchWriter,
    _v: PhantomData<V>,
    _o: PhantomData<O>,
}

impl<V, O> MapStore<V, O>
where
    V: Serialize + DeserializeOwned + Default,
    O: Serialize + DeserializeOwned + Operator<V>,
{
    // TODO: options: in-memory-cache/persistent, dynamically change nmaps.

    /// Create a new MapStore with `nmaps` maps in directory `dir`.
    ///
    /// The key-value pairs will be stored in this maps by hashing on key.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::TempDir;
    /// use map_store::{MapStore, Operator};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct SetOp {
    ///     pub value: u32,
    /// }
    ///
    /// impl Operator<u32> for SetOp {
    ///     fn apply(&self, value: &mut u32) {
    ///         *value = self.value;
    ///     }
    /// }
    ///
    /// let dir = TempDir::new().unwrap();
    ///
    /// let store = MapStore::<u32, SetOp>::new(5, dir.path());
    /// ```
    pub fn new(nmaps: u64, dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        let mut map_path = vec![];
        let mut kv_path = vec![];
        for i in 0..nmaps {
            map_path.push(dir.join(format!("map-{}", i)));
            kv_path.push(dir.join(format!("kv-{}", i)));
        }
        let writer = BatchWriter::new(1000000, map_path.clone());
        Self {
            map_path,
            kv_path,
            nmaps,
            dir,
            writer,
            _v: PhantomData,
            _o: PhantomData,
        }
    }

    /// Apply operation `op` on `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::TempDir;
    /// use map_store::{MapStore, Operator};
    /// use serde::{Deserialize, Serialize};
    ///
    /// let dir = TempDir::new().unwrap();
    /// let mut store = MapStore::new(5, dir.path());
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct SetOp {
    ///     pub value: u32,
    /// }
    ///
    /// impl Operator<u32> for SetOp {
    ///     fn apply(&self, value: &mut u32) {
    ///         *value = self.value;
    ///     }
    /// }
    /// let set1 = SetOp { value: 1 };
    /// store.apply("key", &set1);
    /// ```
    ///
    pub fn apply(&mut self, key: impl AsRef<[u8]>, op: &O) {
        let i = seahash::hash(key.as_ref()) % self.nmaps;

        let value = bincode::serialize(op).expect("failed to serialize operation");
        let mut data = KvIter::entry(key, value);

        self.writer.write(i as usize, &mut data);
    }

    /// Iterate over key-value pair in one map region.
    pub fn iter1(&self, idx: usize) -> impl Iterator<Item = (Vec<u8>, V)> {
        let idx = 0;
        let mut map: HashMap<Vec<u8>, V> = HashMap::default();

        // Read key-value storage.
        let kv_path = self.kv_path.get(idx).expect("id out of bound");
        if let Ok(it) = KvIter::new(kv_path) {
            for (k, v) in it {
                map.insert(
                    bincode::deserialize(&k).unwrap(),
                    bincode::deserialize(&v).unwrap(),
                );
            }
        }

        // Patch operations.
        let map_path = self.map_path.get(idx).expect("id out of bound");
        if let Ok(it) = KvIter::new(map_path) {
            for (k, v) in it {
                let op: O = bincode::deserialize(v.as_ref()).unwrap();
                if let Some(value) = map.get_mut(&k) {
                    op.apply(value);
                } else {
                    let mut x = V::default();
                    op.apply(&mut x);
                    map.insert(k, x);
                }
            }
        }
        map.into_iter()
    }

    /// Iterator over all key-value pairs.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Vec<u8>, V)> + 'a {
        let mut idx = 0;
        let max_idx = self.map_path.len();
        let mut it = self.iter1(0);
        iter::from_fn(move || loop {
            let result = it.next();
            if result.is_none() {
                idx += 1;
                if idx >= max_idx {
                    break None;
                }
                it = self.iter1(idx);
            } else {
                break result;
            }
        })
    }

    /// Retrieve the value associated with specified key.
    pub fn get(key: &[u8]) -> Result<Option<V>> {
        todo!();
    }
}

/// Iterate on a file consisting of `[key_size, value_size, key, value]` entries.
struct KvIter {
    file_sz: u64,
    reader: BufReader<File>,
}

impl KvIter {
    fn new(path: impl AsRef<Path>) -> Result<Self> {
        let f = File::open(path)?;
        let file_sz = f.metadata().unwrap().len();
        let reader = BufReader::with_capacity(1000000, f);
        Ok(Self { file_sz, reader })
    }
    fn entry(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Vec<u8> {
        let key_sz = key.as_ref().len() as u64;
        let value_sz = value.as_ref().len() as u64;

        let key_sz = key_sz.as_ne_bytes();
        let value_sz = value_sz.as_ne_bytes();

        [key_sz, value_sz, key.as_ref(), value.as_ref()].concat()
    }
}

impl Iterator for KvIter {
    type Item = (Vec<u8>, Vec<u8>);
    fn next(&mut self) -> Option<Self::Item> {
        let mut sz = self.file_sz;
        const z: usize = mem::size_of::<u64>();
        debug_assert!(z == 8);

        let mut buf = [0; z];
        self.reader
            .read_exact(&mut buf)
            .expect("failed to read key size");

        let key_sz = u64::from_ne_bytes(buf);

        let mut buf = [0; z];
        self.reader
            .read_exact(&mut buf)
            .expect("failed to read value size");
        let value_sz = u64::from_ne_bytes(buf);

        let mut buf = vec![0; key_sz as usize];
        self.reader
            .read_exact(&mut buf)
            .expect("failed to read key");
        let key = buf;

        let mut buf = vec![0; value_sz as usize];
        self.reader
            .read_exact(&mut buf)
            .expect("failed to read value");
        let value = buf;

        sz -= (z as u64) * 2 + key_sz + value_sz;
        if sz == 0 {
            None
        } else {
            Some((key, value))
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_kv_iter() {
        let dir = TempDir::new().unwrap();
        let p = dir.path().join("tmp");

        let kvs = vec![(b"key1", b"value1"), (b"key2", b"value2")];

        for (key, value) in kvs.iter() {
            fs::write(&p, KvIter::entry(key, value)).expect("failed to write file");
        }

        let it = KvIter::new(p).expect("failed to create iter");

        for ((k1, v1), (k2, v2)) in it.zip(kvs) {
            assert_eq!(&k1, k2);
            assert_eq!(&v1, v2);
        }
    }

    #[test]
    #[should_panic(expected = "failed to read key size")]
    fn test_kv_iter_failed() {
        let dir = TempDir::new().unwrap();
        let p = dir.path().join("tmp");

        let kvs = vec![(b"key1", b"value1"), (b"key2", b"value2")];

        for (key, value) in kvs.iter() {
            fs::write(&p, KvIter::entry(key, value)).unwrap();
        }
        fs::write(&p, b"xxx").unwrap();

        let it = KvIter::new(p).unwrap();

        for ((k1, v1), (k2, v2)) in it.zip(kvs) {
            assert_eq!(&k1, k2);
            assert_eq!(&v1, v2);
        }
    }
}
