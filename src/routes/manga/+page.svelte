<script>
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { ArrowLeft } from 'lucide-svelte';

    import Header from '$lib/components/Header.svelte';

    let manga_id = decodeURIComponent($page.url.searchParams.get('id') || '');

    onMount(() => {
        console.log(manga_id);
    });
</script>

<Header>
    <a href="/">
        <ArrowLeft />
    </a>
</Header>

<h1>Manga {manga_id}</h1>
