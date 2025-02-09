use dioxus::prelude::*;

use crate::component::extension::{Item, ItemDescription, ItemDetail, ItemIcon, ItemTitle, List};
use crate::component::{Header, HorizontalAlign, VerticalAlign};
use crate::hook::use_state;
use crate::Route;

#[component]
pub fn SourceList() -> Element {
    let state = use_state();
    let extensions = state.extensions;

    rsx! {
        Header { h_align: HorizontalAlign::Center, v_align: VerticalAlign::Center,
            h1 { class: "max-w-xl w-full text-xl font-bold", "Explore" }
        }
        div { class: "px-5",
            List { class: "max-w-xl w-full mx-auto",
                for extension in extensions.to_vec().iter() {
                    Item {
                        ItemIcon {
                            src: extension.read().icon_path().to_string_lossy().to_string(),
                            alt: extension.read().source().name.clone(),
                        }
                        ItemDetail {
                            ItemTitle { title: extension.read().source().name.clone() }
                            ItemDescription { language: extension.read().source().language.clone() }
                        }
                        Link {
                            class: "ml-auto btn hover:btn-primary",
                            to: Route::MangaList {
                                extension_id: extension.read().id().to_string(),
                            },
                            "Browse"
                        }
                    }
                }
            }
        }
    }
}
