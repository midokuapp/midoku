use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdGlobe, LdLayoutGrid};
use dioxus_free_icons::{Icon, IconShape};

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        main { class: "flex-1 p-2", Outlet::<Route> {} }
        nav { class: "w-full bg-base-200 border-t border-base-100",
            div { class: "grid grid-cols-2 max-w-xl mx-auto py-3",
                NavLink { to: Route::SourceList {}, icon: LdGlobe, "Browse" }
                NavLink { to: Route::ExtensionList {}, icon: LdLayoutGrid, "Extensions" }
            }
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
        "text-primary"
    } else {
        "opacity-50 hover:opacity-100"
    };

    rsx! {
        Link {
            class: "flex flex-col items-center p-2 text-sm {text_color}",
            // active_class: "active",
            to,
            Icon { class: "size-4", icon }
            {children}
        }
    }
}
