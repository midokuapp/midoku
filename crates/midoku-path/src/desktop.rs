use std::path::PathBuf;

use super::EXTENSIONS_DIR;

pub fn app_local_data_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap()
        .join(midoku_config::identifier())
}

pub fn extensions_dir() -> PathBuf {
    app_local_data_dir().join(EXTENSIONS_DIR)
}
