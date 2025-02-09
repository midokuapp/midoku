use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

#[component]
pub fn BackButton() -> Element {
    rsx! {
        GoBackButton {
            Icon { class: "size-6", icon: LdArrowLeft }
        }
    }
}
