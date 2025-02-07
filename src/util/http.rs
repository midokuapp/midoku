use std::sync::LazyLock;

use reqwest::Client;

use crate::error::Result;
use crate::APP_USER_AGENT;

const CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("could not build http client.")
});

pub async fn download_bytes<S: AsRef<str>>(url: S) -> Result<Vec<u8>> {
    let url = url.as_ref();
    let response = CLIENT.get(url).send().await?;

    let raw = response.bytes().await?;
    let bytes = raw.to_vec();

    Ok(bytes)
}
