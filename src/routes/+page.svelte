<script>
    import { Compass, Ellipsis, LibraryBig, Search, RotateCcw } from "lucide-svelte";

    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";

    import BottomNavBar from "$lib/components/BottomNavBar.svelte";
    import BottomNavElem from "$lib/components/BottomNavElem.svelte";
    import Header from "$lib/components/Header.svelte";
    import MangaGrid from "$lib/components/MangaGrid.svelte";
    import MangaTile from "$lib/components/MangaTile.svelte";

    /** @type {Array<{ id: string, title: string, cover_src: string, unread_chapters: number }>} */
    let mangaList = [];

    // Generate some dummy data
    for (let i = 0; i < 20; i++) {
        mangaList.push({
            id: i.toString(),
            title: `Donec eu finibus dui, vitae vulputate lorem. Sed et vestibulum nulla, quis pellentesque massa.`,
            cover_src: "https://picsum.photos/600/800/?img=" + i,
            unread_chapters: Math.floor(Math.random() * 100) * (i % 2),
        });
    }
</script>

<Header>
    <h1 class="mr-auto text-2xl font-medium">Library</h1>
    <a href="#">
        <Search />
    </a>
    <a href="/settings">
        <RotateCcw />
    </a>
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
    <BottomNavElem href="#">
        <Compass slot="icon" />
        <span slot="text">Browse</span>
    </BottomNavElem>
    <BottomNavElem href="#">
        <Ellipsis slot="icon" />
        <span slot="text">More</span>
    </BottomNavElem>
</BottomNavBar>
