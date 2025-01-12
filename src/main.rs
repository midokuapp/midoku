mod error;
mod layout;
mod model;
mod page;
mod path;

use dioxus::prelude::*;
use midoku_macros::*;

use crate::layout::Navbar;
use crate::model::{
    state::{ExtensionsState, ManifestsState, RepositoryUrlState},
    Config,
};
use crate::path::PathResolver;

use crate::page::{
    browse::{Browse, BrowseExtension, BrowseManga},
    extensions::Extensions,
};

const CSS: Asset = asset!("/assets/main.css");
const CONFIG: Config = get_config!();
const PATH: PathResolver = PathResolver(CONFIG);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[redirect("/", || Route::Browse {})]

    #[nest("/browse")]
        #[layout(Navbar)]
        #[route("")]
        Browse {},
        #[end_layout]

        #[nest("/extension/:extension_id")]
            #[route("")]
            BrowseExtension { extension_id: String },

            #[nest("/manga/:manga_id")]
                #[route("")]
                BrowseManga {
                    extension_id: String,
                    manga_id: String,
                },
            #[end_nest]
        #[end_nest]
    #[end_nest]

    #[layout(Navbar)]
    #[route("/extensions")]
    Extensions {},
}

fn main() {
    #[cfg(target_os = "android")]
    dioxus::launch(App);
    #[cfg(not(target_os = "android"))]
    {
        use dioxus::desktop::{LogicalSize, WindowBuilder};

        let window = WindowBuilder::default()
            .with_title(CONFIG.name)
            .with_inner_size(LogicalSize::new(600, 1000));

        let config = dioxus::desktop::Config::default()
            .with_menu(None)
            .with_window(window);

        LaunchBuilder::new().with_cfg(config).launch(App);
    }
}

#[component]
fn App() -> Element {
    // #[cfg(not(target_os = "android"))]
    // spawn(async move {
    //     use dark_light::Mode;
    //     use dioxus::desktop::tao::window::Theme;
    //     use futures_lite::StreamExt;

    //     let window = dioxus::desktop::window();

    //     let mut stream = dark_light::subscribe().await;
    //     while let Some(mode) = stream.next().await {
    //         match mode {
    //             Mode::Dark => window.set_theme(Some(Theme::Dark)),
    //             Mode::Light => window.set_theme(Some(Theme::Light)),
    //             Mode::Default => window.set_theme(None),
    //         }
    //     }
    // });

    use_context_provider(|| Signal::new(ExtensionsState::init()));
    use_context_provider(|| Signal::new(ManifestsState::default()));
    use_context_provider(|| Signal::new(RepositoryUrlState::default()));

    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

// /// Home page
// #[component]
// fn Home() -> Element {
//     let path_resolver = PathResolver(&CONFIG);
//     let app_local_data_dir = path_resolver.app_local_data_dir().unwrap();

//     rsx! {
//         div { "{app_local_data_dir:?}" }
//     }
// }

// #[component]
// pub fn Blog(id: i32) -> Element {
//     rsx! {
//         div {
//             id: "blog",

//             // Content
//             h1 { "This is blog #{id}!" }
//             p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

//             // Navigation links
//             Link {
//                 to: Route::Blog { id: id - 1 },
//                 "Previous"
//             }
//             span { " <---> " }
//             Link {
//                 to: Route::Blog { id: id + 1 },
//                 "Next"
//             }
//         }
//     }
// }

// #[component]
// fn Navbar() -> Element {
//     rsx! {
//         div {
//             id: "navbar",
//             Link {
//                 to: Route::Home {},
//                 "Home"
//             }
//             Link {
//                 to: Route::Blog { id: 1 },
//                 "Blog"
//             }
//         }

//         Outlet::<Route> {}
//     }
// }
