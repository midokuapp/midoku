<script>
    import { Search } from 'lucide-svelte';
    import { Settings } from 'lucide-svelte';

    import MangaGrid from '$lib/components/MangaGrid.svelte';
    import MangaTile from '$lib/components/MangaTile.svelte';

    /** @type {Array<{ id: string, title: string, cover_src: string, unread_chapters: number }>} */
    let mangaList = [];

    // Generate some dummy data
    for (let i = 0; i < 20; i++) {
        mangaList.push({
            id: i.toString(),
            title: `Donec eu finibus dui, vitae vulputate lorem. Sed et vestibulum nulla, quis pellentesque massa.`,
            cover_src: 'https://picsum.photos/600/800/?img=' + i,
            unread_chapters: Math.floor(Math.random() * 100) * (i % 2)
        });
    }
</script>

<header>
    <h1>Library</h1>
    <a href="#">
        <Search />
    </a>
    <a href="/settings">
        <Settings />
    </a>
</header>
<main>
    <MangaGrid>
        {#each mangaList as manga}
            <MangaTile {manga} />
        {/each}
    </MangaGrid>
</main>

<style>
    header {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: var(--size-3);
        flex-shrink: 0;

        > h1 {
            font-size: var(--size-7);
            font-weight: var(--font-weight-5);

            margin-right: auto;
        }

        > a {
            display: flex;
            align-items: center;
            justify-content: center;

            width: var(--size-7);
            height: var(--size-7);
        }
    }
</style>
