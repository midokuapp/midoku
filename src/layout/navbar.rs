use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdGlobe, LdLayoutGrid};
use dioxus_free_icons::{Icon, IconShape};

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        main { class: "flex-1 p-2", Outlet::<Route> {} }
        nav { class: "w-full grid grid-cols-2 py-3",
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
    let text_color = if path == to {
        "text-black"
    } else {
        "opacity-50"
    };

    rsx! {
        Link {
            class: "flex flex-col items-center {text_color}",
            // active_class: "active",
            to,
            Icon { class: "size-4", icon }
            {children}
        }
    }
}
