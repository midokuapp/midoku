use dioxus::prelude::*;

use crate::hook::use_state;
use crate::Route;

#[component]
pub fn SourceList() -> Element {
    let state = use_state();
    let extensions = state.extensions;

    rsx! {
        div { class: "max-w-xl mx-auto p-3",
            h1 { class: "text-2xl font-bold mb-4", "Explore" }
            Group {
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
                            class: "ml-auto px-3 py-2 rounded-md bg-gray-100 hover:bg-gray-300",
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

#[component]
fn Group(children: Element) -> Element {
    rsx! {
        ul { class: "space-y-4 mb-8", {children} }
    }
}

#[component]
fn Item(children: Element) -> Element {
    rsx! {
        li { class: "flex items-center gap-4 p-3 rounded-lg shadow-md", {children} }
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
fn ItemDescription(language: String) -> Element {
    rsx! {
        p { class: "text-sm",
            span { class: "text-gray-700", "{language}" }
        }
    }
}
