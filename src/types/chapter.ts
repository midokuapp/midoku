export interface Chapter {
  id: string;
  title: string;
  volume: number;
  chapter: number;
  // Unix timestamp in seconds
  dateUploaded: number;
  scanlator: string;
  url: string;
  language: string;
}
