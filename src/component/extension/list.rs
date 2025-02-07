use dioxus::prelude::*;

#[component]
pub fn List(children: Element) -> Element {
    rsx! {
        ul { class: "space-y-4 mb-8", {children} }
    }
}
