use serde::Deserialize;

pub type Manifests = Vec<Manifest>;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Manifest {
    pub id: String,
    pub name: String,
    pub extension: String,
    pub icon: String,
    pub version: String,
    pub language: String,
    pub nsfw: bool,
}
