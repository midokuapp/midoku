<script>
	import { AspectRatio } from 'bits-ui';

	/** @type {Array<{ id: number, title: string, cover_src: string, unread_chapters: number }>} */
	let mangaList = [];

	// Generate some dummy data
	for (let i = 0; i < 20; i++) {
		mangaList.push({
			id: i,
			title: `Manga Donec eu finibus dui, vitae vulputate lorem. Sed et vestibulum nulla, quis pellentesque massa.`,
			cover_src: 'https://via.placeholder.com/150',
			unread_chapters: Math.floor(Math.random() * 100) * (i % 2)
		});
	}
</script>

<div>
	{#each mangaList as manga}
		<a href="/">
			<AspectRatio.Root ratio={3 / 4}>
				<img src={manga.cover_src} alt="{manga.title} cover" />
			</AspectRatio.Root>
			<h4>{manga.title}</h4>
			{#if manga.unread_chapters > 0}
				<span>
					{manga.unread_chapters}
				</span>
			{/if}
		</a>
	{/each}
</div>

<style>
	@import 'open-props/media';

	div {
		padding: var(--size-2);

		display: grid;
		gap: var(--size-2);
		grid-template-columns: repeat(2, minmax(0, 1fr));

		@media (--sm-n-above) {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}

		@media (--md-n-above) {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		@media (--lg-n-above) {
			grid-template-columns: repeat(5, minmax(0, 1fr));
		}

		@media (--xl-n-above) {
			grid-template-columns: repeat(6, minmax(0, 1fr));
		}

		@media (--xxl-n-above) {
			grid-template-columns: repeat(7, minmax(0, 1fr));
		}
	}

	a {
		position: relative;
		padding: var(--size-1);
		color: inherit;
		text-decoration: none;
		padding: var(--size-1);
	}

	img {
		width: 100%;
		height: 100%;

		border-radius: var(--radius-3);
		object-fit: cover;
	}

	h4 {
		font-size: var(--font-size-3);
		font-weight: var(--font-weight-7);

		/* Clamp the title to 2 lines */
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	span {
		position: absolute;
		top: 0;
		left: 0;

		font-size: var(--font-size-2);
		font-weight: var(--font-weight-6);

		color: var(--gray-0);
		background-color: var(--gray-7);

		padding: var(--size-1) var(--size-2);

		border-radius: var(--radius-2);
	}
</style>
