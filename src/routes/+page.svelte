<script>
	import { AspectRatio } from 'bits-ui';

	/** @type {Array<{ id: number, title: string, cover_src: string, unread_chapters: number }>} */
	let mangaList = [];

	// Generate some dummy data
	for (let i = 0; i < 10; i++) {
		mangaList.push({
			id: i,
			title: `Manga Donec eu finibus dui, vitae vulputate lorem. Sed et vestibulum nulla, quis pellentesque massa.`,
			cover_src: 'https://via.placeholder.com/150',
			unread_chapters: Math.floor(Math.random() * 100) * (i % 2)
		});
	}
</script>

<div
	class="grid grid-cols-3 gap-2 p-2 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 xl:grid-cols-7 2xl:grid-cols-8"
>
	{#each mangaList as manga}
		<a class="relative p-1" href="/manga/{manga.id}">
			<AspectRatio.Root ratio={3 / 4}>
				<img
					src={manga.cover_src}
					alt="{manga.title} cover"
					class="h-full w-full rounded-lg object-cover"
				/>
			</AspectRatio.Root>
			<h4 class="text-lg font-semibold line-clamp-2">{manga.title}</h4>
			{#if manga.unread_chapters > 0}
				<span
					class="absolute top-0 left-0 px-2 py-1 text-md font-semibold text-white bg-gray-500 rounded-lg"
				>
					{manga.unread_chapters}
				</span>
			{/if}
		</a>
	{/each}
</div>
