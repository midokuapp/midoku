#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// JNI error.
    #[cfg(target_os = "android")]
    #[error("jni error: {0}")]
    Jni(#[from] jni::errors::Error),

    /// Unknown path.
    #[cfg(not(target_os = "android"))]
    #[error("unknown path")]
    UnknownPath,
}

pub type Result<T> = std::result::Result<T, Error>;
