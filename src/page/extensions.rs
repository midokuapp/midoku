use dioxus::prelude::*;

use crate::component::extension::{
    InstallButton, Item, ItemDescription, ItemDetail, ItemIcon, ItemTitle, List, UninstallButton,
};
use crate::component::{Header, HorizontalAlign, VerticalAlign};
use crate::hook::use_state;
use crate::model::Manifest;
use crate::state::StateRepositoryUrl;

#[component]
pub fn ExtensionList() -> Element {
    let mut state = use_state();
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
        Header { h_align: HorizontalAlign::Center, v_align: VerticalAlign::Center,
            h1 { class: "max-w-xl w-full text-xl font-bold", "Extension Manager" }
        }
        div { class: "px-5",
            div { class: "max-w-xl w-full mx-auto",
                p { class: "mb-4 opacity-70",
                    "Manage your manga extensions: Install new sources or uninstall those you no longer need."
                }
                input {
                    class: "input input-bordered w-full mb-4",
                    r#type: "text",
                    placeholder: "Extension repository URL",
                    value: "{repository_url}",
                    onchange: move |event| repository_url.set(event.value()),
                }
                if extensions.to_vec().len() > 0 {
                    h2 { class: "text-lg font-semibold mb-2", "Installed" }
                    List {
                        for extension in extensions.to_vec().iter() {
                            Item {
                                ItemIcon {
                                    src: extension.read().icon_path().to_string_lossy().to_string(),
                                    alt: extension.read().source().name.clone(),
                                }
                                ItemDetail {
                                    ItemTitle { title: extension.read().source().name.clone() }
                                    ItemDescription {
                                        language: extension.read().source().language.clone(),
                                        version: extension.read().source().version.clone(),
                                        nsfw: extension.read().source().nsfw,
                                    }
                                }
                                UninstallButton { extension_id: extension.read().id() }
                            }
                        }
                    }
                }
                if manifests().iter().filter(|manifest| !extensions.contains(&manifest.id)).count() > 0 {
                    h2 { class: "text-lg font-semibold mb-2", "Available" }
                    List {
                        for manifest in manifests().iter().filter(|manifest| !extensions.contains(&manifest.id)) {
                            Item {
                                ItemIcon {
                                    src: "{repository_url}/icons/{manifest.icon}",
                                    alt: manifest.name.clone(),
                                }
                                ItemDetail {
                                    ItemTitle { title: manifest.name.clone() }
                                    ItemDescription {
                                        language: manifest.language.clone(),
                                        version: manifest.version.clone(),
                                        nsfw: manifest.nsfw,
                                    }
                                }
                                InstallButton { manifest: manifest.clone() }
                            }
                        }
                    }
                }
            }
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
