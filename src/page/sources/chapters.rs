use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdBrush, LdClock, LdUser};
use dioxus_free_icons::Icon;
use midoku_bindings::exports::Status;

use crate::component::{BackButton, Header, ScrollArea, ScrollDirection, VerticalAlign};
use crate::hook::use_state;
use crate::Route;

#[component]
pub fn ChapterList(extension_id: String, manga_id: String) -> Element {
    let extension_id = use_signal(|| extension_id);
    let manga_id = use_signal(|| manga_id);

    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(extension_id.to_string()).unwrap();

    let mut self_state = use_context::<crate::state::ChapterList>();

    let mut loading_manga_details = use_signal(|| self_state.manga_details.peek().is_none());
    let mut loading_chapter_list = use_signal(|| self_state.chapter_list.peek().is_empty());

    use_future(move || async move {
        if !loading_manga_details() {
            return;
        }

        let Ok(manga_details) = extension
            .read()
            .get_manga_details(manga_id.to_string())
            .await
        else {
            dioxus::logger::tracing::error!(
                "could not get manga details: {extension_id} {manga_id}"
            );
            return;
        };
        self_state.manga_details.set(Some(manga_details));

        loading_manga_details.set(false);
    });

    use_future(move || async move {
        if !loading_chapter_list() {
            return;
        }

        let Ok(chapter_list) = extension
            .read()
            .get_chapter_list(manga_id.to_string())
            .await
        else {
            dioxus::logger::tracing::error!(
                "could not get chapter list: {extension_id} {manga_id}"
            );
            return;
        };
        self_state.chapter_list.set(chapter_list);

        loading_chapter_list.set(false);
    });

    if loading_manga_details() {
        return rsx!(
            Header { v_align: VerticalAlign::Center, BackButton {} }
            div { class: "flex-1 flex flex-col items-center justify-center",
                span { class: "loading loading-spinner loading-xl" }
            }
        );
    }

    let manga_details_ref = self_state.manga_details.read();
    let manga_details = manga_details_ref.as_ref().unwrap();

    let id = &manga_details.id;
    let title = &manga_details.title;
    // let url = &manga_details.url;
    let description = &manga_details.description;
    let cover_url = &manga_details.cover_url;
    let author_name = &manga_details.author_name;
    let artist_name = &manga_details.artist_name;
    let categories = &manga_details.categories;
    let status = match manga_details.status {
        Status::Unknown => "Unknown",
        Status::Ongoing => "Ongoing",
        Status::Completed => "Completed",
        Status::Hiatus => "Hiatus",
        Status::Cancelled => "Cancelled",
    };
    // let content_rating = &manga_details.content_rating;
    // let reading_mode = &manga_details.reading_mode;

    let mut cover_loading = use_signal(|| true);

    rsx! {
        Header { v_align: VerticalAlign::Center, BackButton {} }
        ScrollArea { direction: ScrollDirection::Vertical,
            div { class: "max-w-xl w-full h-full mx-auto",
                div { class: "flex gap-3 mb-2",
                    figure {
                        class: format!(
                            "flex-none w-48 aspect-[2/3] rounded-md bg-base-300 {}",
                            if cover_loading() { "animate-pulse" } else { "" },
                        ),
                        img {
                            class: "w-full h-full object-cover rounded-md",
                            src: "{cover_url}",
                            alt: "{title}",
                            onload: move |_| cover_loading.set(false),
                        }
                    }
                    div {
                        class: "grow flex flex-col gap-2 w-full",
                        h1 { class: "text-3xl", "{title}" }
                        div {
                            class: "flex gap-2 items-center",
                            Icon { class: "size-4", icon: LdUser }
                            span { "{author_name}" }
                        }
                        div {
                            class: "flex gap-2 items-center",
                            Icon { class: "size-4", icon: LdBrush }
                            span { "{artist_name}" }
                        }
                        div {
                            class: "flex gap-2 items-center",
                            Icon { class: "size-4", icon: LdClock }
                            span { "{status}" }
                        }
                        div {
                            class: "flex flex-wrap content-start gap-1 grow",
                            for category in categories.iter() {
                                span { class: "badge badge-outline", "{category}" }
                            }
                        }
                    }
                }
                MangaDescription { description }

                if loading_chapter_list() {
                    div { class: "flex-1 flex flex-col items-center justify-center",
                        span { class: "loading loading-spinner loading-xl" }
                    }
                } else {
                    ul {
                        for chapter in self_state.chapter_list.read().iter() {
                            li {
                                Link {
                                    to: Route::PageList {
                                        extension_id: extension_id.to_string(),
                                        manga_id: id.clone(),
                                        chapter_id: chapter.id.clone(),
                                    },
                                    if chapter.volume >= 0.0 {
                                        "vol {chapter.volume} ch {chapter.chapter}: {chapter.title}"
                                    } else {
                                        "ch {chapter.chapter}: {chapter.title}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MangaDescription(description: String) -> Element {
    use dioxus_free_icons::icons::ld_icons::{LdChevronDown, LdChevronUp};

    let mut expanded = use_signal(|| false);

    rsx! {
        div {
            onclick: move |_| expanded.toggle(),

            p {
                class: "break-words mb-2",
                class: if expanded() {
                    "line-clamp-none"
                } else {
                    "line-clamp-3"
                },
                "{description}",
            }

            div {
                class: "flex justify-center",
                if expanded() {
                    Icon { class: "size-6", icon: LdChevronUp }
                } else {
                    Icon { class: "size-6", icon: LdChevronDown }
                }
            }
        }
    }
}
