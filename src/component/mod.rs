mod button;
mod header;
mod scroll_area;

pub mod extension;
pub mod manga;

pub use button::*;
pub use header::*;
pub use scroll_area::*;

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

use extract_attributes;
use merge_attributes;
