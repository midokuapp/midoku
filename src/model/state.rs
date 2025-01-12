use super::Manifest;

#[derive(Debug, Default)]
pub struct ManifestsState(pub Vec<Manifest>);

#[derive(Default)]
pub struct RepositoryUrlState(pub String);

impl std::fmt::Display for RepositoryUrlState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
