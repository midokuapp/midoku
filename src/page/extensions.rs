use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdDownload, LdLoaderCircle, LdTrash2};
use dioxus_free_icons::Icon;

use crate::hook::use_state;
use crate::model::Manifest;
use crate::state::{StateExtensions, StateRepositoryUrl};

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
        div { class: "max-w-xl mx-auto p-3",
            h1 { class: "text-2xl font-bold mb-4", "Extension Manager" }
            p { class: "mb-4 text-gray-700",
                "Manage your manga extensions: Install new sources or uninstall those you no longer need."
            }
            input {
                class: "w-full border border-gray-500 rounded-md mb-4 p-2",
                r#type: "text",
                placeholder: "Extension repository URL",
                value: "{repository_url}",
                onchange: move |event| repository_url.set(event.value()),
            }
            h2 { class: "text-xl font-semibold mb-2", "Installed" }
            Group {
                for extension in extensions.to_vec().iter() {
                    Item {
                        ItemIcon {
                            src: extension.icon_path().to_string_lossy().to_string(),
                            alt: extension.source().name.clone(),
                        }
                        ItemDetail {
                            ItemTitle { title: extension.source().name.clone() }
                            ItemDescription {
                                language: extension.source().language.clone(),
                                version: extension.source().version.clone(),
                                nsfw: extension.source().nsfw,
                            }
                        }
                        UninstallButton { extension_id: extension.id() }
                    }
                }
            }
            h2 { class: "text-xl font-semibold mb-2", "Available" }
            Group {
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

#[component]
fn Group(children: Element) -> Element {
    rsx! {
        ul { class: "space-y-4 mb-8", {children} }
    }
}

#[component]
fn Item(children: Element) -> Element {
    rsx! {
        li { class: "flex item-center gap-4 p-3 rounded-lg shadow-md", {children} }
    }
}

#[component]
fn ItemIcon(src: String, alt: String) -> Element {
    rsx! {
        figure { class: "size-12",
            img { class: "rounded-md", src, alt }
        }
    }
}

#[component]
fn ItemDetail(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col", {children} }
    }
}

#[component]
fn ItemTitle(title: String) -> Element {
    rsx! {
        h3 { class: "text-lg font-semibold", "{title}" }
    }
}

#[component]
fn ItemDescription(language: String, version: String, nsfw: bool) -> Element {
    rsx! {
        p { class: "text-sm",
            span { class: "text-gray-700", "{language} {version}" }
            if nsfw {
                span { class: "text-red-500", " +18" }
            }
        }
    }
}

#[component]
fn InstallButton(manifest: Manifest) -> Element {
    let mut state = use_state();

    let mut disabled = use_signal(|| false);

    rsx! {
        button {
            class: "ml-auto rounded-full bg-gray-100 hover:bg-green-500 size-12 flex justify-center items-center",
            disabled: "{disabled}",
            onclick: move |_| {
                disabled.set(true);
                let manifest = manifest.clone();
                async move { state.install_extension(&manifest).await.unwrap() }
            },
            if disabled() {
                Icon { class: "size-5 animate-spin", icon: LdLoaderCircle }
            } else {
                Icon { class: "size-5", icon: LdDownload }
            }
        }
    }
}

#[component]
fn UninstallButton(extension_id: String) -> Element {
    let mut state = use_state();

    rsx! {
        button {
            class: "ml-auto rounded-full bg-gray-100 hover:bg-red-500 size-12 flex justify-center items-center",
            onclick: move |_| {
                let extension_id = extension_id.clone();
                async move { state.uninstall_extension(&extension_id).await.unwrap() }
            },
            Icon { class: "size-5", icon: LdTrash2 }
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
