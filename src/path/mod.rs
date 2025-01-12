use crate::error::*;
use crate::model::Config;

#[cfg(target_os = "android")]
mod android;
#[cfg(not(target_os = "android"))]
mod desktop;

const EXTENSIONS_DIR: &str = "extensions";

#[cfg(target_os = "android")]
pub use android::PathResolver;
#[cfg(not(target_os = "android"))]
pub use desktop::PathResolver;
