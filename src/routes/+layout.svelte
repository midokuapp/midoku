<script>
    import { Toaster } from "$lib/components/ui/sonner";
    import { ModeWatcher } from "mode-watcher";
    import { setTheme } from "$lib/tauri-api";

    import "../app.css";

    setTheme("auto");
</script>

<ModeWatcher />
<Toaster />

<div class="flex h-screen w-screen flex-col">
    <slot />
</div>
