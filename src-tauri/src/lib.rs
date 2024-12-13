mod command;
mod error;
mod extension;
mod protocol;
mod util;

use std::path::PathBuf;

use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_store::StoreExt;

use crate::extension::Extensions;
use crate::protocol::gallery;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const EXTENSIONS_DIR: &str = "extensions";
const STORE_FILE: &str = "app_data.json";

type Result<T> = std::result::Result<T, crate::error::Error>;

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
                .level(if cfg!(debug_assertions) {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .targets([Target::new(TargetKind::Stdout)])
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build());

    // Register states
    let builder = builder.manage(pool);

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
        command::extension::get_extensions,
        command::extension::get_repository_extensions,
        command::extension::install_extension,
        command::extension::uninstall_extension,
        command::source::get_manga_list,
        command::source::get_manga_details,
        command::source::get_chapter_list,
        command::source::get_page_list
    ]);

    // Register the custom protocols.
    let builder = builder.register_asynchronous_uri_scheme_protocol("gallery", gallery);

    // Run the application
    builder
        .run(ctx)
        .expect("error while running tauri application");
}
