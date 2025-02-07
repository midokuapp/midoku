use dioxus::prelude::*;

use crate::state::MangaList;
use crate::Route;

#[component]
pub fn MangaListState() -> Element {
    use_context_provider(|| MangaList {
        mangas: Signal::new(vec![]),
        has_more: Signal::new(true),
        page: Signal::new(0),
    });

    rsx! {
        main { class: "flex flex-col h-screen", Outlet::<Route> {} }
    }
}
