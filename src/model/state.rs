use std::collections::{btree_map, BTreeMap};

use dioxus::logger::tracing::*;
use midoku_path::use_path_resolver;

use crate::hook::UsePersistent;

use super::{Extension, Manifest};

pub struct ExtensionsState(BTreeMap<String, Extension>);

impl ExtensionsState {
    pub fn init() -> Self {
        let path_resolver = use_path_resolver();

        let extensions_dir = path_resolver.extensions_dir();
        std::fs::create_dir_all(extensions_dir.clone()).unwrap();
        let extensions = std::fs::read_dir(extensions_dir)
            .unwrap()
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

pub trait RepositoryUrlState {
    const REPOSITORY_URL_KEY: &str;
    fn get_repository_url(&self) -> String;
    fn set_repository_url(&mut self, value: String);
}

impl RepositoryUrlState for UsePersistent {
    const REPOSITORY_URL_KEY: &str = "repositoryUrl";

    fn get_repository_url(&self) -> String {
        self.get(Self::REPOSITORY_URL_KEY)
            .unwrap_or_else(|| String::new())
    }

    fn set_repository_url(&mut self, value: String) {
        self.set(Self::REPOSITORY_URL_KEY, value);
    }
}
