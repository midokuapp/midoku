use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdGlobe, LdLayoutGrid, LdSettings};
use dioxus_free_icons::{Icon, IconShape};

use crate::Route;

#[component]
fn NavIcon<T: IconShape + Clone + PartialEq + 'static>(icon: T) -> Element where {
    rsx! {
        Icon {
            style: "color: inherit",
            icon
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    let path: Route = use_route();

    rsx! {
        p { "{path}" }

        main {
            id: "outlet",
            Outlet::<Route> {}
        }
        nav {
            id: "navbar",
            Link {
                class: if (path == Route::Browse {}) { "active" },
                // active_class: "active",
                to: Route::Browse {},
                NavIcon { icon: LdGlobe }
                "Browse"
            }
            Link {
                class: if (path == Route::Extensions {}) { "active" },
                // active_class: "active",
                to: Route::Extensions {},
                NavIcon { icon: LdLayoutGrid }
                "Extensions"
            }
            Link {
                class: if (path == Route::More {}) { "active" },
                // active_class: "active",
                to: Route::More {},
                NavIcon { icon: LdSettings }
                "More"
            }
        }
    }
}
