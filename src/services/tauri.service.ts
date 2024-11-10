import { convertFileSrc, invoke } from "@tauri-apps/api/core";

import { Extension, Source } from "../types/extension.ts";
import { Manifest } from "../types/manifest.ts";

export async function getExtensions(): Promise<Extension[]> {
  return (await invoke<[string, Source, string][]>("get_extensions"))
    .map(([id, source, iconPath]) =>
      <Extension> {
        id: id,
        source: source,
        iconUrl: convertFileSrc(iconPath),
      }
    );
}

export async function getRepositoryExtensions(
  repositoryUrl: string,
): Promise<Manifest[]> {
  return (await invoke<Manifest[]>("get_repository_extensions", {
    repositoryUrl: repositoryUrl,
  }));
}

export async function installExtension(
  repositoryUrl: string,
  manifest: Manifest,
): Promise<void> {
  await invoke("install_extension", {
    repositoryUrl: repositoryUrl,
    manifest: manifest,
  });
}

export async function uninstallExtension(extensionId: string): Promise<void> {
  await invoke("uninstall_extension", {
    extensionId: extensionId,
  });
}

export async function downloadImage(
  url: string,
  width: number | null,
  height: number | null,
): Promise<Uint8Array> {
  return new Uint8Array(
    await invoke("download_image", {
      url: url,
      width: width,
      height: height,
    }),
  );
}
