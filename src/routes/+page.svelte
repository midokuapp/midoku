<script>
    import { Search, RotateCcw, ArrowLeft, ListFilter } from "lucide-svelte";
    import { toast } from "svelte-sonner";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    import BottomNavBar from "$lib/components/BottomNavBar.svelte";
    import Header from "$lib/components/Header.svelte";
    import MangaGrid from "$lib/components/MangaGrid.svelte";
    import MangaTile from "$lib/components/MangaTile.svelte";

    /**
     * @typedef {import('./+layout').Manga} Manga
     */

    /** @type {import('./$types').PageData} */
    export let data;

    /** @type {Manga[]} */
    let filteredMangaList = [];

    /** @type {boolean} */
    let searchEnabled = false;

    /** @type {string} */
    let searchQuery = "";

    $: {
        if (searchQuery === "") {
            filteredMangaList = data.mangaList;
        } else {
            filteredMangaList = data.mangaList.filter((manga) =>
                manga.title
                    .normalize()
                    .toLowerCase()
                    .includes(searchQuery.normalize().toLowerCase()),
            );
        }
    }
</script>

<Header>
    {#if !searchEnabled}
        <h1 class="mr-auto text-2xl">Library</h1>
        <button on:click={() => (searchEnabled = true)}>
            <Search />
        </button>
    {:else}
        <button on:click={() => (searchEnabled = false)}>
            <ArrowLeft />
        </button>
        <input
            type="text"
            placeholder="Search..."
            class="h-full flex-grow bg-transparent"
            bind:value={searchQuery}
            autofocus
        />
    {/if}
    <button on:click={() => toast("TODO")}>
        <RotateCcw />
    </button>
    <button on:click={() => toast("TODO")}>
        <ListFilter />
    </button>
</Header>

<ScrollArea class="flex-grow">
    <MangaGrid>
        {#each filteredMangaList as manga}
            <MangaTile {manga} />
        {/each}
    </MangaGrid>
</ScrollArea>

<BottomNavBar active="library" />
