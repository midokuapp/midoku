import { forwardRef, useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { useInView } from "react-intersection-observer";

import { Extension } from "../../types/extension.ts";
import { Manga } from "../../types/manga.ts";
import { getMangaList } from "../../services/extensions.service.ts";
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
    <div className="px-1">
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
  <ul className="grid grid-cols-[repeat(auto-fill,minmax(100px,5fr))] gap-3">
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
) => {
  const { ref, inView } = useInView();
  const [src, setSrc] = useState<string | null>(null);

  const compressBlob = async (src: string, size: number) =>
    await new Promise<string>((resolve) => {
      const image = new Image();
      image.src = src;
      image.onload = () => {
        if (image.width <= size && image.height <= size) {
          resolve(src);
          return;
        }

        const aspectRatio = image.width / image.height;

        let width, height;
        if (image.width < image.height) {
          width = size;
          height = Math.round(width / aspectRatio);
        } else {
          height = size;
          width = Math.round(height * aspectRatio);
        }

        const canvas = document.createElement("canvas");
        canvas.width = width;
        canvas.height = height;

        const ctx = canvas.getContext("2d");
        ctx?.drawImage(image, 0, 0, width, height);

        const url = canvas.toDataURL("image/jpeg", 0.7);
        resolve(url);
      };
    });

  useEffect(() => {
    fetch(manga.coverUrl)
      .then((response) => response.blob())
      .then((blob) => {
        const url = URL.createObjectURL(blob);
        compressBlob(url, 300).then((url) => {
          setSrc(url);
        });
      });
  }, []);

  return (
    <>
      <Link
        ref={ref}
        to={{ pathname: `/browse/${extensionId}/${manga.id}` }}
        state={manga}
      >
        <div className="w-full aspect-[2/3]">
          {inView && src && (
            <img
              src={src}
              alt={manga.title}
              className="w-full h-full rounded-md object-cover"
            />
          )}
        </div>
        <p className="mx-1 mt-1 line-clamp-2 text-sm font-bold">
          {manga.title}
        </p>
      </Link>
    </>
  );
};
