use std::path::PathBuf;

use super::CONFIG;
use super::EXTENSIONS_DIR;

pub fn app_local_data_dir() -> PathBuf {
    dirs::data_local_dir().unwrap().join(CONFIG.identifier)
}

pub fn extensions_dir() -> PathBuf {
    app_local_data_dir().join(EXTENSIONS_DIR)
}
