<script>
    /** @type {string} */
    export let href = "#";

    /** @type {boolean} */
    export let active = false;
</script>

{#if active}
    <a {href} class="flex flex-col items-center gap-1">
        <slot name="icon" />
        <slot name="text" />
    </a>
{:else}
    <a {href} class="flex flex-col items-center gap-1 text-muted-foreground">
        <slot name="icon" />
        <slot name="text" />
    </a>
{/if}
