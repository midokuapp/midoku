mod extension;

use std::path::PathBuf;

use log::trace;
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use tauri::{Manager, State};
use tauri_plugin_http::reqwest;
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_store::StoreExt;

use crate::extension::{Extensions, Manifest, Source};

const EXTENSIONS_DIR: &str = "extensions";
const STORE_FILE: &str = "app_data.json";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

#[tauri::command]
async fn get_repository_extensions(repository_url: String) -> tauri::Result<Vec<Manifest>> {
    trace!("get_repository_extensions called");
    let response = reqwest::get(&repository_url).await;

    if response.is_err() {
        return Ok(vec![]);
    }

    let response = response.unwrap().json::<Vec<Manifest>>().await;

    if response.is_err() {
        return Ok(vec![]);
    }

    Ok(response.unwrap())
}

macro_rules! call_extension {
    ($state:expr, $extension_id:expr, $method:ident, $($args:expr),*) => {{
        trace!("{} called with extension_id: {}", stringify!($method), $extension_id);
        $state
            .lock()
            .get($extension_id.as_str())
            .ok_or(tauri::Error::AssetNotFound(
                "extension not found".to_string(),
            ))
            .and_then(|extension| {
                extension.$method($($args),*).map_err(|e| tauri::Error::AssetNotFound(e.to_string()))
            })
    }};
}

#[tauri::command]
async fn get_manga_list(
    state: State<'_, Extensions>,
    extension_id: String,
    filters: Vec<Filter>,
    page: u32,
) -> tauri::Result<(Vec<Manga>, bool)> {
    call_extension!(state, extension_id, get_manga_list, filters, page)
}

#[tauri::command]
async fn get_manga_details(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
) -> tauri::Result<Manga> {
    call_extension!(state, extension_id, get_manga_details, manga_id)
}

#[tauri::command]
async fn get_chapter_list(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
) -> tauri::Result<Vec<Chapter>> {
    call_extension!(state, extension_id, get_chapter_list, manga_id)
}

#[tauri::command]
async fn get_page_list(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
    chapter_id: String,
) -> tauri::Result<Vec<Page>> {
    call_extension!(state, extension_id, get_page_list, manga_id, chapter_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
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
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_local_data_dir: PathBuf = app
                .path()
                .app_local_data_dir()
                .expect("failed to get local app data dir");

            // Load the extensions.
            let extensions_dir = app_local_data_dir.join(EXTENSIONS_DIR);
            let extensions = Extensions::from_dir(extensions_dir);
            app.manage(extensions);

            // Load the store.
            let _store = app.store(STORE_FILE)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_extensions,
            get_repository_extensions,
            get_manga_list,
            get_manga_details,
            get_chapter_list,
            get_page_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
