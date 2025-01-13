#[cfg(target_os = "android")]
mod android;
#[cfg(not(target_os = "android"))]
mod desktop;

#[cfg(target_os = "android")]
pub use android::UsePathResolver;
#[cfg(not(target_os = "android"))]
pub use desktop::UsePathResolver;

use midoku_config::use_config;

const EXTENSIONS_DIR: &str = "extensions";

pub fn use_path_resolver() -> UsePathResolver {
    let config = use_config();
    UsePathResolver { config }
}
