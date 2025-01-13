mod store;

use std::collections::HashMap;

use dioxus::prelude::*;
use midoku_path::use_path_resolver;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::store::Store;

type SharedStore = Signal<Store>;
pub type StoreCollection = HashMap<String, SharedStore>;

pub fn use_store<S: ToString>(name: S) -> UseStore {
    let path_resolver = use_path_resolver();
    let mut store_collection = use_context::<Signal<StoreCollection>>();

    let name = name.to_string();
    let path = path_resolver
        .app_local_data_dir()
        .join(&format!("{}.json", name));

    let mut stores = store_collection.write();
    let store = stores
        .entry(name)
        .or_insert_with(|| use_signal(|| Store::new(path)))
        .clone();

    UseStore { inner: store }
}

#[derive(Clone, Copy)]
pub struct UseStore {
    inner: SharedStore,
}

impl UseStore {
    pub fn get<K: AsRef<str>, T: DeserializeOwned>(&self, key: K) -> Option<T> {
        self.inner.read().get(key)
    }

    pub fn set<K: ToString, T: Serialize>(&mut self, key: K, value: T) {
        self.inner.write().set(key, value);
    }
}
