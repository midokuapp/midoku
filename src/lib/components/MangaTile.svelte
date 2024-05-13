<script>
    import { AspectRatio } from 'bits-ui';

    /** @type {{ id: string, title: string, cover_src: string, unread_chapters: number }} */
    export let manga;
</script>

<a href="/manga?id={encodeURIComponent(manga.id)}">
    <div class="manga-cover">
        <AspectRatio.Root ratio={3 / 4}>
            <img src={manga.cover_src} alt="{manga.title} cover" loading="lazy" />
        </AspectRatio.Root>
    </div>
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

    .manga-cover {
        position: relative;

        &::after {
            position: absolute;
            content: '';
            top: 0;
            left: 0;
            height: 100%;
            width: 100%;
            background: linear-gradient(to bottom, transparent 75%, var(--text-1) 100%);
            border-radius: var(--radius-2);
        }
    }

    img {
        width: 100%;
        height: 100%;

        border-radius: var(--radius-2);
        object-fit: cover;
    }

    h4 {
        position: absolute;
        bottom: var(--size-00);
        margin: var(--size-2);

        color: var(--surface-1);
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
