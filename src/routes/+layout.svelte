<script>
    import { Toaster } from "$lib/components/ui/sonner";
    import { ModeWatcher } from "mode-watcher";

    import "../app.css";
</script>

<ModeWatcher />
<Toaster />

<div class="flex h-screen w-screen flex-col">
    <slot />
</div>
