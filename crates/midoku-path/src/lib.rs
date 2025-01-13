#[cfg(target_os = "android")]
mod android;
#[cfg(not(target_os = "android"))]
mod desktop;

#[cfg(target_os = "android")]
pub use android::PathResolver;
#[cfg(not(target_os = "android"))]
pub use desktop::PathResolver;

use midoku_config::Config;
use midoku_macros::get_config;

const CONFIG: Config = get_config!();
const EXTENSIONS_DIR: &str = "extensions";
