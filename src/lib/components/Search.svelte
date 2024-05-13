<script>
    import { Search } from 'lucide-svelte';

    /** @type {string} */
    export let placeholder = '';
</script>

<div>
    <Search />
    <input type="text" {placeholder} />
</div>

<style>
    div {
        display: flex;
        align-items: center;
        gap: var(--size-1);
        padding: var(--size-2);
        background-color: var(--surface-2);
        border-radius: var(--size-1);

        color: var(--text-1);
    }

    input {
        flex: 1;
        font-size: var(--font-size-3);
    }
</style>
