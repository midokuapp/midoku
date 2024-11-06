import { invoke } from "@tauri-apps/api/core";

import { Extension, Source } from "./types/extension.ts";
import { Manifest } from "./types/manifest.ts";

export async function getExtensions(): Promise<Extension[]> {
  return (await invoke<[string, Source, string][]>("get_extensions"))
    .map(([id, source, iconPath]) => new Extension(id, source, iconPath));
}

export async function getRepositoryExtensions(
  repositoryUrl: string,
): Promise<Manifest[]> {
  return (await invoke<Manifest[]>("get_repository_extensions", {
    repositoryUrl: repositoryUrl,
  }));
}
