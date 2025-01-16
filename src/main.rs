mod error;
mod hook;
mod layout;
mod model;
mod page;
mod state;
mod util;

use dioxus::prelude::*;

use crate::layout::Navbar;
use crate::state::use_state_provider;

use crate::page::{
    extensions::ExtensionList,
    sources::{ChapterList, MangaList, MangaState, PageList, SourceList},
};

const CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[redirect("/", || Route::SourceList {})]

    #[nest("/sources")]
        #[layout(Navbar)]
        #[route("")]
        SourceList {},
        #[end_layout]

        #[layout(MangaState)]
        #[nest("/:extension_id/mangas")]
            #[route("")]
            MangaList { extension_id: String },

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
            .with_title(midoku_config::name())
            .with_inner_size(LogicalSize::new(600, 1000));

        let config = dioxus::desktop::Config::default()
            .with_menu(None)
            .with_window(window);

        LaunchBuilder::new().with_cfg(config).launch(App);
    }
}

#[component]
fn App() -> Element {
    use_state_provider();

    #[cfg(not(target_os = "android"))]
    spawn(async move {
        // use dioxus::desktop::tao::window::Theme;
        use midoku_theme::prelude::*;

        // let window = dioxus::desktop::window();

        let mut stream = midoku_theme::subscribe().await;
        while let Some(mode) = stream.next().await {
            match mode {
                // Mode::Dark => window.set_theme(Some(Theme::Dark)),
                // Mode::Light => window.set_theme(Some(Theme::Light)),
                // Mode::Unspecified => window.set_theme(None),
                Mode::Dark => dioxus::logger::tracing::debug!("Dark Theme"),
                Mode::Light => dioxus::logger::tracing::debug!("Light Theme"),
                Mode::Unspecified => dioxus::logger::tracing::debug!("Default Theme"),
            }
        }
    });

    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}
