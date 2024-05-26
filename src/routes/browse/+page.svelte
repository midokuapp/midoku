<script>
    import { Compass, Ellipsis, LibraryBig } from "lucide-svelte";
    import Header from "$lib/components/Header.svelte";
    import BottomNavBar from "$lib/components/BottomNavBar.svelte";
    import BottomNavElem from "$lib/components/BottomNavElem.svelte";
</script>

<Header>
    <h1 class="mr-auto text-2xl font-medium">Browse</h1>
</Header>

<main class="flex-grow">

</main>

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
