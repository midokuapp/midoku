use dioxus::prelude::*;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::error::Result;
use crate::model::{
    state::{ExtensionsState, RepositoryUrlState},
    Extension, Manifest,
};
use crate::store;

pub fn use_extensions() -> UseExtensions {
    let extensions = use_context::<Signal<ExtensionsState>>();
    UseExtensions { inner: extensions }
}

#[derive(Clone, Copy)]
pub struct UseExtensions {
    inner: Signal<ExtensionsState>,
}

impl UseExtensions {
    pub fn read(&self) -> ReadableRef<Signal<ExtensionsState>> {
        self.inner.read()
    }

    pub async fn install(&mut self, manifest: &Manifest) -> Result<()> {
        let app_store = store::app_data();
        let repository_url = app_store.get_repository_url();

        let extensions_dir = crate::util::extensions_dir().unwrap();
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
        self.inner.write().insert(extension);

        Ok(())
    }

    pub async fn uninstall(&mut self, extension_id: &str) -> Result<()> {
        let extensions_dir = crate::util::extensions_dir().unwrap();
        let extension_path = extensions_dir.join(extension_id);

        // Remove the extension directory
        std::fs::remove_dir_all(&extension_path)?;

        // Unregister the extension
        self.inner.write().remove(extension_id);

        Ok(())
    }
}
