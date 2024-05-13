<script>
    import MangaGrid from '$lib/components/MangaGrid.svelte';
    import MangaTile from '$lib/components/MangaTile.svelte';
    import Search from '$lib/components/Search.svelte';
    import { Settings } from 'lucide-svelte';

    /** @type {Array<{ id: number, title: string, cover_src: string, unread_chapters: number }>} */
    let mangaList = [];

    // Generate some dummy data
    for (let i = 0; i < 20; i++) {
        mangaList.push({
            id: i,
            title: `Donec eu finibus dui, vitae vulputate lorem. Sed et vestibulum nulla, quis pellentesque massa.`,
            cover_src: 'https://source.unsplash.com/random/300x400/?img=' + i,
            unread_chapters: Math.floor(Math.random() * 100) * (i % 2)
        });
    }
</script>

<div>
    <section>
        <a href="/settings">
            <Settings />
        </a>
    </section>
    <h1>Library</h1>
    <Search placeholder="Search for manga..." />
    <MangaGrid>
        {#each mangaList as manga}
            <MangaTile {manga} />
        {/each}
    </MangaGrid>
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        gap: var(--size-3);
        padding: var(--size-3);
    }

    h1 {
        font-size: var(--font-size-7);
    }

    section {
        display: flex;
        justify-content: flex-end;
    }
</style>
