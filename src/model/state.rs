use std::collections::BTreeMap;

use super::{Extension, Manifest};

#[derive(Default)]
pub struct ExtensionsState(pub BTreeMap<String, Extension>);

#[derive(Default)]
pub struct ManifestsState(pub Vec<Manifest>);

#[derive(Default)]
pub struct RepositoryUrlState(pub String);

impl std::fmt::Display for RepositoryUrlState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
