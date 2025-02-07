use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdDownload, LdTrash2};
use dioxus_free_icons::Icon;

use crate::hook::use_state;
use crate::model::Manifest;
use crate::state::StateExtensions;

#[component]
pub fn InstallButton(manifest: Manifest) -> Element {
    let mut state = use_state();

    let mut disabled = use_signal(|| false);

    rsx! {
        button {
            class: "ml-auto btn btn-circle hover:btn-success",
            disabled: "{disabled}",
            onclick: move |_| {
                disabled.set(true);
                let manifest = manifest.clone();
                async move { state.install_extension(&manifest).await.unwrap() }
            },
            if disabled() {
                div { class: "loading loading-spinner" }
            } else {
                Icon { class: "size-4", icon: LdDownload }
            }
        }
    }
}

#[component]
pub fn UninstallButton(extension_id: String) -> Element {
    let mut state = use_state();

    rsx! {
        button {
            class: "ml-auto btn btn-circle hover:btn-error",
            onclick: move |_| {
                let extension_id = extension_id.clone();
                async move { state.uninstall_extension(&extension_id).await.unwrap() }
            },
            Icon { class: "size-4", icon: LdTrash2 }
        }
    }
}
