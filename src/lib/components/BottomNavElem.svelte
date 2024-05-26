<script>
    /** @type {string} */
    export let href = "#";
</script>

<a {href} class="flex flex-col items-center gap-1 font-semibold">
    <slot name="icon" />
    <slot name="text" />
</a>
