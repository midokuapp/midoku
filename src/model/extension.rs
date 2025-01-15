use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use dioxus::logger::tracing::*;
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use midoku_bindings::Bindings;

use crate::error::{Error, Result};

use super::Source;

pub type Extensions = BTreeMap<String, Arc<Extension>>;

pub struct Extension {
    id: String,
    source: Source,
    icon_path: PathBuf,
    bindings: Bindings,
}

pub async fn init_extensions() -> Extensions {
    let extensions_dir = crate::util::extensions_dir().unwrap();
    std::fs::create_dir_all(extensions_dir.clone()).unwrap();

    let mut extensions = Extensions::new();
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

    extensions
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
