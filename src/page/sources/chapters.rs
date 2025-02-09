use dioxus::prelude::*;

use crate::component::extension::List;
use crate::component::{BackButton, Header, ScrollArea, ScrollDirection, VerticalAlign};
use crate::hook::use_state;
use crate::Route;

#[component]
pub fn ChapterList(extension_id: String, manga_id: String) -> Element {
    let extension_id = use_signal(|| extension_id);
    let manga_id = use_signal(|| manga_id);

    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(extension_id.to_string()).unwrap();

    let mut self_state = use_context::<crate::state::ChapterList>();

    let mut loading_manga_details = use_signal(|| self_state.manga_details.peek().is_none());
    let mut loading_chapter_list = use_signal(|| self_state.chapter_list.peek().is_empty());

    use_future(move || async move {
        if !loading_manga_details() {
            return;
        }

        let Ok(manga_details) = extension
            .read()
            .get_manga_details(manga_id.to_string())
            .await
        else {
            dioxus::logger::tracing::error!(
                "could not get manga details: {extension_id} {manga_id}"
            );
            return;
        };
        self_state.manga_details.set(Some(manga_details));

        loading_manga_details.set(false);
    });

    use_future(move || async move {
        if !loading_chapter_list() {
            return;
        }

        let Ok(chapter_list) = extension
            .read()
            .get_chapter_list(manga_id.to_string())
            .await
        else {
            dioxus::logger::tracing::error!(
                "could not get chapter list: {extension_id} {manga_id}"
            );
            return;
        };
        self_state.chapter_list.set(chapter_list);

        loading_chapter_list.set(false);
    });

    if loading_manga_details() {
        return rsx!(
            Header { v_align: VerticalAlign::Center, BackButton {} }
            div { class: "flex-1 flex flex-col items-center justify-center",
                span { class: "loading loading-spinner loading-xl" }
            }
        );
    }

    let manga_details_ref = self_state.manga_details.read();
    let manga_details = manga_details_ref.as_ref().unwrap();

    let id = &manga_details.id;
    let title = &manga_details.title;

    rsx! {
        Header { v_align: VerticalAlign::Center, BackButton {} }
        if loading_chapter_list() {
            div { class: "flex-1 flex flex-col items-center justify-center",
                span { class: "loading loading-spinner loading-xl" }
            }
        } else {
            ScrollArea {
                direction: ScrollDirection::Vertical,
                ul {
                    for chapter in self_state.chapter_list.read().iter() {
                        li {
                            Link {
                                to: Route::PageList {
                                    extension_id: extension_id.to_string(),
                                    manga_id: id.clone(),
                                    chapter_id: chapter.id.clone(),
                                },
                                if chapter.volume >= 0.0 {
                                    "vol {chapter.volume} ch {chapter.chapter}: {chapter.title}"
                                } else {
                                    "ch {chapter.chapter}: {chapter.title}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
