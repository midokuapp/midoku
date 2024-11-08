import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import MangaImage from "../components/Manga/MangaImage.tsx";
import { Manga, ReadingMode } from "../types/manga.ts";
import { getMangaDetails } from "../services/extensions.service.ts";

import MangaChapters from "../components/Manga/MangaChapters.tsx";

export default function MangaDetails() {
  const { extensionId, mangaId } = useParams<
    { extensionId: string; mangaId: string }
  >();

  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [manga, setManga] = useState<Manga | null>(null);
  const [descriptionExpanded, setDescriptionExpanded] = useState<boolean>(
    false,
  );

  useEffect(() => {
    if (!extensionId || !mangaId) {
      setError("Error loading manga details.");
      setLoading(false);
      return;
    }
    getMangaDetails(extensionId, mangaId)
      .then((data) => {
        setManga(data);
        setLoading(false);
      })
      .catch((err) => {
        setError("Error loading manga details: " + err);
        setLoading(false);
      });
  }, [extensionId, mangaId]);

  if (loading) {
    return (
      <div className="flex flex-col justify-center items-center h-screen text-white">
        <div>Loading...</div>
        <div className="loader"></div>
      </div>
    );
  }

  if (error) {
    return <div className="p-4 text-red-500">{error}</div>;
  }

  if (!manga) {
    return null;
  }

  return (
    <div className="max-w-md mx-auto text-white ">
      <div className="p-2">
        {/* Manga title and cover */}
        <div className="flex gap-4 items-start">
          <MangaImage
            src={manga.coverUrl}
            alt={manga.title}
            height={150}
            width={30}
          />
          <div className="flex flex-col">
            <h1 className="text-lg font-semibold mb-1">{manga.title}</h1>
            <div className="text-gray-400 text-sm mb-1">
              <strong>Author:</strong> {manga.authorName}
            </div>
            <div className="text-gray-400 text-sm">
              <strong>Satus:</strong> {manga.status}
            </div>
            <p className="text-gray-400 text-sm">
              <strong>Content Rating:</strong> {manga.contentRating}
            </p>
          </div>
        </div>

        {/* Extra details */}
        <div className="mt-4">
          <h2 className="text-md font-semibold mb-2">Details</h2>
          <p className="text-gray-400 text-sm">
            <strong>Reading Style:</strong>{" "}
            {manga.readingMode === ReadingMode.RightToLeft
              ? "Right to Left"
              : "Left to Right"}
          </p>
          <p className="text-gray-400 text-sm">
            <strong>Categories:</strong> {manga.categories.join(", ")}
          </p>
        </div>

        {/* Collapsible description */}
        <div className="mt-4">
          <h2 className="text-md font-semibold mb-2">Description</h2>
          <p
            className={`text-gray-300 text-sm  transition ${
              descriptionExpanded ? "" : "line-clamp-3"
            }`}
          >
            {manga.description}
          </p>
          <button
            onClick={() => setDescriptionExpanded(!descriptionExpanded)}
            className="text-blue-400 text-sm mt-2"
          >
            {descriptionExpanded ? "Show less" : "Show more"}
          </button>
        </div>
      </div>

      {/* Chapter List */}
      <div className="mt-4">
        <MangaChapters extensionId={extensionId} mangaId={mangaId} />
      </div>
    </div>
  );
}
