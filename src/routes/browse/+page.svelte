<script>
    import Header from "$lib/components/Header.svelte";
    import BottomNavBar from "$lib/components/BottomNavBar.svelte";
    import * as Tabs from "$lib/components/ui/tabs";
    import Source from "$lib/components/browse/source.svelte";
</script>

<Header>
    <h1 class="mr-auto text-2xl">Browse</h1>
</Header>

<Tabs.Root value="sources" class="w-full flex-grow">
    <Tabs.List class="w-full justify-around rounded-none px-3">
        <Tabs.Trigger value="sources">Sources</Tabs.Trigger>
        <Tabs.Trigger value="extensions">Extensions</Tabs.Trigger>
        <Tabs.Trigger value="migration">Migration</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="sources"><Source /></Tabs.Content>
    <Tabs.Content value="extensions">TODO</Tabs.Content>
    <Tabs.Content value="migration">TODO</Tabs.Content>
</Tabs.Root>

<BottomNavBar active="browse" />
