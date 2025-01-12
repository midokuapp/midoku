use dioxus::prelude::*;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::error::Result;
use crate::model::{
    state::{ExtensionsState, ManifestsState, RepositoryUrlState},
    Extension, Manifest,
};
use crate::PATH;

#[component]
pub fn Extensions() -> Element {
    let extensions = use_context::<Signal<ExtensionsState>>();
    let mut manifests = use_context::<Signal<ManifestsState>>();
    let mut repository_url = use_context::<Signal<RepositoryUrlState>>();

    let is_installed = |extension_id: &str| extensions.read().0.contains_key(extension_id);

    rsx! {
        input {
            r#type: "text",
            placeholder: "Extension repository URL",
            value: "{repository_url}",
            onchange: move |event| async move {
                repository_url.write().0 = event.value();
                manifests.write().0 = get_repository_extensions(event.value()).await;
            }
        }
        h2 { "Installed" }
        ul {
            {extensions.read().0.iter().map(|(_, extension)| {
                let extension_id = &extension.id;
                rsx! {
                    "{extension_id}"
                    UninstallButton { extension_id }
                }
            })}
        }
        h2 { "Available" }
        ul {
            {manifests.read().0.iter().flat_map(|manifest| {
                let extension_id = &manifest.id;
                (!is_installed(extension_id)).then(|| rsx! {
                    "{extension_id}"
                    InstallButton { manifest: manifest.clone() }
                })
            })}
        }
    }
}

#[component]
pub fn InstallButton(manifest: Manifest) -> Element {
    let mut disabled = use_signal(|| false);

    rsx! {
        button {
            disabled: "{disabled}",
            onclick: move |_| {
                disabled.set(true);
                let manifest = manifest.clone();
                async move { install_extension(&manifest).await.unwrap() }
            },
            "Install"
        }
    }
}

#[component]
pub fn UninstallButton(extension_id: String) -> Element {
    rsx! {
        button {
            onclick: move |_| {
                let extension_id = extension_id.clone();
                async move { uninstall_extension(&extension_id).await.unwrap() }
            },
            "Uninstall"
        }
    }
}

async fn get_repository_extensions(repository_url: String) -> Vec<Manifest> {
    let Ok(response) = reqwest::get(&repository_url).await else {
        return vec![];
    };

    let Ok(manifests) = response.json::<Vec<Manifest>>().await else {
        return vec![];
    };

    manifests
}

async fn install_extension(manifest: &Manifest) -> Result<()> {
    let mut extensions = use_context::<Signal<ExtensionsState>>();
    let repository_url = use_context::<Signal<RepositoryUrlState>>();

    let extensions_dir = PATH.extensions_dir().expect("failed to get extensions dir");
    let extension_path = extensions_dir.join(&manifest.id);

    // If the path exists, then the extensions have already been installed.
    if extension_path.exists() {
        return Ok(());
    }

    std::fs::create_dir_all(&extension_path)?;

    let extension_package_url = format!("{}/extensions/{}", repository_url, manifest.extension);

    // Download the extension package
    let extension_package = reqwest::get(&extension_package_url)
        .await
        .expect("failed to download extension package")
        .bytes()
        .await
        .expect("failed to read extension package");

    // Extract the extension package
    let extension_package = GzDecoder::new(extension_package.as_ref());
    let mut extension_package = Archive::new(extension_package);

    // Unpack the extension package
    extension_package.unpack(&extension_path)?;

    // Register the extension
    let extension = Extension::from_path(extension_path)?;
    extensions.write().0.insert(extension.id.clone(), extension);

    Ok(())
}

async fn uninstall_extension(extension_id: &str) -> Result<()> {
    let mut extensions = use_context::<Signal<ExtensionsState>>();

    let extensions_dir = PATH.extensions_dir().expect("failed to get extensions dir");
    let extension_path = extensions_dir.join(extension_id);

    // Remove the extension directory
    std::fs::remove_dir_all(&extension_path)?;

    // Unregister the extension
    extensions.write().0.remove(extension_id);

    Ok(())
}
