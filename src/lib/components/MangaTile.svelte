<script>
    import { AspectRatio } from 'bits-ui';

    /** @type {{ title: string, cover_src: string, unread_chapters: number }} */
    export let manga;
</script>

<a href="/">
    <AspectRatio.Root ratio={3 / 4}>
        <img src={manga.cover_src} alt="{manga.title} cover" />
    </AspectRatio.Root>
    <h4>{manga.title}</h4>
    {#if manga.unread_chapters > 0}
        <span>
            {manga.unread_chapters}
        </span>
    {/if}
</a>

<style>
    a {
        position: relative;
        color: var(--text-1);
        text-decoration: none;
    }

    img {
        width: 100%;
        height: 100%;

        border-radius: var(--radius-3);
        object-fit: cover;
    }

    h4 {
        font-size: var(--font-size-2);
        font-weight: var(--font-weight-7);

        /* Clamp the title to 2 lines */
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
        overflow: hidden;
    }

    span {
        position: absolute;
        top: var(--size-00);
        left: var(--size-00);

        font-size: var(--font-size-2);
        font-weight: var(--font-weight-6);

        color: var(--surface-1);
        background-color: var(--brand);

        padding: var(--size-1) var(--size-2);

        border-radius: var(--radius-2);
    }
</style>
