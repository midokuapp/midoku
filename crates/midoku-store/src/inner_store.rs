use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};

pub struct InnerStore {
    path: PathBuf,
    cache: Map<String, Value>,
}

impl InnerStore {
    pub(crate) fn new(path: PathBuf) -> Self {
        let mut store = Self {
            path,
            cache: Map::new(),
        };
        store.read();
        store
    }

    fn read(&mut self) {
        _ = OpenOptions::new().create(true).write(true).open(&self.path);

        let raw = read_to_string(&self.path).unwrap();
        self.cache = serde_json::from_str(&raw).unwrap();
    }

    fn write(&self) {
        let mut file = File::create(&self.path).unwrap();
        let content = serde_json::to_vec_pretty(&self.cache).unwrap();
        file.write_all(&content).unwrap();
    }

    pub(crate) fn get<K: AsRef<str>, T: DeserializeOwned>(&self, key: K) -> Option<T> {
        let key = key.as_ref();
        self.cache
            .get(key)
            .cloned()
            .map(|value| serde_json::from_value(value).unwrap())
    }

    pub(crate) fn set<K: ToString, T: Serialize>(&mut self, key: K, value: T) {
        let key = key.to_string();
        let value = serde_json::to_value(value).unwrap();
        self.cache.insert(key, value);
        self.write();
    }
}
