use dioxus::prelude::*;

use crate::state::ChapterList;
use crate::Route;

#[component]
pub fn ChapterListState(extension_id: String) -> Element {
    use_context_provider(|| ChapterList {
        manga_details: Signal::new(None),
        chapter_list: Signal::new(vec![]),
    });

    rsx! {
        Outlet::<Route> {}
    }
}
