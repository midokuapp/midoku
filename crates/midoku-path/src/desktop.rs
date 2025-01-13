use std::path::PathBuf;

use midoku_config::UseConfig;

use super::EXTENSIONS_DIR;

#[derive(Clone, Copy)]
pub struct UsePathResolver {
    pub(crate) config: UseConfig,
}

impl UsePathResolver {
    pub fn app_local_data_dir(&self) -> PathBuf {
        dirs::data_local_dir()
            .unwrap()
            .join(&self.config.identifier())
    }

    pub fn extensions_dir(&self) -> PathBuf {
        self.app_local_data_dir().join(EXTENSIONS_DIR)
    }
}
