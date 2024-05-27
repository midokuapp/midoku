<script>
    import { Compass, Ellipsis, Library } from "lucide-svelte";

    import NavBar from "$lib/components/NavBar.svelte";
    import NavElem from "$lib/components/NavElem.svelte";
</script>

<NavBar>
    <NavElem href="/">
        <Library slot="icon" />
        <span slot="text">Library</span>
    </NavElem>
    <NavElem href="/browse">
        <Compass slot="icon" />
        <span slot="text">Browse</span>
    </NavElem>
    <NavElem href="/more">
        <Ellipsis slot="icon" />
        <span slot="text">More</span>
    </NavElem>
</NavBar>
