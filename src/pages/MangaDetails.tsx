import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import { Manga, ReadingMode } from "../types/manga.ts";
import { getMangaDetails } from "../services/extensions.service.ts";

import MangaChapters from "../components/Manga/MangaChapters.tsx";

export default function MangaDetails() {
  const { extensionId, mangaId } = useParams<
    { extensionId: string; mangaId: string }
  >();

  const [loading, setLoading] = useState<boolean>(true);
  const [manga, setManga] = useState<Manga | null>(null);
  const [descriptionExpanded, setDescriptionExpanded] = useState<boolean>(
    false,
  );

  useEffect(() => {
    if (!extensionId || !mangaId) return;

    getMangaDetails(extensionId, mangaId)
      .then((data) => {
        setManga(data);
        setLoading(false);
      });
  }, [extensionId, mangaId]);

  if (loading) {
    return (
      <div className="flex flex-col justify-center items-center h-screen">
        <div>Loading...</div>
        <div className="loader"></div>
      </div>
    );
  }

  if (!manga) {
    return null;
  }

  return (
    <div className="max-w-md mx-auto">
      <div className="p-2">
        {/* Manga title and cover */}
        <div className="flex gap-4 items-start">
          <img
            src={manga.coverUrl}
            alt={manga.title}
            className="aspect-[2/3] w-36 rounded-md object-cover"
          />
          <div className="flex flex-col">
            <h1 className="text-lg font-semibold mb-1">{manga.title}</h1>
            <div className="flex gap-1 flex-col">
              <p className="text-sm opacity-70">
                <strong>Author:</strong> {manga.authorName}
              </p>
              <p className="text-sm opacity-70">
                <strong>Status:</strong> {manga.status}
              </p>
              <p className="text-sm opacity-70">
                <strong>Content Rating:</strong> {manga.contentRating}
              </p>
              <p className="text-sm opacity-70">
                <strong>Reading Style:</strong> {(() => {
                  switch (manga.readingMode) {
                    case ReadingMode.RightToLeft:
                      return "Right to Left";
                    case ReadingMode.LeftToRight:
                      return "Left to Right";
                    case ReadingMode.Vertical:
                      return "Vertical";
                    case ReadingMode.Scroll:
                      return "Scroll";
                    default:
                      return "Unknown";
                  }
                })()}
              </p>
              <p className="text-sm opacity-70">
                {manga.categories.map((category: string) => (
                  <span key={category} className="badge badge-outline mr-1">
                    {category}
                  </span>
                ))}
              </p>
            </div>
          </div>
        </div>

        {/* Collapsible description */}
        <div className="mt-4">
          <h2 className="text-md font-semibold mb-2">Description</h2>
          <p
            className={`text-sm transition ${
              !descriptionExpanded && "line-clamp-3"
            }`}
          >
            {manga.description}
          </p>
          <button
            onClick={() => setDescriptionExpanded(!descriptionExpanded)}
            className="text-primary text-sm mt-2"
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
