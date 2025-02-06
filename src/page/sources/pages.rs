use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

use crate::hook::use_state;

#[component]
pub fn PageList(extension_id: String, manga_id: String, chapter_id: String) -> Element {
    let extension_id = use_signal(|| extension_id);
    let manga_id = use_signal(|| manga_id);
    let chapter_id = use_signal(|| chapter_id);

    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(extension_id.to_string()).unwrap();

    let mut page_list = use_signal(|| vec![]);
    let mut loading_index = use_signal(|| 0);

    use_future(move || {
        let extension = extension.clone();
        let manga_id = manga_id.clone();
        let chapter_id = chapter_id.clone();
        async move {
            let Ok(mut new_page_list) = extension.get_page_list(manga_id, chapter_id).await else {
                dioxus::logger::tracing::error!(
                    "could not get page list: {extension_id} {manga_id} {chapter_id}"
                );
                return;
            };
            // make sure pages are sorted by their index
            new_page_list.sort_by(|a, b| a.index.cmp(&b.index));

            // check if there are gaps or repeated indices
            if let Some(page) = new_page_list.first() {
                if page.index != 0 {
                    dioxus::logger::tracing::error!(
                        "pages do not start with index 0: {extension_id} {manga_id} {chapter_id}"
                    );
                    return;
                }
            }

            let last_index = new_page_list.len() as u32 - 1;
            if let Some(page) = new_page_list.last() {
                if page.index != last_index {
                    dioxus::logger::tracing::error!(
                        "pages do not end with index {last_index}: {extension_id} {manga_id} {chapter_id}"
                    );
                    return;
                }
            }

            for (idx, b) in new_page_list.iter().enumerate().skip(1) {
                let a = new_page_list.get(idx - 1).unwrap();
                if a.index == b.index {
                    dioxus::logger::tracing::error!(
                        "pages cannot have duplicate indices: {extension_id} {manga_id} {chapter_id}"
                    );
                    return;
                }
            }

            page_list.set(new_page_list);
        }
    });

    rsx! {
        div { GoBackButton {
            Icon { style: "color: inherit", icon: LdArrowLeft }
        } }
        ul { id: "page-view",
            for page in page_list.read().iter() {
                Page {
                    index: page.index.clone(),
                    url: page.url.clone(),
                    loading_index,
                }
            }
        }
    }
}

#[component]
fn Page(index: u32, url: String, mut loading_index: Signal<u32>) -> Element {
    let mut loading = use_signal(|| false);

    use_effect(move || {
        if index == loading_index() {
            loading.set(true);
        }
    });

    if !loading() {
        return rsx! {
            p { "loading..." }
        };
    }

    rsx! {
        li { class: "page",
            img { onload: move |_| loading_index += 1, src: "{url}" }
        }
    }
}
