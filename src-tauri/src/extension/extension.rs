use std::path::PathBuf;

use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use midoku_bindings::Bindings;
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub language: String,
    pub version: String,
    pub url: String,
    pub nsfw: bool,
}

pub struct Extension {
    pub id: String,
    pub source: Source,
    pub icon_path: PathBuf,
    bindings: Bindings,
}

impl std::fmt::Debug for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Extension")
            .field("id", &self.id)
            .field("source", &self.source)
            .field("icon_path", &self.icon_path)
            .finish()
    }
}

impl Extension {
    pub fn from_path(extension_path: PathBuf) -> Result<Self> {
        // TODO: validate the extension directory
        // TODO: load other data for the extension

        let extension_id = extension_path
            .file_name()
            .ok_or("failed to get extension id")?
            .to_string_lossy()
            .to_string();

        let source_path = extension_path.join("source.json");
        let source_reader = std::fs::File::open(source_path)?;
        let source = serde_json::from_reader(source_reader)?;

        let icon_path = extension_path.join("icon.png");

        let extension_wasm = extension_path.join("extension.wasm");
        let bindings = Bindings::from_file(extension_wasm)?;

        bindings
            .initialize()
            .map_err(|_| "failed to initialize bindings")?;

        Ok(Self {
            id: extension_id,
            source,
            icon_path,
            bindings,
        })
    }

    pub fn get_manga_list(&self, filters: Vec<Filter>, page: u32) -> Result<(Vec<Manga>, bool)> {
        tokio::task::block_in_place(|| {
            self.bindings
                .get_manga_list(filters, page)
                .map_err(|_| "failed to get manga list".into())
        })
    }

    pub fn get_manga_details(&self, manga_id: String) -> Result<Manga> {
        tokio::task::block_in_place(|| {
            self.bindings
                .get_manga_details(manga_id)
                .map_err(|_| "failed to get manga details".into())
        })
    }

    pub fn get_chapter_list(&self, manga_id: String) -> Result<Vec<Chapter>> {
        tokio::task::block_in_place(|| {
            self.bindings
                .get_chapter_list(manga_id)
                .map_err(|_| "failed to get chapter list".into())
        })
    }

    pub fn get_page_list(&self, manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
        tokio::task::block_in_place(|| {
            self.bindings
                .get_page_list(manga_id, chapter_id)
                .map_err(|_| "failed to get page list".into())
        })
    }
}
