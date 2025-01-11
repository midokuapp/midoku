use std::path::PathBuf;

use super::{Config, Error, Result};

pub struct PathResolver<'a>(pub(crate) &'a Config<'a>);

impl PathResolver<'_> {
    pub fn app_local_data_dir(&self) -> Result<PathBuf> {
        dirs::data_local_dir()
            .ok_or(Error::UnknownPath)
            .map(|dir| dir.join(&self.0.identifier))
    }
}
