use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;
use midoku_bindings::exports::Manga;

use crate::hook::use_state;
use crate::Route;

#[component]
pub fn MangaState() -> Element {
    use_context_provider(|| MangaListState {
        mangas: Signal::new(vec![]),
        has_more: Signal::new(true),
        page: Signal::new(0),
    });

    rsx! {
        Outlet::<Route> {}
    }
}

#[derive(Clone, Copy)]
struct MangaListState {
    mangas: Signal<Vec<Manga>>,
    has_more: Signal<bool>,
    page: Signal<u32>,
}

#[component]
pub fn MangaList(extension_id: String) -> Element {
    let state = use_state();
    let extensions = state.extensions;
    let extension = extensions.get(&extension_id).unwrap();

    let name = extension.source().name.clone();
    let icon_path = extension.icon_path();

    let self_state = use_context::<MangaListState>();

    let mut mangas = self_state.mangas;
    let mut has_more = self_state.has_more;
    let mut page = self_state.page;

    let load_more = move || {
        let extension = extension.clone();
        async move {
            let _page = *page.read();
            let Ok((mut next_mangas, next_has_more)) =
                extension.get_manga_list(vec![], _page).await
            else {
                return;
            };
            mangas.write().append(&mut next_mangas);
            has_more.set(next_has_more);
            page.set(_page + 1);
        }
    };

    use_future(load_more);

    rsx! {
        div {
            GoBackButton {
                Icon { style: "color: inherit", icon: LdArrowLeft }
            }
            h2 { "{name}" }
        }
        ul {
            {
                mangas
                    .read()
                    .iter()
                    .map(|manga| {
                        let title = &manga.title;
                        let manga_id = manga.id.clone();
                        rsx! {
                            li {
                                Link {
                                    to: Route::ChapterList {
                                        extension_id: extension_id.clone(),
                                        manga_id,
                                    },
                                    "{title}"
                                }
                            }
                        }
                    })
            }
        }
    }
}
