use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use log::{debug, warn};
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tauri_plugin_log::{Target, TargetKind};

const EXTENSIONS_DIR: &str = "extensions";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Extension {
    pub name: String,
    pub language: String,
    pub version: String,
    pub url: String,
    pub nsfw: bool,
}

impl Extension {
    fn from_path(extension_path: PathBuf) -> Result<Self> {
        // TODO: validate the extension directory
        // TODO: load other data for the extension

        let source = extension_path.join("source.json");
        let source_reader = std::fs::File::open(source)?;
        let extension = serde_json::from_reader(source_reader)?;
        Ok(extension)
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
                        Some((extension.name.clone(), extension))
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
fn get_extensions(state: State<Extensions>) -> Vec<Extension> {
    debug!("get_extensions called");
    state.lock().iter().map(|(_, v)| v.clone()).collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
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
        .invoke_handler(tauri::generate_handler![get_extensions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
