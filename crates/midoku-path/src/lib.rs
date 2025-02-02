pub mod error;

#[cfg(target_os = "android")]
mod android;
#[cfg(not(target_os = "android"))]
mod desktop;

#[cfg(target_os = "android")]
pub use android::*;
#[cfg(not(target_os = "android"))]
pub use desktop::*;
