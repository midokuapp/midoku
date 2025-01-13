pub mod error;

#[cfg(target_os = "android")]
mod android;
#[cfg(not(target_os = "android"))]
mod desktop;

#[cfg(target_os = "android")]
pub use android::*;
#[cfg(not(target_os = "android"))]
pub use desktop::*;

use std::path::PathBuf;

use crate::error::Result;

const EXTENSIONS_DIR: &str = "extensions";

pub fn extensions_dir() -> Result<PathBuf> {
    app_local_data_dir().map(|path| path.join(EXTENSIONS_DIR))
}
