use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

use crate::hook::use_state;

#[component]
pub fn PageList(extension_id: String, manga_id: String, chapter_id: String) -> Element {
    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(&extension_id).unwrap();

    let mut page_list = use_signal(|| vec![]);
    use_future(move || {
        let extension = extension.clone();
        let manga_id = manga_id.clone();
        let chapter_id = chapter_id.clone();
        async move { page_list.set(extension.get_page_list(manga_id, chapter_id).await.unwrap()) }
    });

    rsx! {
        div {
            GoBackButton {
                Icon { style: "color: inherit", icon: LdArrowLeft }
            }
        }
        ul {
            {
                page_list
                    .read()
                    .iter()
                    .map(|page| {
                        let page_url = page.url.clone();
                        rsx! {
                            li {
                                img { src: "{page_url}" }
                            }
                        }
                    })
            }
        }
    }
}
