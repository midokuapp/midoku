export enum Status {
  Unknown,
  Ongoing,
  Completed,
  Hiatus,
  Cancelled,
}

export enum ContentRating {
  Safe,
  Suggestive,
  Nsfw,
}

export enum ReadingMode {
  RightToLeft,
  LeftToRight,
  Vertical,
  Scroll,
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

export interface MangaList {
  data: [Manga[], boolean];
}
