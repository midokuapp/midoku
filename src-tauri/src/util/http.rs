use std::sync::LazyLock;

use tauri_plugin_http::reqwest::blocking::Client;

use crate::{Result, APP_USER_AGENT};

const CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap()
});

pub fn download_bytes<S: AsRef<str>>(url: S) -> Result<Vec<u8>> {
    let url = url.as_ref();
    let response = CLIENT.get(url).send()?;

    let image = response.bytes()?;
    let image_bytes = image.to_vec();

    Ok(image_bytes)
}
