<script>
    import '../app.css';
</script>

<div>
    <slot />
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        gap: var(--size-3);
        padding: var(--size-3);
    }
</style>
