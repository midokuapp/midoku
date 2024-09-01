use deunicode::deunicode;
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::gallery::{Grid, Tile};
use crate::header::Header;
use crate::navigation::NavigationBar;

#[derive(Serialize, Deserialize, Clone)]
struct Manga {
    pub id: String,
    pub title: String,
    pub cover_src: String,
    pub unread_chapters: u32,
}

async fn get_library() -> Vec<Manga> {
    crate::invoke!("get_library")
}

fn filter_entry(search: ReadSignal<String>) -> impl Fn(&Manga) -> bool {
    let search_value = deunicode(&search.get()).to_lowercase();

    move |entry: &Manga| {
        let entry_title = deunicode(&entry.title).to_lowercase();

        entry_title.contains(&search_value)
    }
}

fn map_entry(entry: Manga) -> View {
    view! {
        <Tile
            id=entry.id
            title=entry.title
            cover_src=entry.cover_src
            unread_chapters=entry.unread_chapters
        />
    }
}

#[component]
pub fn Library() -> impl IntoView {
    let (search, set_search) = create_signal("".to_string());

    let library = create_resource(|| (), |_| async move { get_library().await });
    let tiles = move || {
        library
            .get()
            .unwrap_or_default()
            .drain(..)
            .filter(filter_entry(search))
            .map(map_entry)
            .collect::<Vec<View>>()
    };

    view! {
        <div class="flex flex-col w-screen h-screen">
            <Header>
                <h1 class="mr-auto text-2xl">Library</h1>
                <input
                    type="text"
                    placeholder="Search..."
                    class="h-full flex-grow bg-transparent"
                    on:input=move |ev| set_search.set(event_target_value(&ev))
                    prop:value=search
                />
            </Header>
            <main class="overflow-y-scroll grow">
                <Grid>{tiles}</Grid>
            </main>
            <NavigationBar/>
        </div>
    }
}
