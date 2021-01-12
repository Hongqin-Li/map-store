use std::{
    collections::{HashMap, LinkedList},
    path::PathBuf,
    ptr,
    rc::Rc,
    sync::Arc,
};

use crate::merkle::MerkleStore;

type Value = i64;

#[derive(Clone)]
struct Item {
    key: String,
    value: Value,
    prev: *mut Item,
    next: *mut Item,
}

impl Item {
    fn from_kv(key: String, value: Value) -> Box<Self> {
        let mut x = Box::new(Item {
            key,
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        });
        x.prev = x.as_mut() as *mut Item;
        x.next = x.as_mut() as *mut Item;
        x
    }
    fn unlink(&self) {
        unsafe {
            (*self.prev).next = self.next;
            (*self.next).prev = self.prev;
        }
    }
    fn link_back(&mut self, ptr: *mut Item) {
        unsafe {
            (*ptr).prev = self.prev;
            (*self.prev).next = ptr;
            (*ptr).next = self as *mut Item;
            self.prev = ptr;
        }
    }
    fn unlink_front(&mut self) {
        debug_assert!(!self.empty());
        unsafe {
            self.next = (*self.next).next;
            (*self.next).prev = self as *mut Item;
        }
    }
    fn front(&self) -> *mut Self {
        self.next
    }
    fn empty(&self) -> bool {
        if self.prev == self.next {
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct HashKv {
    max_items: u32,
    nitems: u32,

    map: HashMap<String, Box<Item>>,
    lru: Box<Item>,
    store: MerkleStore,
}

impl HashKv {
    pub fn new(path: impl Into<PathBuf>, max_items: u32) -> Self {
        Self {
            max_items,
            map: HashMap::default(),
            lru: Item::from_kv("".to_string(), 0),
            nitems: 0,
            store: MerkleStore::new(path),
        }
    }
    /// Fetch item by key
    ///
    /// Fetch from disk if not exist in the in-memory map.
    fn fetch(&mut self, key: String) -> *mut Item {
        if let Some(v) = self.map.get_mut(&key) {
            (*v).unlink();
            self.lru.link_back(v.as_mut());
            return v.as_mut() as *mut Item;
        } else {
            let v = if let Some(v) = self.store.remove(&key) {
                v.parse().expect("parsing string to int error")
            } else {
                Value::default()
            };

            let mut i = Item::from_kv(key.clone(), v);
            let ptr = i.as_mut() as *mut Item;
            self.map.insert(key, i);

            self.lru.link_back(ptr);
            self.nitems += 1;

            // If too many items exist in memory, flush them to disk.
            if self.nitems > self.max_items {
                let ptr = self.lru.front();
                self.lru.unlink_front();
                let (key, value) = unsafe { ((*ptr).key.clone(), (*ptr).value.clone()) };
                self.map.remove(&key);
                self.store.set(key, value.to_string());
            }

            return ptr;
        }
    }

    /// Increment value by key.
    pub fn increment(&mut self, key: String) {
        let ptr = self.fetch(key);
        unsafe {
            (*ptr).value += 1;
        }
    }

    pub fn for_each(&self, mut f: impl FnMut(&String, &Value)) {
        for (k, v) in self.map.iter() {
            f(k, &v.value);
        }
        // self.store.for_each(f);
    }
}
