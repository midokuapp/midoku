use dioxus::prelude::*;

use crate::component::{extract_attributes, merge_attributes};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
}

#[component]
pub fn ScrollArea(
    direction: ScrollDirection,
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (attributes, classes): (Vec<_>, Vec<_>) = extract_attributes!(attributes, "class");
    let class = merge_attributes!(classes, " ");

    rsx! {
        div {
            class: if direction == ScrollDirection::Horizontal { "overflow-x-auto" },
            class: if direction == ScrollDirection::Vertical { "overflow-y-auto" },
            class: "{class}",
            ..attributes,
            {children}
        }
    }
}
