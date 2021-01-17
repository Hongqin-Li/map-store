use std::{collections::hash_map::Iter, fmt::Debug, io::prelude::*, iter, path::Path};
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
use log::{debug, trace};
use serde::{de::DeserializeOwned, Serialize};
use tempfile::{tempdir, tempfile};

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
    V: Serialize + DeserializeOwned + Default + Debug,
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
    /// assert_eq!(store.get("key").unwrap(), set1.value);
    /// ```
    ///
    pub fn apply(&mut self, key: impl AsRef<[u8]> + Debug, op: &O) {
        let i = self.hash1(&key);

        trace!("apply on hash {}", i);

        let value = bincode::serialize(op).expect("failed to serialize operation");
        let mut data = KvIter::entry(key, value);

        self.writer.write(i, &mut data);
    }

    fn hash1(&self, key: impl AsRef<[u8]>) -> usize {
        (seahash::hash(key.as_ref()) % self.nmaps) as usize
    }
    fn temp_path(&self) -> PathBuf {
        self.dir.join("tmp")
    }

    /// Get the key-value map in one map region without any doing any compactions.
    ///
    /// Return a pair of map and updated, indicating if any operations are applied.
    /// This function will first flush any cached operations on this region to disk.
    /// Then read the persisted key-value map to memory from kv file.
    /// Finally, loop through all operations on this region and apply them to associated key-value pair.
    ///
    /// Without compaction means that we won't remove the operation log file and won't modify kv file
    /// after operating on the in-memory key-value map.
    pub fn map1_without_compact(&mut self, idx: usize) -> (HashMap<Vec<u8>, V>, bool) {
        let mut map: HashMap<Vec<u8>, V> = HashMap::default();

        // Read key-value storage.
        let kv_path = self.kv_path.get(idx).expect("id out of bound");
        if let Ok(it) = KvIter::new(kv_path) {
            for (k, v) in it {
                map.insert(k, bincode::deserialize(&v).unwrap());
            }
        }

        // Flush the cached operations to disk first.
        self.writer.flush1(idx);

        // Patch operations.
        let map_path = self.map_path.get(idx).expect("id out of bound");
        debug!("cached op file content: {:?}", fs::read(&map_path));
        if let Ok(it) = KvIter::new(map_path) {
            for (k, v) in it {
                let op: O = bincode::deserialize(v.as_ref()).unwrap();
                trace!("op to patch on key {:?}: {:?}", String::from_utf8(k.clone()), v);
                if let Some(value) = map.get_mut(&k) {
                    op.apply(value);
                } else {
                    let mut x = V::default();
                    op.apply(&mut x);
                    map.insert(k, x);
                }
            }
            trace!("patched map: {:?}", map);
            (map, true)
        } else {
            trace!("unchanged map: {:?}", map);
            (map, false)
        }
    }

    /// Get the key-value Mapping in one map region without any doing any compactions.
    ///
    /// This function will first flush any cached operations on this region to disk.
    /// Then read the persisted key-value map to memory from kv file.
    /// Finally, loop through all operations on this region and apply them to associated key-value pair.
    ///
    /// Compared to [map1_without_compaction], it will remove the operation log file and
    /// updating the kv file atomically.
    pub fn map1(&mut self, idx: usize) -> HashMap<Vec<u8>, V> {
        let (map, changed) = self.map1_without_compact(idx);

        // FIXME: This is slow when called frequently.
        if changed {
            let path = self.kv_path.get(idx).unwrap();
            let tmp_path = self.temp_path();

            trace!("patch kv old: {:?}, tmp: {:?}", path, tmp_path);

            let mut data = vec![];
            for (k, v) in map.iter() {
                data.append(&mut KvIter::entry(
                    k,
                    bincode::serialize(v).expect("failed to serialize value"),
                ));
            }
            fs::write(&tmp_path, data).unwrap();
            fs::rename(tmp_path, path).expect("failed to replace with updated kv file");
        }
        map
    }

    /// Iterate over key-value pair in one map region.
    pub fn iter1(&mut self, idx: usize) -> impl Iterator<Item = (Vec<u8>, V)> {
        self.map1(idx).into_iter()
    }

    /// Iterator over all key-value pairs.
    pub fn iter<'a>(&'a mut self) -> impl Iterator<Item = (Vec<u8>, V)> + 'a {
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
    
    /// Iterate over key-value pair in one map region.
    fn iter1_without_compaction(&mut self, idx: usize) -> impl Iterator<Item = (Vec<u8>, V)> {
        let (map, changed) = self.map1_without_compact(idx);
        map.into_iter()
    }

    /// Iterator over all key-value pairs without compaction on operation logs.
    pub fn iter_without_compaction<'a>(&'a mut self) -> impl Iterator<Item = (Vec<u8>, V)> + 'a {
        let mut idx = 0;
        let max_idx = self.map_path.len();
        let mut it = self.iter1_without_compaction(0);
        iter::from_fn(move || loop {
            let result = it.next();
            if result.is_none() {
                idx += 1;
                if idx >= max_idx {
                    break None;
                }
                it = self.iter1_without_compaction(idx);
            } else {
                break result;
            }
        })
    }

    /// Retrieve the value associated with specified key.
    pub fn get(&mut self, key: impl AsRef<[u8]>) -> Option<V> {
        let i = self.hash1(&key);
        let mut map = self.map1(i);
        map.remove(key.as_ref()) // Modification on `map` won't affect on-disk data.
    }
}

