use dioxus::prelude::*;

use crate::model::{Manifest, ManifestsState, RepositoryUrlState};

#[component]
pub fn Extensions() -> Element {
    let mut manifests = use_context::<Signal<ManifestsState>>();
    let mut repository_url = use_context::<Signal<RepositoryUrlState>>();

    rsx! {
        input {
            r#type: "text",
            placeholder: "Extension repository URL",
            value: "{repository_url}",
            onchange: move |event| async move {
                repository_url.write().0 = event.value();
                manifests.write().0 = get_repository_extensions(event.value()).await;
            }
        }
        p { "{manifests:?}" }
    }
}

async fn get_repository_extensions(repository_url: String) -> Vec<Manifest> {
    let Ok(response) = reqwest::get(&repository_url).await else {
        return vec![];
    };

    let Ok(manifests) = response.json::<Vec<Manifest>>().await else {
        return vec![];
    };

    manifests
}
