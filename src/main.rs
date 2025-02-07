mod component;
mod error;
mod hook;
mod layout;
mod model;
mod page;
mod state;
mod util;

use const_format::concatcp;
use dioxus::prelude::*;

use crate::hook::{use_gallery_handler, use_mode_provider, use_state_provider};
use crate::layout::{ChapterListState, MangaListState, Navbar};

use crate::page::{
    extensions::ExtensionList,
    sources::{ChapterList, MangaList, PageList, SourceList},
};

const APP_USER_AGENT: &str = concatcp!(midoku_config::NAME, "/", midoku_config::VERSION);
const CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[redirect("/", || Route::SourceList {})]

    #[nest("/sources")]
        #[layout(Navbar)]
        #[route("")]
        SourceList {},
        #[end_layout]

        #[layout(MangaListState)]
        #[nest("/:extension_id/mangas")]
            #[route("")]
            MangaList { extension_id: String },

            #[layout(ChapterListState)]
            #[nest("/:manga_id")]
                #[route("")]
                ChapterList {
                    extension_id: String,
                    manga_id: String
                },

                #[route("/chapter/:chapter_id")]
                PageList {
                    extension_id: String,
                    manga_id: String,
                    chapter_id: String
                },
            #[end_nest]
            #[end_layout]
        #[end_nest]
        #[end_layout]
    #[end_nest]

    #[layout(Navbar)]
    #[route("/extensions")]
    ExtensionList {},
}

fn main() {
    #[cfg(target_os = "android")]
    dioxus::launch(App);
    #[cfg(not(target_os = "android"))]
    {
        use dioxus::desktop::{LogicalSize, WindowBuilder};

        let window = WindowBuilder::default()
            .with_title(midoku_config::NAME)
            .with_inner_size(LogicalSize::new(600, 1000));

        let config = dioxus::desktop::Config::default()
            .with_menu(None)
            .with_window(window);

        LaunchBuilder::new().with_cfg(config).launch(App);
    }
}

#[component]
fn App() -> Element {
    use_gallery_handler();
    use_state_provider();
    use_mode_provider();

    rsx! {
        div { class: "flex flex-col h-screen",
            document::Stylesheet { href: CSS }
            Router::<Route> {}
        }
    }
}
