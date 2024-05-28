import { invoke } from "@tauri-apps/api/core";

/**
 * @typedef {import('$lib/types').Manga} Manga
 */

/**
 * Get the library data from the backend
 *
 * @returns {Promise<Manga[]>}
 */
export async function getLibraryMangaList() {
    return await invoke("get_library");
}
