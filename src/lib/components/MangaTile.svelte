<script>
    import { AspectRatio } from "$lib/components/ui/aspect-ratio";

    /** @type {{ id: string, title: string, cover_src: string, unread_chapters: number }} */
    export let manga;
</script>

<a href="/manga?id={encodeURIComponent(manga.id)}" class="relative">
    <AspectRatio
        ratio={3 / 4}
        class="m-1 after:absolute after:left-0 after:top-0 after:h-full after:w-full
        after:rounded-md after:bg-gradient-to-b after:from-transparent after:from-60%
        after:to-black/80 after:content-['']"
    >
        <img
            src={manga.cover_src}
            alt="{manga.title} cover"
            loading="lazy"
            class="rounded-md object-cover"
        />
    </AspectRatio>
    <h1 class="absolute bottom-0 left-0 m-3 line-clamp-2 font-semibold text-white">
        {manga.title}
    </h1>
    {#if manga.unread_chapters > 0}
        <span
            class="absolute left-0 top-0 rounded-md bg-[hsl(var(--primary))] px-2 py-1 font-semibold
            text-[hsl(var(--primary-foreground))]"
        >
            {manga.unread_chapters}
        </span>
    {/if}
</a>
