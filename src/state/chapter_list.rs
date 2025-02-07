use dioxus::prelude::*;
use midoku_bindings::exports::{Chapter, Manga};

#[derive(Clone, Copy)]
pub struct ChapterList {
    pub manga_details: Signal<Option<Manga>>,
    pub chapter_list: Signal<Vec<Chapter>>,
}
