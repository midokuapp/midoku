use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub id: String,
    pub name: String,
    pub extension: String,
    pub icon: String,
    pub version: String,
    pub language: String,
    pub nsfw: bool,
}
