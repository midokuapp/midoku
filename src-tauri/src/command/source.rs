use log::trace;
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use tauri::State;

use crate::extension::Extensions;

macro_rules! call_extension {
    ($state:expr, $extension_id:expr, $method:ident, $($args:expr),*) => {{
        trace!("{} called with extension_id: {}", stringify!($method), $extension_id);
        $state
            .lock()
            .get($extension_id.as_str())
            .ok_or(tauri::Error::AssetNotFound(
                "extension not found".to_string(),
            ))
            .and_then(|extension| {
                extension.$method($($args),*).map_err(|e| tauri::Error::AssetNotFound(e.to_string()))
            })
    }};
}

#[tauri::command]
pub async fn get_manga_list(
    state: State<'_, Extensions>,
    extension_id: String,
    filters: Vec<Filter>,
    page: u32,
) -> tauri::Result<(Vec<Manga>, bool)> {
    call_extension!(state, extension_id, get_manga_list, filters, page)
}

#[tauri::command]
pub async fn get_manga_details(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
) -> tauri::Result<Manga> {
    call_extension!(state, extension_id, get_manga_details, manga_id)
}

#[tauri::command]
pub async fn get_chapter_list(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
) -> tauri::Result<Vec<Chapter>> {
    call_extension!(state, extension_id, get_chapter_list, manga_id)
}

#[tauri::command]
pub async fn get_page_list(
    state: State<'_, Extensions>,
    extension_id: String,
    manga_id: String,
    chapter_id: String,
) -> tauri::Result<Vec<Page>> {
    call_extension!(state, extension_id, get_page_list, manga_id, chapter_id)
}
