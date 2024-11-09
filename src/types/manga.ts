export enum Status {
  Unknown = "Unknown",
  Ongoing = "Ongoing",
  Completed = "Completed",
  Hiatus = "Hiatus",
  Cancelled = "Cancelled",
}

export enum ContentRating {
  Safe = "Safe",
  Suggestive = "Suggestive",
  Nsfw = "Nsfw",
}

export enum ReadingMode {
  RightToLeft = "RightToLeft",
  LeftToRight = "LeftToRight",
  Vertical = "Vertical",
  Scroll = "Scroll",
}

export interface Manga {
  id: string;
  title: string;
  url: string;
  description: string;
  coverUrl: string;
  authorName: string;
  artistName: string;
  categories: string[];
  status: Status;
  contentRating: ContentRating;
  readingMode: ReadingMode;
}

export type MangaList = [Manga[], boolean];
