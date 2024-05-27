<script>
    import { Toaster } from "$lib/components/ui/sonner";
    import { ModeWatcher } from "mode-watcher";
    import { invoke } from "@tauri-apps/api/core";

    import "../app.css";

    // Set the application theme to auto
    invoke("plugin:theme|set_theme", {
        theme: "auto",
    });
</script>

<ModeWatcher />
<Toaster />

<div class="flex h-screen w-screen flex-col">
    <slot />
</div>
