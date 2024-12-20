import { invoke } from "@tauri-apps/api/core";

import { Chapter } from "../types/chapter.ts";
import { Filter } from "../types/filter.ts";
import { Manga } from "../types/manga.ts";
import { Page } from "../types/page.ts";

export async function getMangaList(
  extensionId: string,
  filters: Filter[],
  page: number,
): Promise<[Manga[], boolean]> {
  return await invoke("get_manga_list", {
    extensionId: extensionId,
    filters: filters,
    page: page,
  });
}

export async function getMangaDetails(
  extensionId: string,
  mangaId: string,
): Promise<Manga> {
  return await invoke("get_manga_details", {
    extensionId: extensionId,
    mangaId: mangaId,
  });
}

export async function getChapterList(
  extensionId: string,
  mangaId: string,
): Promise<Chapter[]> {
  return await invoke("get_chapter_list", {
    extensionId: extensionId,
    mangaId: mangaId,
  });
}

export async function getPageList(
  extensionId: string,
  mangaId: string,
  chapterId: string,
): Promise<Page[]> {
  return await invoke("get_page_list", {
    extensionId: extensionId,
    mangaId: mangaId,
    chapterId: chapterId,
  });
}
