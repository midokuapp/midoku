use std::collections::BTreeMap;
use std::path::PathBuf;

use dioxus::logger::tracing::*;
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use midoku_bindings::Bindings;

use crate::error::{Error, Result};

use super::Source;

pub type Extensions = BTreeMap<String, Extension>;

pub struct Extension {
    pub id: String,
    pub source: Source,
    pub icon_path: PathBuf,
    bindings: Bindings,
}

pub fn init_extensions() -> Extensions {
    let extensions_dir = crate::util::extensions_dir().unwrap();
    std::fs::create_dir_all(extensions_dir.clone()).unwrap();
    std::fs::read_dir(extensions_dir)
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
        .collect()
}

impl Extension {
    pub fn from_path(extension_path: PathBuf) -> Result<Self> {
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
        let bindings =
            Bindings::from_file(extension_wasm).map_err(|e| Error::Wasm(e.to_string()))?;

        bindings
            .initialize()
            .map_err(|_| Error::Wasm("failed to initialize bindings".into()))?;

        Ok(Self {
            id: extension_id,
            source,
            icon_path,
            bindings,
        })
    }

    pub fn get_manga_list(&self, filters: Vec<Filter>, page: u32) -> Result<(Vec<Manga>, bool)> {
        self.bindings
            .get_manga_list(filters, page)
            .map_err(|_| Error::ExtensionMethod("failed to get manga list".to_string()))
    }

    pub fn get_manga_details(&self, manga_id: String) -> Result<Manga> {
        self.bindings
            .get_manga_details(manga_id)
            .map_err(|_| Error::ExtensionMethod("failed to get manga details".to_string()))
    }

    pub fn get_chapter_list(&self, manga_id: String) -> Result<Vec<Chapter>> {
        self.bindings
            .get_chapter_list(manga_id)
            .map_err(|_| Error::ExtensionMethod("failed to get chapter list".to_string()))
    }

    pub fn get_page_list(&self, manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
        self.bindings
            .get_page_list(manga_id, chapter_id)
            .map_err(|_| Error::ExtensionMethod("failed to get page list".to_string()))
    }
}
