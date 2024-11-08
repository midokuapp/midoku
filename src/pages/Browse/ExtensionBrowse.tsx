import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";

import { Extension } from "../../types/extension.ts";
import { Manga } from "../../types/manga.ts";
import { getIconUrl, getMangaList } from "../../services/extensions.service.ts";
import MangaImage from "../../components/Manga/MangaImage.tsx";
import { useExtensions } from "../../context/extensions.ts";

export default function ExtensionBrowse() {
  const { extensionId } = useParams();
  const { extensions } = useExtensions();

  const extension = extensions.find((extension) =>
    extension.id === extensionId
  )!;

  const [mangas, setMangas] = useState<Array<Manga>>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [pagination, setPagination] = useState<number>(0);

  useEffect(() => {
    setLoading(true);
    getMangaList(extension.id, [], pagination)
      .then((data) => setMangas([...mangas, ...data[0]]))
      .catch(() => setError("Erreur lors du chargement des mangas."))
      .finally(() => setLoading(false));
  }, [extension, pagination]);

  useEffect(() => {
    const handleScroll = () => {
      if (
        document.body.scrollHeight - 300 <
          globalThis.scrollY + globalThis.innerHeight
      ) {
        if (!loading) {
          setPagination(pagination + 1);
        }
      }
    };
    globalThis.addEventListener("scroll", handleScroll);
    return () => globalThis.removeEventListener("scroll", handleScroll);
  }, [loading]);

  if (error) return <ErrorMessage error={error} />;

  return (
    <div className="px-2">
      <ExtensionHeader extension={extension} />
      <MangaGrid mangas={mangas} extensionId={extension.id} />
      {loading && <Loader />}
    </div>
  );
}

const ErrorMessage = ({ error }: { error: string }) => (
  <div className="px-8">
    <p className="text-red-600">{error}</p>
  </div>
);

const Loader = () => (
  <div className="flex flex-col items-center justify-center">
    <div className="text-center py-5">Chargement...</div>
    <div className="border-gray-300 h-10 w-10 animate-spin rounded-full border-8 border-t-blue-600">
    </div>
  </div>
);

const ExtensionHeader = ({ extension }: { extension: Extension }) => (
  <div className="flex items-center gap-3 mb-5">
    <img
      src={getIconUrl(extension.iconPath)}
      alt={extension.source.name}
      className="w-12 h-12"
    />
    <h2 className="text-2xl">{extension.source.name}</h2>
  </div>
);

const MangaGrid = (
  { mangas, extensionId }: { mangas: Array<Manga>; extensionId: string },
) => (
  <ul className="grid grid-cols-[repeat(auto-fill,minmax(100px,5fr))] gap-4 list-none p-0">
    {mangas.map((manga) => (
      <li key={manga.id} className="text-center">
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
          className="text-gray-800 no-underline"
        >
          <p className="mt-2 text-white text-sm font-bold overflow-hidden overflow-ellipsis whitespace-nowrap">
            {manga.title}
          </p>
        </a>
      </li>
    ))}
  </ul>
);
