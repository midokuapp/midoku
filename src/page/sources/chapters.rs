use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

#[component]
pub fn ChapterList(extension_id: String, manga_id: String) -> Element {
    rsx! {
        div {
            GoBackButton {
                Icon { style: "color: inherit", icon: LdArrowLeft }
            }
            h2 { "{manga_id}" }
        }
    }
}
