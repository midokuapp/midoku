import { forwardRef, useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";

import { Extension } from "../../types/extension.ts";
import { Manga } from "../../types/manga.ts";
import { getMangaList } from "../../services/extensions.service.ts";
import { useStore } from "../../services/store.service.ts";
import useInfiniteScroll from "../../utils/infinite-scroll-hook.ts";
import LazyImage from "../../components/LazyImage.tsx";

export default function ExtensionBrowse() {
  const { extensionId } = useParams();
  const getExtension = useStore((state) => state.getExtension);

  const [extension, setExtension] = useState<Extension | null>(null);
  const [mangas, setMangas] = useState<Array<Manga>>([]);
  const [hasMore, setHasMore] = useState<boolean>(true);
  const [page, setPage] = useState<number>(0);

  useEffect(() => {
    if (!extensionId) return;

    const extension = getExtension(extensionId)!;
    setExtension(extension);
  }, [extensionId]);

  const loadMore = async () => {
    if (!extension) return;
    const [nextMangas, nextHasMore] = await getMangaList(
      extension.id,
      [],
      page,
    );
    setMangas([...mangas, ...nextMangas]);
    setHasMore(nextHasMore);
    setPage(page + 1);
  };

  const { containerRef, loading } = useInfiniteScroll({
    hasMore: hasMore,
    onLoadMore: loadMore,
    offset: "50vh",
  });

  if (!extension) return <Loader />;

  return (
    <div className="px-1">
      <ExtensionHeader extension={extension} />
      <Grid ref={containerRef}>
        {mangas.map((manga: Manga) => (
          <GridItem key={manga.id}>
            <MangaItem manga={manga} extensionId={extension.id} />
          </GridItem>
        ))}
        {(loading || hasMore) && (
          <GridItem className="col-span-full flex flex-col items-center justify-center">
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

type GridProps = {
  children: React.ReactNode;
};

const Grid = forwardRef<React.ComponentRef<"ul">, GridProps>(
  function Grid(props: GridProps, ref: React.LegacyRef<HTMLUListElement>) {
    return (
      <ul
        ref={ref}
        className="grid grid-cols-[repeat(auto-fill,minmax(100px,5fr))] gap-3"
      >
        {props.children}
      </ul>
    );
  },
);

const GridItem = (
  { children, key, className }: {
    children: React.ReactNode;
    key?: string;
    className?: string;
  },
) => (
  <li key={key} className={className}>
    {children}
  </li>
);

const MangaItem = (
  { manga, extensionId }: { manga: Manga; extensionId: string },
) => {
  const [loading, setLoading] = useState<boolean>(true);

  const platform = (globalThis as Record<string, unknown>).TAURI_ENV_PLATFORM;
  let uri: string;

  if (platform === "windows" || platform === "android") {
    uri = "http://gallery.localhost";
  } else {
    uri = "gallery://localhost";
  }

  return (
    <Link
      to={{ pathname: `/browse/${extensionId}/${manga.id}` }}
      state={manga}
    >
      <div className="w-full aspect-[2/3] skeleton rounded-md">
        <LazyImage
          src={`${uri}/?url=${
            encodeURIComponent(manga.coverUrl)
          }&width=300&height=450`}
          alt={manga.title}
          onLoad={() => setLoading(false)}
          onChange={(inView) => {
            if (inView) setLoading(true);
          }}
          className={`w-full h-full object-cover rounded-md transition-opacity duration-300 ${
            loading ? "opacity-0" : "opacity-1"
          }`}
          offset="200vh"
        />
      </div>
      <p className="mx-1 mt-1 line-clamp-2 text-sm font-bold">
        {manga.title}
      </p>
    </Link>
  );
};
