<script>
    import { Compass, Ellipsis, Library } from "lucide-svelte";

    import NavBar from "$lib/components/NavBar.svelte";
    import NavElem from "$lib/components/NavElem.svelte";

    /** @type {"library" | "browse" | "more" | null} */
    export let active = null;

    const navElems = [
        { href: "/", icon: Library, text: "Library" },
        { href: "/browse", icon: Compass, text: "Browse" },
        { href: "/more", icon: Ellipsis, text: "More" },
    ];
</script>

<NavBar>
    {#each navElems as { href, icon, text }, i}
        <NavElem {href} active={active === text.toLowerCase()}>
            <svelte:component this={icon} slot="icon" />
            <span slot="text">{text}</span>
        </NavElem>
    {/each}
</NavBar>
