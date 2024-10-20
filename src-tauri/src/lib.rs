use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use log::{debug, trace, warn};
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use midoku_bindings::Bindings;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tauri_plugin_log::{Target, TargetKind};

const EXTENSIONS_DIR: &str = "extensions";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Source {
    pub name: String,
    pub language: String,
    pub version: String,
    pub url: String,
    pub nsfw: bool,
}

struct Extension {
    id: String,
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
    fn from_path(extension_path: PathBuf) -> Result<Self> {
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

    fn get_manga_list(&self, filters: Vec<Filter>, page: u32) -> Result<(Vec<Manga>, bool)> {
        tokio::task::block_in_place(|| {
            self.bindings
                .get_manga_list(filters, page)
                .map_err(|_| "failed to get manga list".into())
        })
    }

    fn get_manga_details(&self, manga_id: String) -> Result<Manga> {
        tokio::task::block_in_place(|| {
            self.bindings
                .get_manga_details(manga_id)
                .map_err(|_| "failed to get manga details".into())
        })
    }

    fn get_chapter_list(&self, manga_id: String) -> Result<Vec<Chapter>> {
        tokio::task::block_in_place(|| {
            self.bindings
                .get_chapter_list(manga_id)
                .map_err(|_| "failed to get chapter list".into())
        })
    }

    fn get_page_list(&self, manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
        tokio::task::block_in_place(|| {
            self.bindings
                .get_page_list(manga_id, chapter_id)
                .map_err(|_| "failed to get page list".into())
        })
    }
}

struct Extensions {
    store: Mutex<HashMap<String, Extension>>,
}

impl Extensions {
    fn from_dir(extensions_dir: PathBuf) -> Self {
        let extensions = std::fs::read_dir(extensions_dir)
            .expect("failed to read extensions directory")
            .filter_map(|entry| {
                let entry = entry.expect("failed to read entry");
                let extension = Extension::from_path(entry.path());

                match extension {
                    Ok(extension) => {
                        debug!("loaded extension: {:?}", extension);
                        Some((extension.id.clone(), extension))
                    }
                    Err(e) => {
                        warn!("failed to load extension: {}", e);
                        None
                    }
                }
            })
            .collect();

        Self {
            store: Mutex::new(extensions),
        }
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<String, Extension>> {
        self.store.lock().expect("failed to lock extensions store")
    }
}

#[tauri::command]
async fn get_extensions(
    state: State<'_, Extensions>,
) -> tauri::Result<Vec<(String, Source, PathBuf)>> {
    trace!("get_extensions called");
    Ok(state
        .lock()
        .iter()
        .map(|(_, v)| (v.id.clone(), v.source.clone(), v.icon_path.clone()))
        .collect())
}

macro_rules! call_extension {
    ($state:expr, $extension_id:expr, $func:tt) => {
        $state
            .lock()
            .get($extension_id.as_str())
            .ok_or(tauri::Error::AssetNotFound(
                "extension not found".to_string(),
            ))
            .and_then(|extension| {
                $func(extension).map_err(|e| tauri::Error::AssetNotFound(e.to_string()))
            })
    };
}

#[tauri::command]
async fn get_manga_list(
    state: State<'_, Extensions>,
    extension_id: String,
    filters: Vec<Filter>,
    page: u32,
) -> tauri::Result<(Vec<Manga>, bool)> {
    trace!("get_manga_list called with extension_id: {}", extension_id);
    call_extension!(
        state,
        extension_id,
        (|extension: &Extension| extension.get_manga_list(filters, page))
    )
}

#[tauri::command]
async fn get_manga_details(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
) -> tauri::Result<Manga> {
    trace!(
        "get_manga_details called with extension_id: {}",
        extension_id
    );
    call_extension!(
        state,
        &extension_id,
        (|extension: &Extension| extension.get_manga_details(manga_id))
    )
}

#[tauri::command]
async fn get_chapter_list(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
) -> tauri::Result<Vec<Chapter>> {
    trace!(
        "get_chapter_list called with extension_id: {}",
        extension_id
    );
    call_extension!(
        state,
        &extension_id,
        (|extension: &Extension| extension.get_chapter_list(manga_id))
    )
}

#[tauri::command]
async fn get_page_list(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
    chapter_id: String,
) -> tauri::Result<Vec<Page>> {
    trace!("get_page_list called with extension_id: {}", extension_id);
    call_extension!(
        state,
        &extension_id,
        (|extension: &Extension| extension.get_page_list(manga_id, chapter_id))
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Trace)
                .level_for("cranelift_codegen", log::LevelFilter::Info)
                .level_for("cranelift_wasm", log::LevelFilter::Info)
                .level_for("regalloc2", log::LevelFilter::Info)
                .level_for("reqwest", log::LevelFilter::Info)
                .level_for("wasmtime", log::LevelFilter::Info)
                .level_for("wasmtime_cranelift", log::LevelFilter::Info)
                .level_for("wasmtime_environ", log::LevelFilter::Info)
                .targets([Target::new(TargetKind::Stdout)])
                .build(),
        )
        .setup(|app| {
            // Load the extensions.
            let extensions_dir: PathBuf = app
                .path()
                .app_local_data_dir()
                .expect("failed to get local app data dir")
                .join(EXTENSIONS_DIR);

            app.manage(Extensions::from_dir(extensions_dir));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_extensions,
            get_manga_list,
            get_manga_details,
            get_chapter_list,
            get_page_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