/// Iterate on a file consisting of `[key_size, value_size, key, value]` entries.
struct KvIter {
    file_sz: i64,
    reader: BufReader<File>,
}

impl KvIter {
    fn new(path: impl AsRef<Path>) -> Result<Self> {
        let f = File::open(path)?;
        let file_sz = f.metadata().unwrap().len() as i64;
        let reader = BufReader::with_capacity(1000000, f);
        Ok(Self { file_sz, reader })
    }
    fn entry(key: impl AsRef<[u8]> + Debug, value: impl AsRef<[u8]> + Debug) -> Vec<u8> {
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
        if self.file_sz == 0 {
            return None;
        }
        const z: usize = mem::size_of::<u64>();

        debug_assert!(z == 8);
        debug_assert!(self.file_sz > 0);

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

        self.file_sz -= ((z as u64) * 2 + key_sz + value_sz) as i64;   
        Some((key, value))
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_kv_iter() {
        let dir = TempDir::new().unwrap();
        let p = dir.path().join("tmp");

        let kvs = vec![(b"key1", b"value1"), (b"key2", b"value2")];

        let mut data = vec![];
        for (key, value) in kvs.iter() {
            data.append(&mut KvIter::entry(key, value));
        }
        fs::write(&p, data).unwrap();

        let it = KvIter::new(p).expect("failed to create iter");

        let mut nkvs = kvs.len();
        for ((k1, v1), (k2, v2)) in it.zip(kvs) {
            assert_eq!(&k1, k2);
            assert_eq!(&v1, v2);
            nkvs -= 1;
        }
        assert_eq!(nkvs, 0);
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

    #[test]
    fn test_apply() {
        // pretty_env_logger::try_init();
        let dir = TempDir::new().unwrap();
        let mut store = MapStore::new(5, dir.path());

        #[derive(Serialize, Deserialize)]
        enum Op {
            Add(i32),
        }

        impl Operator<i32> for Op {
            fn apply(&self, value: &mut i32) {
                match self {
                    Op::Add(x) => {
                        *value += x;
                    }
                }
            }
        }

        let mut rng = rand::thread_rng();

        let mut ans = HashMap::new();
        
        for i in 0..10 {
            let key = vec![rng.gen()];
            let dx = rng.gen::<i32>() % 100;
            if let Some(x) = ans.get_mut(&key) {
                *x += dx;
            } else {
                ans.insert(key.clone(), dx);
            }
            store.apply(&key, &Op::Add(dx))
        }

        for (k, v) in store.iter() {
            assert_eq!(&v, ans.get(&k).unwrap());
        }
    }
}
