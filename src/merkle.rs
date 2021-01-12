use std::path::PathBuf;

#[derive(Clone)]
pub struct MerkleStore {
    path: PathBuf,
}

impl MerkleStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path: PathBuf = path.into();
        Self { path }
    }

    pub fn remove(&self, key: impl AsRef<str>) -> Option<String> {
        todo!();
    }

    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) {
        todo!();
    }
}
