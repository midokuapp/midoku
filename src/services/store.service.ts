import { Store } from "@tauri-apps/plugin-store";
import { create } from "zustand";
import { Extension } from "../types/extension.ts";
import { Manifest } from "../types/manifest.ts";

const appData = await Store.load("app_data.json");

// Load the repository URL from the app data store
const repositoryUrl = await appData.get<string>("extensionRepositoryUrl") ?? "";

interface StoreState {
  extensions: Extension[];
  setExtensions: (extensions: Extension[]) => void;
  getExtension: (extensionId: string) => Extension | undefined;

  repositoryUrl: string;
  setRepositoryUrl: (repositoryUrl: string) => void;

  manifests: Manifest[];
  setManifests: (manifests: Manifest[]) => void;
}

export const useStore = create<StoreState>()((set, get) => ({
  extensions: new Array<Extension>(),
  setExtensions: (extensions: Extension[]) => set({ extensions }),
  getExtension: (extensionId: string) => {
    const extensions = get().extensions;
    return extensions.find((extension) => extension.id === extensionId);
  },

  repositoryUrl: repositoryUrl,
  setRepositoryUrl: (repositoryUrl: string) => {
    set({ repositoryUrl });
    appData.set("extensionRepositoryUrl", repositoryUrl);
  },

  manifests: new Array<Manifest>(),
  setManifests: (manifests: Manifest[]) => set({ manifests }),
}));
