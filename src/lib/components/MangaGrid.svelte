<script>
    /** @type {string?} */
    export let gridSize = null;
</script>

{#if gridSize === null}
    <div
        class="m-2 grid grid-cols-3 gap-1 sm:grid-cols-4 md:grid-cols-5
        lg:grid-cols-6 xl:grid-cols-7 2xl:grid-cols-8"
    >
        <slot />
    </div>
{:else}
    <div class="m-2 grid grid-cols-{gridSize} gap-1">
        <slot />
    </div>
{/if}
