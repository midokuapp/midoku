mod chapter_list;
mod extensions;
mod manga_list;
mod repository_url;

pub use chapter_list::ChapterList;
pub use extensions::StateExtensions;
pub use manga_list::MangaList;
pub use repository_url::StateRepositoryUrl;

use dioxus::prelude::*;
use midoku_store::Store;

use crate::model::{Extensions, Manifests};

#[derive(Clone, Copy)]
pub struct State {
    pub app_store: Signal<Store>,
    pub extensions: Extensions,
    pub manifests: Signal<Manifests>,
}
