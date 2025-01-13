use std::path::PathBuf;

use crate::error::Result;

pub fn extensions_dir() -> Result<PathBuf> {
    let dir = midoku_path::app_local_data_dir()?;
    Ok(dir.join("extensions"))
}
