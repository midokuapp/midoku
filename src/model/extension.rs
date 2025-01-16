use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use dioxus::logger::tracing::*;
use dioxus::prelude::*;
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use midoku_bindings::Bindings;

use crate::error::{Error, Result};

use super::Source;

#[derive(Clone, Copy)]
pub struct Extensions {
    pub inner: Signal<BTreeMap<String, Arc<Extension>>>,
}

impl Extensions {
    pub async fn init() -> Self {
        let extensions_dir = crate::util::extensions_dir().unwrap();
        std::fs::create_dir_all(extensions_dir.clone()).unwrap();

        let mut extensions = BTreeMap::new();
        for entry in std::fs::read_dir(extensions_dir).unwrap() {
            let entry = entry.expect("failed to read entry");
            let extension = Extension::from_path(entry.path()).await;

            match extension {
                Ok(extension) => {
                    debug!("loaded extension: {}", &extension.id);
                    extensions.insert(extension.id().to_string(), Arc::new(extension));
                }
                Err(e) => warn!("failed to load extension: {}", e),
            }
        }

        Extensions {
            inner: Signal::new(extensions),
        }
    }

    pub fn contains<S: AsRef<str>>(&self, extension_id: S) -> bool {
        self.inner.read().contains_key(extension_id.as_ref())
    }

    pub fn get<S: AsRef<str>>(&self, extension_id: S) -> Option<Arc<Extension>> {
        self.inner.read().get(extension_id.as_ref()).cloned()
    }

    pub fn add(&mut self, extension: Extension) {
        self.inner
            .write()
            .insert(extension.id.clone(), Arc::new(extension));
    }

    pub fn remove<S: AsRef<str>>(&mut self, extension_id: S) {
        self.inner.write().remove(extension_id.as_ref());
    }

    pub fn to_vec(&self) -> Vec<Arc<Extension>> {
        let mut extensions: Vec<Arc<Extension>> = self
            .inner
            .cloned()
            .into_iter()
            .map(|(_, extension)| extension)
            .collect();
        extensions.sort_by(|a, b| a.source.name.cmp(&b.source.name));
        extensions
    }
}

pub struct Extension {
    id: String,
    source: Source,
    icon_path: PathBuf,
    bindings: Bindings,
}

impl Extension {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn icon_path(&self) -> &PathBuf {
        &self.icon_path
    }

    pub async fn from_path(extension_path: PathBuf) -> Result<Self> {
        // TODO: validate the extension directory
        // TODO: load other data for the extension

        let extension_id = extension_path
            .file_name()
            .ok_or(Error::UnknownPath)?
            .to_string_lossy()
            .to_string();

        let source_path = extension_path.join("source.json");
        let source_reader = std::fs::File::open(source_path)?;
        let source = serde_json::from_reader(source_reader)
            .map_err(|e| Error::ExtensionCorruption(e.to_string()))?;

        let icon_path = extension_path.join("icon.png");

        let extension_wasm = extension_path.join("extension.wasm");
        let bindings = Bindings::from_file(extension_wasm)
            .await
            .map_err(|e| Error::Wasm(e.to_string()))?;

        bindings
            .initialize()
            .await
            .map_err(|_| Error::Wasm("failed to initialize bindings".into()))?;

        Ok(Self {
            id: extension_id,
            source,
            icon_path,
            bindings,
        })
    }

    pub async fn get_manga_list(
        &self,
        filters: Vec<Filter>,
        page: u32,
    ) -> Result<(Vec<Manga>, bool)> {
        self.bindings
            .get_manga_list(filters, page)
            .await
            .map_err(|_| Error::ExtensionMethod("failed to get manga list".to_string()))
    }

    pub async fn get_manga_details(&self, manga_id: String) -> Result<Manga> {
        self.bindings
            .get_manga_details(manga_id)
            .await
            .map_err(|_| Error::ExtensionMethod("failed to get manga details".to_string()))
    }

    pub async fn get_chapter_list(&self, manga_id: String) -> Result<Vec<Chapter>> {
        self.bindings
            .get_chapter_list(manga_id)
            .await
            .map_err(|_| Error::ExtensionMethod("failed to get chapter list".to_string()))
    }

    pub async fn get_page_list(&self, manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
        self.bindings
            .get_page_list(manga_id, chapter_id)
            .await
            .map_err(|_| Error::ExtensionMethod("failed to get page list".to_string()))
    }
}
