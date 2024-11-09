import { forwardRef, useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import useInfiniteScroll from "react-infinite-scroll-hook";

import { Extension } from "../../types/extension.ts";
import { Manga } from "../../types/manga.ts";
import { getMangaList } from "../../services/extensions.service.ts";
import MangaImage from "../../components/Manga/MangaImage.tsx";
import { useStore } from "../../services/store.service.ts";

export default function ExtensionBrowse() {
  const { extensionId } = useParams();
  const getExtension = useStore((state) => state.getExtension);

  const [extension, setExtension] = useState<Extension | null>(null);
  const [mangas, setMangas] = useState<Array<Manga>>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [hasMore, setHasMore] = useState<boolean>(true);
  const [page, setPage] = useState<number>(0);

  useEffect(() => {
    if (!extensionId) return;

    const extension = getExtension(extensionId)!;
    setExtension(extension);
  }, [extensionId]);

  const loadMore = async () => {
    if (!extension) return;
    setLoading(true);
    const [nextMangas, nextHasMore] = await getMangaList(
      extension.id,
      [],
      page,
    );
    setMangas([...mangas, ...nextMangas]);
    setHasMore(nextHasMore);
    setPage(page + 1);
    setLoading(false);
  };

  const [sentryRef] = useInfiniteScroll({
    loading,
    hasNextPage: hasMore,
    onLoadMore: loadMore,
  });

  if (!extension) return <Loader />;

  return (
    <div className="px-2 overflow-auto">
      <ExtensionHeader extension={extension} />
      <Grid>
        {mangas.map((manga: Manga) => (
          <GridItem key={manga.id}>
            <MangaItem manga={manga} extensionId={extension.id} />
          </GridItem>
        ))}
        {(loading || hasMore) && (
          <GridItem
            ref={sentryRef}
            className="col-span-full flex flex-col items-center justify-center"
          >
            <Loader />
          </GridItem>
        )}
      </Grid>
    </div>
  );
}

const Loader = () => (
  <div className="flex flex-col items-center justify-center">
    <div className="loading loading-dots"></div>
  </div>
);

const ExtensionHeader = ({ extension }: { extension: Extension }) => (
  <div className="flex items-center gap-3 mb-5">
    <img
      src={extension.iconUrl}
      alt={extension.source.name}
      className="w-12 h-12"
    />
    <h2 className="text-2xl">{extension.source.name}</h2>
  </div>
);

const Grid = ({ children }: { children: React.ReactNode }) => (
  <ul className="grid grid-cols-[repeat(auto-fill,minmax(100px,5fr))] gap-4 list-none p-0">
    {children}
  </ul>
);

type GridItemProps = {
  children: React.ReactNode;
  className?: string;
};

const GridItem = forwardRef<React.ComponentRef<"li">, GridItemProps>(
  function GridItem(props: GridItemProps, ref: React.LegacyRef<HTMLLIElement>) {
    return (
      <li ref={ref} className={props.className}>
        {props.children}
      </li>
    );
  },
);

const MangaItem = (
  { manga, extensionId }: { manga: Manga; extensionId: string },
) => (
  <>
    <Link
      to={{ pathname: `/browse/${extensionId}/${manga.id}` }}
      state={manga}
    >
      <MangaImage src={manga.coverUrl} alt={manga.title} />
    </Link>
    <a
      href={manga.url}
      target="_blank"
      rel="noopener noreferrer"
    >
      <p className="mt-2 text-sm font-bold overflow-hidden overflow-ellipsis whitespace-nowrap">
        {manga.title}
      </p>
    </a>
  </>
);
