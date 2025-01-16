use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;
use midoku_bindings::exports::{Chapter, Manga};
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::hook::use_state;
use crate::Route;

#[component]
pub fn ChapterState(extension_id: String, manga_id: String) -> Element {
    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(&extension_id).unwrap();

    let manga_details = block_in_place(|| {
        let handle = Handle::current();
        handle.block_on(extension.get_manga_details(manga_id.clone()))
    })
    .unwrap();

    let mut chapter_list = Signal::new(vec![]);
    use_future(move || {
        let manga_id = manga_id.clone();
        let extension = extension.clone();
        async move { chapter_list.set(extension.get_chapter_list(manga_id).await.unwrap()) }
    });

    use_context_provider(|| ChapterListState {
        manga_details: Signal::new(manga_details),
        chapter_list,
    });

    rsx! {
        Outlet::<Route> {}
    }
}

#[derive(Clone, Copy)]
struct ChapterListState {
    manga_details: Signal<Manga>,
    chapter_list: Signal<Vec<Chapter>>,
}

#[component]
pub fn ChapterList(extension_id: String, manga_id: String) -> Element {
    let self_state = use_context::<ChapterListState>();

    let manga_details = self_state.manga_details.read();
    let chapter_list = self_state.chapter_list;

    let id = &manga_details.id;
    let title = &manga_details.title;

    rsx! {
        div {
            GoBackButton {
                Icon { style: "color: inherit", icon: LdArrowLeft }
            }
            h2 { "{title}" }
        }
        ul {
            {
                chapter_list
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
                                        extension_id: extension_id.clone(),
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
