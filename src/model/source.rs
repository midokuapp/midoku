use serde::Deserialize;

#[derive(Deserialize)]
pub struct Source {
    pub name: String,
    pub language: String,
    pub version: String,
    pub url: String,
    pub nsfw: bool,
}
