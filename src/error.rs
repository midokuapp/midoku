#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// JNI error.
    #[cfg(target_os = "android")]
    #[error("jni error: {0}")]
    Jni(#[from] jni::errors::Error),

    /// Unknown path.
    #[error("unknown path")]
    UnknownPath,

    /// IO error.
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    /// Extension corruption.
    #[error("extension corruption: {0}")]
    ExtensionCorruption(String),

    /// Extension method error.
    #[error("extension method error: {0}")]
    ExtensionMethod(String),

    /// WebAssembly error.
    #[error("WASM error: {0}")]
    Wasm(String),
}

pub type Result<T> = std::result::Result<T, Error>;
