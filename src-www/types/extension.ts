export interface Source {
  name: string;
  language: string;
  version: string;
  url: string;
  nsfw: boolean;
}

export interface Extension {
  id: string;
  iconPath: string;
  source: Source;
}
