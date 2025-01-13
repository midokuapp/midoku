use std::path::PathBuf;

use super::{CONFIG, EXTENSIONS_DIR};

pub struct PathResolver;

impl PathResolver {
    pub fn app_local_data_dir() -> PathBuf {
        dirs::data_local_dir().unwrap().join(&CONFIG.identifier)
    }

    pub fn extensions_dir() -> PathBuf {
        Self::app_local_data_dir().join(EXTENSIONS_DIR)
    }
}
