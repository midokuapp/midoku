use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;
use midoku_bindings::exports::{Chapter, Manga};

use crate::hook::use_state;
use crate::Route;

#[component]
pub fn ChapterState(extension_id: String) -> Element {
    use_context_provider(|| ChapterListState {
        manga_details: Signal::new(None),
        chapter_list: Signal::new(vec![]),
    });

    rsx! {
        Outlet::<Route> {}
    }
}

#[derive(Clone, Copy)]
struct ChapterListState {
    manga_details: Signal<Option<Manga>>,
    chapter_list: Signal<Vec<Chapter>>,
}

#[component]
pub fn ChapterList(extension_id: String, manga_id: String) -> Element {
    let extension_id = use_signal(|| extension_id);
    let manga_id = use_signal(|| manga_id);

    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(extension_id.to_string()).unwrap();

    let mut self_state = use_context::<ChapterListState>();

    let mut loading = use_signal(|| true);

    use_future(move || {
        let extension = extension.clone();

        async move {
            let Ok(manga_details) = extension.get_manga_details(manga_id.to_string()).await else {
                dioxus::logger::tracing::error!(
                    "could not get manga details: {extension_id} {manga_id}"
                );
                return;
            };
            self_state.manga_details.set(Some(manga_details));

            let Ok(chapter_list) = extension.get_chapter_list(manga_id.to_string()).await else {
                dioxus::logger::tracing::error!(
                    "could not get chapter list: {extension_id} {manga_id}"
                );
                return;
            };
            self_state.chapter_list.set(chapter_list);

            loading.set(false);
        }
    });

    if loading() {
        return rsx!(
            p { "loading..." }
        );
    }

    let manga_details_ref = self_state.manga_details.read();
    let manga_details = manga_details_ref.as_ref().unwrap();

    let id = &manga_details.id;
    let title = &manga_details.title;

    rsx! {
        div {
            GoBackButton {
                Icon { style: "color: inherit", icon: LdArrowLeft }
            }
            h2 { "{title}" }
        }
        ul { class: "chapter-list",
            {
                self_state
                    .chapter_list
                    .read()
                    .iter()
                    .map(|chapter| {
                        let chapter_id = chapter.id.clone();
                        let chapter_title = chapter.title.clone();
                        let chapter_volume = chapter.volume;
                        let chapter_chapter = chapter.chapter;
                        rsx! {
                            li {
                                Link {
                                    to: Route::PageList {
                                        extension_id: extension_id.to_string(),
                                        manga_id: id.clone(),
                                        chapter_id,
                                    },
                                    if chapter_volume >= 0.0 {
                                        "vol {chapter_volume} ch {chapter_chapter}: {chapter_title}"
                                    } else {
                                        "ch {chapter_chapter}: {chapter_title}"
                                    }
                                }
                            }
                        }
                    })
            }
        }
    }
}
