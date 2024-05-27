// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lipsum::lipsum_words_with_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Manga {
    pub id: String,
    pub title: String,
    pub cover_src: String,
    pub unread_chapters: u32,
}

#[tauri::command]
fn get_library() -> Vec<Manga> {
    let mut rng = rand::thread_rng();
    let mut library = Vec::new();
    for i in 0..20 {
        let manga = Manga {
            id: i.to_string(),
            title: lipsum_words_with_rng(&mut rng, 7),
            cover_src: format!("https://picsum.photos/600/800/?img={}", i),
            unread_chapters: if i % 2 == 0 { 0 } else { i },
        };
        library.push(manga);
    }
    library
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_library])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
