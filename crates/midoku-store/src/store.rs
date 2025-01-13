use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::inner_store::InnerStore;

static STORE_COLLECTION: LazyLock<Mutex<HashMap<String, Store>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
pub struct Store(Arc<Mutex<InnerStore>>);

impl Store {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self(Arc::new(Mutex::new(InnerStore::new(path))))
    }

    pub fn open<S: ToString>(name: S) -> Self {
        let name = name.to_string();
        STORE_COLLECTION
            .lock()
            .unwrap()
            .entry(name.clone())
            .or_insert_with(move || {
                let file_name = format!("{}.json", name);
                let path = midoku_path::app_local_data_dir().unwrap().join(&file_name);
                Store::new(path)
            })
            .clone()
    }

    pub fn get<K: AsRef<str>, T: DeserializeOwned>(&self, key: K) -> Option<T> {
        self.0.lock().unwrap().get(key)
    }

    pub fn set<K: ToString, T: Serialize>(&mut self, key: K, value: T) {
        self.0.lock().unwrap().set(key, value);
    }
}
