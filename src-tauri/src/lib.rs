mod extension;

use std::path::PathBuf;

use flate2::read::GzDecoder;
use log::trace;
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use tar::Archive;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_http::reqwest;
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_store::StoreExt;

use crate::extension::{Extension, Extensions, Manifest, Source};

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
    trace!(
        "get_repository_extensions called with repository_url: {}",
        repository_url
    );
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

#[tauri::command]
async fn install_extension(
    app_handle: AppHandle,
    state: State<'_, Extensions>,
    repository_url: String,
    manifest: Manifest,
) -> tauri::Result<()> {
    trace!("install_extension called with manifest: {:?}", manifest);

    let app_local_data_dir: PathBuf = app_handle
        .path()
        .app_local_data_dir()
        .expect("failed to get local app data dir");

    let extension_dir = app_local_data_dir.join(EXTENSIONS_DIR);
    let extension_path = extension_dir.join(&manifest.id);

    if extension_path.exists() {
        return Ok(());
    }

    std::fs::create_dir_all(&extension_path)?;

    let extension_package_url = format!("{}/extensions/{}", repository_url, manifest.extension);

    // Download the extension package
    let extension_package = reqwest::get(&extension_package_url)
        .await
        .expect("failed to download extension package");
    let extension_package = extension_package
        .bytes()
        .await
        .expect("failed to read extension package");

    // Extract the extension package
    let extension_package = GzDecoder::new(extension_package.as_ref());
    let mut extension_package = Archive::new(extension_package);

    // Unpack the extension package
    extension_package.unpack(&extension_path)?;

    // Register the extension
    Extension::from_path(extension_path)
        .and_then(|extension| {
            state.lock().insert(extension.id.clone(), extension);
            Ok(())
        })
        .expect("failed to register extension");

    Ok(())
}

#[tauri::command]
async fn uninstall_extension(
    app_handle: AppHandle,
    state: State<'_, Extensions>,
    extension_id: String,
) -> tauri::Result<()> {
    trace!(
        "uninstall_extension called with extension_id: {}",
        extension_id
    );

    let app_local_data_dir: PathBuf = app_handle
        .path()
        .app_local_data_dir()
        .expect("failed to get local app data dir");

    let extension_dir = app_local_data_dir.join(EXTENSIONS_DIR);
    let extension_path = extension_dir.join(&extension_id);

    // Remove the extension directory
    std::fs::remove_dir_all(&extension_path)?;

    // Unregister the extension
    state.lock().remove(&extension_id);

    Ok(())
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
    let mut ctx = tauri::generate_context!();
    let builder = tauri::Builder::default();

    // Register the plugins
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let builder = builder.plugin(tauri_plugin_theme::init(ctx.config_mut()));
    let builder = builder
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
        .plugin(tauri_plugin_store::Builder::default().build());

    // Setup the app
    let builder = builder.setup(|app| {
        let app_local_data_dir: PathBuf = app
            .path()
            .app_local_data_dir()
            .expect("failed to get local app data dir");

        let extensions_dir = app_local_data_dir.join(EXTENSIONS_DIR);
        std::fs::create_dir_all(&extensions_dir).expect("failed to create extensions dir");

        // Load the extensions.
        let extensions = Extensions::from_dir(extensions_dir);
        app.manage(extensions);

        // Load the store.
        let _store = app.store(STORE_FILE)?;

        Ok(())
    });

    // Register the commands
    let builder = builder.invoke_handler(tauri::generate_handler![
        get_extensions,
        get_repository_extensions,
        install_extension,
        uninstall_extension,
        get_manga_list,
        get_manga_details,
        get_chapter_list,
        get_page_list
    ]);

    // Run the application
    builder
        .run(ctx)
        .expect("error while running tauri application");
}
