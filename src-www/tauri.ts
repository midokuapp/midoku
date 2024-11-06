import { invoke } from "@tauri-apps/api/core";

import { Extension, Source } from "./types/extension.ts";

export async function getExtensions(): Promise<Extension[]> {
  return (await invoke<[string, Source, string][]>("get_extensions")).map((
    [id, source, iconPath],
  ) => new Extension(id, source, iconPath));
}
