use dioxus::prelude::*;

use crate::component::extension::{Item, ItemDescription, ItemDetail, ItemIcon, ItemTitle, List};
use crate::hook::use_state;
use crate::Route;

#[component]
pub fn SourceList() -> Element {
    let state = use_state();
    let extensions = state.extensions;

    rsx! {
        div { class: "max-w-xl mx-auto p-3",
            h1 { class: "text-2xl font-bold mb-4", "Explore" }
            List {
                for extension in extensions.to_vec().iter() {
                    Item {
                        ItemIcon {
                            src: extension.icon_path().to_string_lossy().to_string(),
                            alt: extension.source().name.clone(),
                        }
                        ItemDetail {
                            ItemTitle { title: extension.source().name.clone() }
                            ItemDescription { language: extension.source().language.clone() }
                        }
                        Link {
                            class: "ml-auto btn hover:btn-primary",
                            to: Route::MangaList {
                                extension_id: extension.id().to_string(),
                            },
                            "Browse"
                        }
                    }
                }
            }
        }
    }
}
