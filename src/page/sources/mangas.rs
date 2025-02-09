use dioxus::prelude::*;

use crate::component::manga::{Grid, Item, ItemImage, ItemTitle};
use crate::component::{BackButton, Header, VerticalAlign};
use crate::hook::use_state;

#[component]
pub fn MangaList(extension_id: String) -> Element {
    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(&extension_id).unwrap();

    let extension_name = extension.read().source().name.clone();

    let mut self_state = use_context::<crate::state::MangaList>();

    let mut loading = use_signal(|| false);

    _ = use_resource(move || async move {
        while loading() && *self_state.has_more.peek() {
            let page = *self_state.page.peek();
            let Ok((mut next_mangas, next_has_more)) =
                extension.read().get_manga_list(vec![], page).await
            else {
                return;
            };
            self_state.mangas.write().append(&mut next_mangas);
            self_state.has_more.set(next_has_more);
            self_state.page.set(page + 1);
        }
    });

    const WIDTH: u32 = 300;
    const HEIGHT: u32 = 450;

    rsx! {
        Header { v_align: VerticalAlign::Center,
            BackButton {}
            h1 { class: "text-xl font-bold", "{extension_name}" }
        }
        Grid {
            for manga in self_state.mangas.read().iter() {
                Item {
                    extension_id: extension_id.clone(),
                    manga_id: manga.id.clone(),
                    ItemImage {
                        src: format!("/gallery/?url={}&width={WIDTH}&height={HEIGHT}", manga.cover_url.clone()),
                        alt: manga.title.clone(),
                    }
                    ItemTitle { title: manga.title.clone() }
                }
            }
            li {
                class: "col-span-full flex flex-col items-center justify-center",
                onvisible: move |event| {
                    let data = event.data();
                    let is_intersecting = data.is_intersecting().unwrap_or_default();
                    if loading() != is_intersecting {
                        loading.set(is_intersecting);
                    }
                },
                div { class: "loading loading-dots" }
            }
        }
    }
}
