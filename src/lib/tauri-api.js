import { invoke } from "@tauri-apps/api/core";

/**
 * @typedef {Object} Manga
 * @property {string} id
 * @property {string} title
 * @property {string} cover_src
 * @property {number} unread_chapters
 */

/**
 * Get the library data from the backend
 *
 * @returns {Promise<Manga[]>}
 */
export async function getLibraryMangaList() {
    return await invoke("get_library");
}

/**
 * Set the theme of the application
 *
 * @param {"auto" | "light" | "dark"} theme
 */
export async function setTheme(theme) {
    await invoke("plugin:theme|set_theme", {
        theme: theme,
    });
}
