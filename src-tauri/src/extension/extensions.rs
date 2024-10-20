use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use log::{debug, warn};

use super::Extension;

pub struct Extensions {
    store: Mutex<BTreeMap<String, Extension>>,
}

impl Extensions {
    pub fn from_dir(extensions_dir: PathBuf) -> Self {
        let extensions = std::fs::read_dir(extensions_dir)
            .expect("failed to read extensions directory")
            .filter_map(|entry| {
                let entry = entry.expect("failed to read entry");
                let extension = Extension::from_path(entry.path());

                match extension {
                    Ok(extension) => {
                        debug!("loaded extension: {:?}", extension);
                        Some((extension.id.clone(), extension))
                    }
                    Err(e) => {
                        warn!("failed to load extension: {}", e);
                        None
                    }
                }
            })
            .collect();

        Self {
            store: Mutex::new(extensions),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, BTreeMap<String, Extension>> {
        self.store.lock().expect("failed to lock extensions store")
    }
}
