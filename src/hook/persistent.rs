use std::fs::OpenOptions;
use std::io::Write;

use dioxus::prelude::*;
use midoku_path::use_path_resolver;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

fn get<K: AsRef<str>, T: DeserializeOwned>(key: K) -> Option<T> {
    let path_resolver = use_path_resolver();

    let path = path_resolver.app_local_data_dir();
    let mut file_path = path.join(key.as_ref());
    file_path.set_extension("json");
    _ = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path.clone());

    let raw = std::fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&raw).ok()
}

fn set<K: AsRef<str>, T: Serialize>(key: K, value: &T) {
    let path_resolver = use_path_resolver();

    let path = path_resolver.app_local_data_dir();
    std::fs::create_dir_all(&path).unwrap();
    let mut file_path = path.join(key.as_ref());
    file_path.set_extension("json");

    let mut file = std::fs::File::create(file_path).unwrap();
    let content = serde_json::to_vec_pretty(value).unwrap();
    file.write_all(&content).unwrap()
}

pub fn use_persistent(key: impl ToString) -> UsePersistent {
    let state = use_signal(move || {
        let key = key.to_string();
        let value = get(&key).unwrap_or_else(|| {
            let value = Map::new();
            set(&key, &value);
            value
        });
        StorageEntry { key, value }
    });

    UsePersistent { inner: state }
}

#[derive(Clone, Copy)]
pub struct UsePersistent {
    inner: Signal<StorageEntry>,
}

impl UsePersistent {
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.inner
            .read()
            .value
            .get(key)
            .cloned()
            .map(|value| serde_json::from_value(value).unwrap())
    }

    pub fn set<K: ToString, T: Serialize>(&mut self, key: K, value: T) {
        let value = serde_json::to_value(value).unwrap();
        let mut inner = self.inner.write();
        inner.value.insert(key.to_string(), value);
        set(&inner.key, &inner.value);
    }
}

struct StorageEntry {
    key: String,
    value: Map<String, Value>,
}
