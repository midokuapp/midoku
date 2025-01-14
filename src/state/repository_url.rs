use dioxus::prelude::*;

use super::State;

pub trait StateRepositoryUrl {
    const REPOSITORY_URL_KEY: &str;

    fn repository_url(&self) -> String;
    fn set_repository_url(&mut self, value: String);
}

impl StateRepositoryUrl for State {
    const REPOSITORY_URL_KEY: &str = "repositoryUrl";

    fn repository_url(&self) -> String {
        self.app_store
            .read()
            .get(Self::REPOSITORY_URL_KEY)
            .unwrap_or_default()
    }

    fn set_repository_url(&mut self, value: String) {
        self.app_store.write().set(Self::REPOSITORY_URL_KEY, value);
    }
}
