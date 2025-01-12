use std::path::PathBuf;

use super::Config;
use super::EXTENSIONS_DIR;

pub struct PathResolver(pub Config);

impl PathResolver {
    pub fn app_local_data_dir(&self) -> PathBuf {
        dirs::data_local_dir().unwrap().join(&self.0.identifier)
    }

    pub fn extensions_dir(&self) -> PathBuf {
        self.app_local_data_dir().join(EXTENSIONS_DIR)
    }
}
