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

#[component]
pub fn App() -> impl IntoView {
    let library = create_resource(|| (), |_| async move { get_library().await });
    let tiles = move || {
        library
            .get()
            .unwrap_or_default()
            .drain(..)
            .map(|entry| {
                view! {
                    <Tile
                        id=entry.id
                        title=entry.title
                        cover_src=entry.cover_src
                        unread_chapters=entry.unread_chapters
                    />
                }
            })
            .collect::<Vec<View>>()
    };

    view! {
        <div class="flex flex-col w-screen h-screen">
            <Header>
                <h1 class="mr-auto text-2xl">Library</h1>
            </Header>
            <Grid>{tiles}</Grid>
            <NavigationBar/>
        </div>
    }
}
