use std::path::PathBuf;

use flate2::read::GzDecoder;
use log::trace;
use tar::Archive;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_http::reqwest;

use crate::extension::{Extension, Extensions, Manifest, Source};
use crate::{Result, EXTENSIONS_DIR};

#[tauri::command]
pub async fn get_extensions(
    state: State<'_, Extensions>,
) -> Result<Vec<(String, Source, PathBuf)>> {
    trace!("get_extensions called");
    Ok(state
        .lock()
        .iter()
        .map(|(_, v)| (v.id.clone(), v.source.clone(), v.icon_path.clone()))
        .collect())
}

#[tauri::command]
pub async fn get_repository_extensions(repository_url: String) -> Result<Vec<Manifest>> {
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
pub async fn install_extension(
    app_handle: AppHandle,
    state: State<'_, Extensions>,
    repository_url: String,
    manifest: Manifest,
) -> Result<()> {
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
pub async fn uninstall_extension(
    app_handle: AppHandle,
    state: State<'_, Extensions>,
    extension_id: String,
) -> Result<()> {
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
