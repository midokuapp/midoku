use std::path::PathBuf;

use crate::error::{Error, Result};

/// Returns the path to the suggested directory for the app's config files.
pub fn app_config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .ok_or(Error::UnknownPath)
        .map(|dir| dir.join(midoku_config::IDENTIFIER))
}

/// Returns the path to the suggested directory for the app's data files.
pub fn app_data_dir() -> Result<PathBuf> {
    dirs::data_dir()
        .ok_or(Error::UnknownPath)
        .map(|dir| dir.join(midoku_config::IDENTIFIER))
}

/// Returns the path to the suggested directory for the app's local data files.
pub fn app_local_data_dir() -> Result<PathBuf> {
    dirs::data_local_dir()
        .ok_or(Error::UnknownPath)
        .map(|dir| dir.join(midoku_config::IDENTIFIER))
}

/// Returns the path to the suggested directory for the app's cache files.
pub fn app_cache_dir() -> Result<PathBuf> {
    dirs::cache_dir()
        .ok_or(Error::UnknownPath)
        .map(|dir| dir.join(midoku_config::IDENTIFIER))
}

/// Returns the path to the suggested directory for the app's log files.
pub fn app_log_dir() -> Result<PathBuf> {
    #[cfg(target_os = "macos")]
    let path = dirs::home_dir()
        .ok_or(Error::UnknownPath)
        .map(|dir| dir.join("Library/Logs").join(&midoku_config::IDENTIFIER));

    #[cfg(not(target_os = "macos"))]
    let path = dirs::data_local_dir()
        .ok_or(Error::UnknownPath)
        .map(|dir| dir.join(midoku_config::IDENTIFIER).join("logs"));

    path
}
