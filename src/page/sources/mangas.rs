use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;
use midoku_bindings::exports::Manga;

use crate::hook::use_state;
use crate::Route;

#[component]
pub fn MangaState() -> Element {
    use_context_provider(|| MangaListState {
        mangas: Signal::new(vec![]),
        has_more: Signal::new(true),
        page: Signal::new(0),
    });

    rsx! {
        main { class: "flex flex-col h-screen", Outlet::<Route> {} }
    }
}

#[derive(Clone, Copy)]
struct MangaListState {
    mangas: Signal<Vec<Manga>>,
    has_more: Signal<bool>,
    page: Signal<u32>,
}

#[component]
pub fn MangaList(extension_id: String) -> Element {
    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(&extension_id).unwrap();

    let extension_name = extension.source().name.clone();

    let mut self_state = use_context::<MangaListState>();

    let mut loading = use_signal(|| false);

    _ = use_resource(move || {
        let extension = extension.clone();

        async move {
            while loading() && *self_state.has_more.peek() {
                let page = *self_state.page.peek();
                let Ok((mut next_mangas, next_has_more)) =
                    extension.get_manga_list(vec![], page).await
                else {
                    return;
                };
                self_state.mangas.write().append(&mut next_mangas);
                self_state.has_more.set(next_has_more);
                self_state.page.set(page + 1);
            }
        }
    });

    const WIDTH: u32 = 300;
    const HEIGHT: u32 = 450;

    rsx! {
        Header { title: extension_name.clone() }
        Grid {
            for manga in self_state.mangas.read().iter() {
                Item {
                    extension_id: extension_id.clone(),
                    manga_id: manga.id.clone(),
                    ItemImage {
                        src: format!("/gallery/?url={}&width={WIDTH}&height={HEIGHT}", manga.cover_url.clone()),
                        alt: manga.title.clone(),
                    }
                    ItemTitle { title: manga.title.clone() }
                }
            }
            li {
                class: "col-span-full flex flex-col items-center justify-center",
                onvisible: move |event| {
                    let data = event.data();
                    let is_intersecting = data.is_intersecting().unwrap_or_default();
                    if loading() != is_intersecting {
                        loading.set(is_intersecting);
                    }
                },
                div { class: "loading loading-dots" }
            }
        }
    }
}

#[component]
fn Header(title: String) -> Element {
    rsx! {
        div { class: "p-5 flex items-center gap-3",
            GoBackButton {
                Icon { class: "size-6", icon: LdArrowLeft }
            }
            h1 { class: "text-2xl font-bold", "{title}" }
        }
    }
}

#[component]
fn Grid(children: Element) -> Element {
    rsx! {
        div { class: "flex-1 overflow-y-auto",
            ul { class: "p-2 grid grid-cols-[repeat(auto-fill,minmax(100px,5fr))] gap-3",
                {children}
            }
        }
    }
}

#[component]
fn Item(extension_id: String, manga_id: String, children: Element) -> Element {
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
fn ItemImage(src: String, alt: String) -> Element {
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
fn ItemTitle(title: String) -> Element {
    rsx! {
        p { class: "mx-1 mt-1 line-clamp-2 text-sm font-bold", "{title}" }
    }
}
