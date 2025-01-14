use dioxus::prelude::*;

use crate::model::Manifest;
use crate::state::{State, StateExtensions, StateRepositoryUrl};

#[component]
pub fn Extensions() -> Element {
    let mut state = use_context::<State>();
    let extensions = state.extensions;
    let mut manifests = state.manifests;

    let mut repository_url = use_signal(|| state.repository_url());

    _ = use_resource(move || async move {
        let repository_url = repository_url.read();
        state.set_repository_url(repository_url.clone());
        let repository_extensions = get_repository_extensions(repository_url.clone()).await;
        manifests.set(repository_extensions.into());
    });

    rsx! {
        input {
            r#type: "text",
            placeholder: "Extension repository URL",
            value: "{repository_url}",
            onchange: move |event| repository_url.set(event.value()),
        }
        h2 { "Installed" }
        ul {
            {
                extensions
                    .read()
                    .iter()
                    .map(|(_, extension)| {
                        let extension_id = &extension.id;
                        rsx! {
                            li {
                                "{extension_id}"
                                UninstallButton { extension_id }
                            }
                        }
                    })
            }
        }
        h2 { "Available" }
        ul {
            {
                manifests
                    .read()
                    .iter()
                    .flat_map(|manifest| {
                        let extension_id = &manifest.id;
                        (!extensions.read().contains_key(extension_id))
                            .then(|| rsx! {
                                li {
                                    "{extension_id}"
                                    InstallButton { manifest: manifest.clone() }
                                }
                            })
                    })
            }
        }
    }
}

#[component]
pub fn InstallButton(manifest: Manifest) -> Element {
    let mut state = use_context::<State>();

    let mut disabled = use_signal(|| false);

    rsx! {
        button {
            disabled: "{disabled}",
            onclick: move |_| {
                disabled.set(true);
                let manifest = manifest.clone();
                async move { state.install_extension(&manifest).await.unwrap() }
            },
            "Install"
        }
    }
}

#[component]
pub fn UninstallButton(extension_id: String) -> Element {
    let mut state = use_context::<State>();

    rsx! {
        button {
            onclick: move |_| {
                let extension_id = extension_id.clone();
                async move { state.uninstall_extension(&extension_id).await.unwrap() }
            },
            "Uninstall"
        }
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
