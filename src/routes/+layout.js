import { invoke } from "@tauri-apps/api/core";

export const prerender = true;
export const ssr = false;

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
async function getLibrary() {
    return await invoke("get_library");
}

/** @type {import('./$types').LayoutLoad} */
export async function load() {
    let mangaList = await getLibrary();
    mangaList.sort((a, b) => a.title.localeCompare(b.title));
    return {
        mangaList: mangaList,
    };
}
