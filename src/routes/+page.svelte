<script>
    import { Search, RotateCcw, ArrowLeft, ListFilter } from "lucide-svelte";
    import { toast } from "svelte-sonner";

    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";

    import BottomNavBar from "$lib/components/BottomNavBar.svelte";
    import Header from "$lib/components/Header.svelte";
    import MangaGrid from "$lib/components/MangaGrid.svelte";
    import MangaTile from "$lib/components/MangaTile.svelte";

    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    /** @type {Array<{ id: string, title: string, cover_src: string, unread_chapters: number }>} */
    let mangaList = [];

    /** @type {Array<{ id: string, title: string, cover_src: string, unread_chapters: number }>} */
    let filteredMangaList = [];

    /** @type {boolean} */
    let searchEnabled = false;

    /** @type {string} */
    let searchQuery = "";

    /**
     * Get the library data from the backend
     *
     * @returns {Promise<Array<{ id: string, title: string, cover_src: string, unread_chapters: number }>>}
     */
    async function getLibrary() {
        return await invoke("get_library");
    }

    /**
     * Set the application theme for the given value
     *
     * @param {"auto" | "light" | "dark"} theme
     */
    async function setTheme(theme) {
        await invoke("plugin:theme|set_theme", {
            theme: theme,
        });
    }

    onMount(() => {
        getLibrary().then((data) => {
            mangaList = data;
            mangaList.sort((a, b) => a.title.localeCompare(b.title));
            filteredMangaList = mangaList;
        });
    });

    $: {
        if (searchQuery === "") {
            filteredMangaList = mangaList;
        } else {
            filteredMangaList = mangaList.filter((manga) =>
                manga.title
                    .normalize()
                    .toLowerCase()
                    .includes(searchQuery.normalize().toLowerCase()),
            );
        }
    }

    setTheme("auto");
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

<BottomNavBar />
