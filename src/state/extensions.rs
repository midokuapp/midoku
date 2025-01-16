use flate2::read::GzDecoder;
use tar::Archive;

use crate::error::Result;
use crate::model::{Extension, Manifest};

use super::{State, StateRepositoryUrl};

pub trait StateExtensions {
    async fn install_extension(&mut self, manifest: &Manifest) -> Result<()>;
    async fn uninstall_extension(&mut self, extension_id: &str) -> Result<()>;
}

impl StateExtensions for State {
    async fn install_extension(&mut self, manifest: &Manifest) -> Result<()> {
        let repository_url = self.repository_url();

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
        let extension = Extension::from_path(extension_path).await?;
        self.extensions.add(extension);

        Ok(())
    }

    async fn uninstall_extension(&mut self, extension_id: &str) -> Result<()> {
        let extensions_dir = crate::util::extensions_dir().unwrap();
        let extension_path = extensions_dir.join(extension_id);

        // Remove the extension directory
        std::fs::remove_dir_all(&extension_path)?;

        // Unregister the extension
        self.extensions.remove(extension_id);

        Ok(())
    }
}
