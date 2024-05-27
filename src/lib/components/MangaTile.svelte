<script>
    import { AspectRatio } from "$lib/components/ui/aspect-ratio";

    /** @type {{ id: string, title: string, cover_src: string, unread_chapters: number }} */
    export let manga;
</script>

<a href="/manga?id={encodeURIComponent(manga.id)}" class="relative">
    <div class="mx-1 mt-1">
        <AspectRatio ratio={3 / 4}>
            <img
                src={manga.cover_src}
                alt="{manga.title} cover"
                loading="lazy"
                class="rounded-md object-cover"
            />
        </AspectRatio>
    </div>
    <h1 class="mx-1 line-clamp-2 h-10 text-sm font-semibold text-foreground">
        {manga.title}
    </h1>
    {#if manga.unread_chapters > 0}
        <span
            class="absolute left-0 top-0 rounded-md bg-primary px-2 py-1 font-semibold
            text-primary-foreground"
        >
            {manga.unread_chapters}
        </span>
    {/if}
</a>
