use dioxus::prelude::*;

macro_rules! extract_attributes {
    ($attributes:expr, $name:literal) => {
        $attributes
            .into_iter()
            .partition(|attr| !(attr.namespace == Some($name) || attr.name == $name))
    };
}

macro_rules! merge_attributes {
    ($attributes:expr, $sep:literal) => {
        $attributes
            .into_iter()
            .filter_map(|attr| {
                let value = match attr.value {
                    ::dioxus::dioxus_core::AttributeValue::Text(v) => v,
                    ::dioxus::dioxus_core::AttributeValue::Float(v) => ::std::format!("{v}"),
                    ::dioxus::dioxus_core::AttributeValue::Int(v) => ::std::format!("{v}"),
                    ::dioxus::dioxus_core::AttributeValue::Bool(v) => ::std::format!("{v}"),
                    _ => return None,
                };

                if attr.namespace.is_some() {
                    Some(::std::format!("{}:{}", attr.name, value))
                } else {
                    Some(value)
                }
            })
            .collect::<::std::vec::Vec<_>>()
            .join($sep)
    };
}

#[component]
pub fn List(
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (attributes, classes): (Vec<_>, Vec<_>) = extract_attributes!(attributes, "class");
    let class = merge_attributes!(classes, " ");

    rsx! {
        ul { class: "space-y-4 mb-8 {class}", ..attributes, {children} }
    }
}
