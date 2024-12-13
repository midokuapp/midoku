use std::path::PathBuf;

use tauri_plugin_http::reqwest;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Image resize error: {0}")]
    ImageResize(#[from] fast_image_resize::ResizeError),

    #[error("Extension not found: {0}")]
    ExtensionNotFound(String),

    #[error("Extension method error: {0}")]
    ExtensionMethod(String),

    #[error("WASM error: {0}")]
    Wasm(String),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    Io(String),
    Http(String),
    FileNotFound(String),
    Parse(String),
    Image(String),
    ImageResize(String),
    ExtensionNotFound(String),
    ExtensionMethod(String),
    Wasm(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::Io(_) => ErrorKind::Io(error_message),
            Self::Http(_) => ErrorKind::Http(error_message),
            Self::FileNotFound(_) => ErrorKind::FileNotFound(error_message),
            Self::Parse(_) => ErrorKind::Parse(error_message),
            Self::Image(_) => ErrorKind::Image(error_message),
            Self::ImageResize(_) => ErrorKind::ImageResize(error_message),
            Self::ExtensionNotFound(_) => ErrorKind::ExtensionNotFound(error_message),
            Self::ExtensionMethod(_) => ErrorKind::ExtensionMethod(error_message),
            Self::Wasm(_) => ErrorKind::Wasm(error_message),
        };
        error_kind.serialize(serializer)
    }
}
