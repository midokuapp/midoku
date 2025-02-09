use dioxus::prelude::*;

#[component]
pub fn Grid(children: Element) -> Element {
    rsx! {
        ul { class: "p-2 grid grid-cols-[repeat(auto-fill,minmax(100px,5fr))] gap-3",
            {children}
        }
    }
}
