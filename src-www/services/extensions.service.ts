import { convertFileSrc, invoke } from "@tauri-apps/api/core";

import { Chapter } from "../types/chapter";
import { Filter } from "../types/filter";
import { Manga, MangaList } from "../types/manga";
import { Page } from "../types/page";
import { getExtensions } from "./tauri.service";

export function getIconUrl(iconPath: string): string {
  return convertFileSrc(iconPath);
}

// todo : use memo to avoid re-rendering
export async function getExtension(extensionId: string) {
  return await getExtensions().then((extensions) =>
    extensions.find((extension) => extension.id === extensionId)
  );
}

export async function getMangaList(
  extensionId: string,
  filters: Filter[],
  page: number,
): Promise<MangaList> {
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
