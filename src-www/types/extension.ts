import { convertFileSrc, invoke } from "@tauri-apps/api/core";

import { Chapter } from "./chapter.ts";
import { Filter } from "./filter.ts";
import { Manga } from "./manga.ts";
import { Page } from "./page.ts";

export interface Source {
  name: string;
  language: string;
  version: string;
  url: string;
  nsfw: boolean;
}

export class Extension {
  public name: string;
  public language: string;
  public version: string;
  public url: string;
  public nsfw: boolean;
  public iconUrl: string;

  constructor(
    public id: string,
    source: Source,
    iconPath: string,
  ) {
    this.name = source.name;
    this.language = source.language;
    this.version = source.version;
    this.url = source.url;
    this.nsfw = source.nsfw;
    this.iconUrl = convertFileSrc(iconPath);
  }

  async getMangaList(
    filters: Array<Filter>,
    page: number,
  ): Promise<[Array<Manga>, boolean]> {
    return await invoke("get_manga_list", {
      extensionId: this.id,
      filters: filters,
      page: page,
    });
  }

  async getMangaDetails(mangaId: string): Promise<Manga> {
    return await invoke("get_manga_details", {
      extensionId: this.id,
      mangaId: mangaId,
    });
  }

  async getChapterList(mangaId: string): Promise<Array<Chapter>> {
    return await invoke("get_chapter_list", {
      extensionId: this.id,
      mangaId: mangaId,
    });
  }

  async getPageList(mangaId: string, chapterId: string): Promise<Array<Page>> {
    return await invoke("get_page_list", {
      extensionId: this.id,
      mangaId: mangaId,
      chapterId: chapterId,
    });
  }
}
