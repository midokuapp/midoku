use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

#[component]
pub fn Header(title: String) -> Element {
    rsx! {
        div { class: "p-5 flex items-center gap-3",
            GoBackButton {
                Icon { class: "size-6", icon: LdArrowLeft }
            }
            h1 { class: "text-2xl font-bold", "{title}" }
        }
    }
}
