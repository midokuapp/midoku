use dioxus::prelude::*;

#[component]
pub fn Grid(children: Element) -> Element {
    rsx! {
        div { class: "flex-1 overflow-y-auto",
            ul { class: "p-2 grid grid-cols-[repeat(auto-fill,minmax(100px,5fr))] gap-3",
                {children}
            }
        }
    }
}
