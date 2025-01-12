use std::collections::BTreeMap;

use dioxus::logger::tracing::*;

use crate::PATH;

use super::{Extension, Manifest};

pub struct ExtensionsState(pub BTreeMap<String, Extension>);

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
}

#[derive(Default)]
pub struct ManifestsState(pub Vec<Manifest>);

#[derive(Default)]
pub struct RepositoryUrlState(pub String);

impl std::fmt::Display for RepositoryUrlState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
