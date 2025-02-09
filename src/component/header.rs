use dioxus::prelude::*;

#[component]
pub fn Header(
    h_align: Option<HorizontalAlign>,
    v_align: Option<VerticalAlign>,
    children: Element,
) -> Element {
    rsx! {
        header {
            class: "w-full my-5 px-5 flex gap-3 h-6",
            class: if let Some(HorizontalAlign::Start) = h_align { "justify-start" },
            class: if let Some(HorizontalAlign::Center) = h_align { "justify-center" },
            class: if let Some(HorizontalAlign::End) = h_align { "justify-end" },
            class: if let Some(VerticalAlign::Start) = v_align { "items-start" },
            class: if let Some(VerticalAlign::Center) = v_align { "items-center" },
            class: if let Some(VerticalAlign::End) = v_align { "items-end" },
            {children}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlign {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlign {
    Start,
    Center,
    End,
}
