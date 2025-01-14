use dioxus::prelude::*;

use crate::hook::use_extensions;
use crate::model::{
    state::{ManifestsState, RepositoryUrlState},
    Manifest,
};
use crate::store;

#[component]
pub fn Extensions() -> Element {
    let extensions = use_extensions();
    let mut manifests = use_context::<Signal<ManifestsState>>();
    let mut repository_url = use_signal(|| store::app_data().get_repository_url());

    _ = use_resource(move || async move {
        let repository_url = repository_url.read();
        store::app_data().set_repository_url(repository_url.clone());
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
                            "{extension_id}"
                            UninstallButton { extension_id }
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
                        (!extensions.read().contains(extension_id))
                            .then(|| rsx! {
                                "{extension_id}"
                                InstallButton { manifest: manifest.clone() }
                            })
                    })
            }
        }
    }
}

#[component]
pub fn InstallButton(manifest: Manifest) -> Element {
    let mut extensions = use_extensions();

    let mut disabled = use_signal(|| false);

    rsx! {
        button {
            disabled: "{disabled}",
            onclick: move |_| {
                disabled.set(true);
                let manifest = manifest.clone();
                async move { extensions.install(&manifest).await.unwrap() }
            },
            "Install"
        }
    }
}

#[component]
pub fn UninstallButton(extension_id: String) -> Element {
    let mut extensions = use_extensions();

    rsx! {
        button {
            onclick: move |_| {
                let extension_id = extension_id.clone();
                async move { extensions.uninstall(&extension_id).await.unwrap() }
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
