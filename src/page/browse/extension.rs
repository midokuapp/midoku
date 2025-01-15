use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;
use midoku_bindings::exports::Manga;

use crate::state::State;

#[component]
pub fn BrowseExtension(extension_id: String) -> Element {
    let state = use_context::<State>();
    let extensions = state.extensions.read();
    let extension = use_signal(|| extensions.get(&extension_id).unwrap().clone());
    let extension_ref = extension.read();

    let source = extension_ref.source();
    let icon_path = extension_ref.icon_path();

    let mut mangas = use_signal::<Vec<Manga>>(|| vec![]);
    let mut has_more = use_signal::<bool>(|| true);
    let mut page = use_signal::<u32>(|| 0);

    let load_more = move || async move {
        let _page = *page.read();
        let Ok((mut next_mangas, next_has_more)) =
            extension.read().get_manga_list(vec![], _page).await
        else {
            return;
        };
        mangas.write().append(&mut next_mangas);
        has_more.set(next_has_more);
        page.set(_page + 1);
    };

    use_future(load_more);

    rsx! {
        div {
            GoBackButton {
                Icon { style: "color: inherit", icon: LdArrowLeft }
            }
            h2 { "{source.name}" }
        }
        ul {
            {
                mangas
                    .read()
                    .iter()
                    .map(|manga| {
                        let title = &manga.title;
                        rsx! {
                            li { "{title}" }
                        }
                    })
            }
        }
    }
}
