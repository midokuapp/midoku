<script>
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { ArrowLeft } from 'lucide-svelte';

    let manga_id = decodeURIComponent($page.url.searchParams.get('id') || '');

    onMount(() => {
        console.log(manga_id);
    });
</script>

<section>
    <a href="/">
        <ArrowLeft />
    </a>
</section>
<h1>Manga {manga_id}</h1>

<style>
    h1 {
        font-size: var(--font-size-7);
    }

    section {
        display: flex;
        justify-content: flex-start;
        height: var(--size-7);
    }
</style>
