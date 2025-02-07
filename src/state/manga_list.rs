use dioxus::prelude::*;
use midoku_bindings::exports::Manga;

#[derive(Clone, Copy)]
pub struct MangaList {
    pub mangas: Signal<Vec<Manga>>,
    pub has_more: Signal<bool>,
    pub page: Signal<u32>,
}
