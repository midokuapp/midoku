<script>
    let sources = [
        {
            name: "Local source",
            logo_src: "https://picsum.photos/100/100",
            category: "Other",
        },
    ];

    /** @type {Map<string, Array<{name: string, logo_src: string, category: string}>>} */
    let sourcesMap = new Map();
    for (let source of sources) {
        if (!sourcesMap.has(source.category)) {
            sourcesMap.set(source.category, []);
        }
        sourcesMap.get(source.category)?.push(source);
    }

    /**
     * Get the sources for the given category
     *
     * @param {string} category
     * @returns {Array<{name: string, logo_src: string, category: string}>}
     */
    function getCategory(category) {
        return sourcesMap.get(category) ?? [];
    }

    console.log(sourcesMap);
</script>

<ul>
    {#each Array.from(sourcesMap.keys()) as category}
        <li class="flex flex-col gap-0">
            <h2 class="mx-3 text-muted-foreground">{category}</h2>
            <ul class="m-3 flex flex-col gap-3">
                {#each getCategory(category) as source}
                    <li>
                        <a href="#" class="flex h-14 flex-row items-center gap-3">
                            <img
                                src={source.logo_src}
                                alt={source.name}
                                class="aspect-square h-full rounded-sm object-cover"
                            />
                            <p>{source.name}</p>
                        </a>
                    </li>
                {/each}
            </ul>
        </li>
    {/each}
</ul>
