import { invoke } from "@tauri-apps/api/core";

/**
 * Set the theme of the application
 * 
 * @param {"auto" | "light" | "dark"} theme
 */
export async function setTheme(theme) {
    await invoke("plugin:theme|set_theme", {
        theme: theme,
    });
}
