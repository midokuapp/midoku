<script>
    import { Compass, Ellipsis, LibraryBig, Search, RotateCcw } from "lucide-svelte";

    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";

    import BottomNavBar from "$lib/components/BottomNavBar.svelte";
    import BottomNavElem from "$lib/components/BottomNavElem.svelte";
    import Header from "$lib/components/Header.svelte";
    import MangaGrid from "$lib/components/MangaGrid.svelte";
    import MangaTile from "$lib/components/MangaTile.svelte";

    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    /**
     * Get the library data from the backend
     *
     * @returns {Promise<Array<{ id: string, title: string, cover_src: string, unread_chapters: number }>>}
     */
    async function getLibrary() {
        return await invoke("get_library");
    }

    /** @type {Array<{ id: string, title: string, cover_src: string, unread_chapters: number }>} */
    let mangaList = [];

    onMount(() => {
        getLibrary().then((data) => {
            mangaList = data;
        });
    });
</script>

<Header>
    <h1 class="mr-auto text-2xl font-medium">Library</h1>
    <button>
        <Search />
    </button>
    <button>
        <RotateCcw />
    </button>
</Header>

<ScrollArea class="flex-grow">
    <MangaGrid>
        {#each mangaList as manga}
            <MangaTile {manga} />
        {/each}
    </MangaGrid>
</ScrollArea>

<BottomNavBar>
    <BottomNavElem href="/">
        <LibraryBig slot="icon" />
        <span slot="text">Library</span>
    </BottomNavElem>
    <BottomNavElem href="/browse">
        <Compass slot="icon" />
        <span slot="text">Browse</span>
    </BottomNavElem>
    <BottomNavElem href="/more">
        <Ellipsis slot="icon" />
        <span slot="text">More</span>
    </BottomNavElem>
</BottomNavBar>
