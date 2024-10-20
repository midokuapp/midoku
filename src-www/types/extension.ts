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
  private _id: string;
  private _source: Source;
  private _iconPath: string;

  constructor(id: string, source: Source, iconPath: string) {
    this._id = id;
    this._source = source;
    this._iconPath = iconPath;
  }

  get id(): string {
    return this._id;
  }

  get name(): string {
    return this._source.name;
  }

  get language(): string {
    return this._source.language;
  }

  get version(): string {
    return this._source.version;
  }

  get url(): string {
    return this._source.url;
  }

  get nsfw(): boolean {
    return this._source.nsfw;
  }

  get iconUrl(): string {
    return convertFileSrc(this._iconPath);
  }

  async getMangaList(
    filters: Array<Filter>,
    page: number,
  ): Promise<Array<Manga>> {
    return await invoke("get_manga_list", {
      extensionId: this._id,
      filters: filters,
      page: page,
    });
  }

  async getMangaDetails(mangaId: string): Promise<Manga> {
    return await invoke("get_manga_details", {
      extensionId: this._id,
      mangaId: mangaId,
    });
  }

  async getChapterList(mangaId: string): Promise<Array<Chapter>> {
    return await invoke("get_chapter_list", {
      extensionId: this._id,
      mangaId: mangaId,
    });
  }

  async getPageList(mangaId: string, chapterId: string): Promise<Array<Page>> {
    return await invoke("get_page_list", {
      extensionId: this._id,
      mangaId: mangaId,
      chapterId: chapterId,
    });
  }
}
