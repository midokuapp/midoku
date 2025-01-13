use std::path::PathBuf;

use crate::error::{Error, Result};

pub fn app_local_data_dir() -> Result<PathBuf> {
    dirs::data_local_dir()
        .ok_or(Error::UnknownPath)
        .map(|path| path.join(midoku_config::identifier()))
}
