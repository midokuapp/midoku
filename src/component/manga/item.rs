use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Item(extension_id: String, manga_id: String, children: Element) -> Element {
    rsx! {
        li {
            Link {
                to: Route::ChapterList {
                    extension_id,
                    manga_id,
                },
                {children}
            }
        }
    }
}

#[component]
pub fn ItemImage(src: String, alt: String) -> Element {
    let mut loading = use_signal(|| true);

    rsx! {
        figure {
            class: {
                format!(
                    "w-full aspect-[2/3] rounded-md bg-base-300 {}",
                    if loading() { "animate-pulse" } else { "" },
                )
            },
            img {
                class: "w-full h-full object-cover rounded-md",
                src,
                alt,
                onload: move |_| loading.set(false),
            }
        }
    }
}

#[component]
pub fn ItemTitle(title: String) -> Element {
    rsx! {
        p { class: "mx-1 mt-1 line-clamp-2 text-sm font-bold", "{title}" }
    }
}
