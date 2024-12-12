mod extension;
mod util;

use std::path::PathBuf;

use flate2::read::GzDecoder;
use log::trace;
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use tar::Archive;
use tauri::http::Response;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_http::reqwest;
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_store::StoreExt;

use crate::extension::{Extension, Extensions, Manifest, Source};
use crate::util::{http, image};

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
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
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads.min(4))
        .build()
        .unwrap();

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
                .level_for("async_io", log::LevelFilter::Info)
                .level_for("cranelift_codegen", log::LevelFilter::Info)
                .level_for("cranelift_wasm", log::LevelFilter::Info)
                .level_for("polling", log::LevelFilter::Info)
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

        // Manage the thread pool
        app.manage(pool);

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

    let builder =
        builder.register_asynchronous_uri_scheme_protocol("gallery", |app, request, responder| {
            let pool = app.app_handle().state::<rayon::ThreadPool>();

            let not_found = Response::builder().status(404).body(Vec::new()).unwrap();

            if request.method() != "GET" {
                responder.respond(not_found);
                return;
            }

            let uri = request.uri();
            let query = uri.query().unwrap_or_default();

            let image_url = {
                let start_pos = match query.find("url=") {
                    Some(pos) => pos + 4,
                    None => {
                        responder.respond(not_found);
                        return;
                    }
                };
                let end_pos = match query[start_pos..].find("&") {
                    Some(pos) => start_pos + pos,
                    None => start_pos + query[start_pos..].len(),
                };
                match urlencoding::decode(&query[start_pos..end_pos]) {
                    Ok(url) => url.to_string(),
                    Err(_) => {
                        responder.respond(not_found);
                        return;
                    }
                }
            };

            let image_extension = {
                let extension = reqwest::Url::parse(&image_url).ok().and_then(|url| {
                    url.path()
                        .rsplit_once('.')
                        .map(|(_, extension)| extension.to_string())
                });

                match extension.as_deref() {
                    Some("png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp") => extension.unwrap(),
                    _ => {
                        responder.respond(not_found);
                        return;
                    }
                }
            };

            let width = if let Some(start_pos) = query.find("width=") {
                let start_pos = start_pos + 6;
                let end_pos = match query[start_pos..].find("&") {
                    Some(pos) => start_pos + pos,
                    None => start_pos + query[start_pos..].len(),
                };
                let raw = &query[start_pos..end_pos];
                raw.parse::<u32>().ok()
            } else {
                None
            };

            let height = if let Some(start_pos) = query.find("height=") {
                let start_pos = start_pos + 7;
                let end_pos = match query[start_pos..].find("&") {
                    Some(pos) => start_pos + pos,
                    None => start_pos + query[start_pos..].len(),
                };
                let raw = &query[start_pos..end_pos];
                raw.parse::<u32>().ok()
            } else {
                None
            };

            pool.spawn(move || {
                let image_src = match http::download_bytes(image_url) {
                    Ok(src) => src,
                    Err(_) => {
                        responder.respond(not_found);
                        return;
                    }
                };
                let image_src = match image::resize(image_src, width, height) {
                    Ok(src) => src,
                    Err(_) => {
                        responder.respond(not_found);
                        return;
                    }
                };
                let response = match Response::builder()
                    .header("Content-Type", format!("image/{}", image_extension))
                    .body(image_src)
                {
                    Ok(response) => response,
                    Err(_) => {
                        responder.respond(not_found);
                        return;
                    }
                };
                responder.respond(response);
            });
        });

    // Run the application
    builder
        .run(ctx)
        .expect("error while running tauri application");
}
