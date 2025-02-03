use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdGlobe, LdLayoutGrid};
use dioxus_free_icons::{Icon, IconShape};

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        main { id: "outlet", Outlet::<Route> {} }
        nav { id: "navbar",
            NavLink { to: Route::SourceList {}, icon: LdGlobe, "Browse" }
            NavLink { to: Route::ExtensionList {}, icon: LdLayoutGrid, "Extensions" }
        }
    }
}

#[component]
fn NavLink<I: IconShape + Clone + PartialEq + 'static>(
    to: Route,
    icon: I,
    children: Element,
) -> Element where {
    let path: Route = use_route();

    rsx! {
        Link {
            class: if path == to { "active" },
            // active_class: "active",
            to,
            NavIcon { icon }
            {children}
        }
    }
}

#[component]
fn NavIcon<I: IconShape + Clone + PartialEq + 'static>(icon: I) -> Element where {
    rsx! {
        Icon { style: "color: inherit", icon }
    }
}
