use dioxus::prelude::*;

#[component]
pub fn Item(children: Element) -> Element {
    rsx! {
        li { class: "flex items-center gap-4 p-3 rounded-lg shadow-md", {children} }
    }
}

#[component]
pub fn ItemIcon(src: String, alt: String) -> Element {
    rsx! {
        figure { class: "size-12",
            img { class: "rounded-md", src, alt }
        }
    }
}

#[component]
pub fn ItemDetail(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col", {children} }
    }
}

#[component]
pub fn ItemTitle(title: String) -> Element {
    rsx! {
        h3 { class: "text-lg font-semibold", "{title}" }
    }
}

#[component]
pub fn ItemDescription(
    language: Option<String>,
    version: Option<String>,
    nsfw: Option<bool>,
) -> Element {
    rsx! {
        p { class: "text-sm",
            if let Some(language) = language { span { class: "opacity-70", "{language}"} }
            if let Some(version) = version { span { class: "opacity-70", " {version}"} }
            if let Some(nsfw) = nsfw { span { class: "text-error", " +18"} }
        }
    }
}
