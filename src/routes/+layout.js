import { getLibraryMangaList } from "$lib/tauri-api";

export const prerender = true;
export const ssr = false;

/**
 * @typedef {import('$lib/tauri-api').Manga} Manga
 */

/** @type {import('./$types').LayoutLoad} */
export async function load() {
    let libraryMangaList = await getLibraryMangaList();
    libraryMangaList.sort((a, b) => a.title.localeCompare(b.title));
    return {
        libraryMangaList: libraryMangaList,
    };
}
