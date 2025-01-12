use std::collections::{btree_map, BTreeMap};

use dioxus::logger::tracing::*;

use crate::PATH;

use super::{Extension, Manifest};

pub struct ExtensionsState(BTreeMap<String, Extension>);

impl ExtensionsState {
    pub fn init() -> Self {
        let extensions_dir = PATH.extensions_dir().expect("failed to get extensions dir");
        let extensions = std::fs::read_dir(extensions_dir)
            .expect("failed to read extensions dir")
            .flat_map(|entry| {
                let entry = entry.expect("failed to read entry");
                let extension = Extension::from_path(entry.path());

                match extension {
                    Ok(extension) => {
                        debug!("loaded extension: {}", &extension.id);
                        Some((extension.id.clone(), extension))
                    }
                    Err(e) => {
                        warn!("failed to load extension: {}", e);
                        None
                    }
                }
            })
            .collect();

        Self(extensions)
    }

    pub fn insert(&mut self, extension: Extension) {
        self.0.insert(extension.id.clone(), extension);
    }

    pub fn remove(&mut self, extension_id: &str) {
        self.0.remove(extension_id);
    }

    pub fn contains(&self, extension_id: &str) -> bool {
        self.0.contains_key(extension_id)
    }

    pub fn iter(&self) -> btree_map::Iter<'_, String, Extension> {
        self.0.iter()
    }
}

#[derive(Default)]
pub struct ManifestsState(Vec<Manifest>);

impl From<Vec<Manifest>> for ManifestsState {
    fn from(value: Vec<Manifest>) -> Self {
        Self(value)
    }
}

impl ManifestsState {
    pub fn iter(&self) -> std::slice::Iter<'_, Manifest> {
        self.0.iter()
    }
}

#[derive(Default)]
pub struct RepositoryUrlState(String);

impl std::fmt::Display for RepositoryUrlState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RepositoryUrlState {
    fn from(value: String) -> Self {
        Self(value)
    }
}
