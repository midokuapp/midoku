use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

#[component]
pub fn Header(title: Option<String>, children: Element) -> Element {
    rsx! {
        div { class: "p-5 flex items-center gap-3",
            GoBackButton {
                Icon { class: "size-6", icon: LdArrowLeft }
            }
            if let Some(title) = title {
                h1 { class: "text-2xl font-bold", "{title}" }
            }
            div { class: "ml-auto", {children} }
        }
    }
}
