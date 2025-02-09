use dioxus::prelude::*;

use crate::component::{extract_attributes, merge_attributes};

#[component]
pub fn List(
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (attributes, classes): (Vec<_>, Vec<_>) = extract_attributes!(attributes, "class");
    let class = merge_attributes!(classes, " ");

    rsx! {
        ul { class: "space-y-4 {class}", ..attributes, {children} }
    }
}
