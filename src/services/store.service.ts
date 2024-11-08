import { Store } from "@tauri-apps/plugin-store";

export const storeService = await Store.load("app_data.json");
