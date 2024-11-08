import { useEffect, useState } from "react";
import { Chapter } from "../../types/chapter.ts";
import { getChapterList } from "../../services/extensions.service.ts";

function MangaChapter({ extensionId, mangaId }: {
  extensionId: string | undefined;
  mangaId: string | undefined;
}) {
  const [chapters, setChapters] = useState<Chapter[]>([]);

  useEffect(() => {
    if (!extensionId || !mangaId) return;
    getChapterList(extensionId, mangaId).then(setChapters);
  }, [extensionId, mangaId]);

  const formatDate = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  };

  return (
    <div className="mt-4">
      <h2 className="text-xl text-white font-bold mb-4">Chapters</h2>
      {chapters.length === 0 ? <p>No Chapters</p> : (
        <ul className="space-y-2">
          {chapters.map((chapter: Chapter) => (
            <li
              key={chapter.id}
              className="flex justify-between items-center p-3 bg-base-300 rounded-lg shadow-sm hover:bg-gray-200 transition"
            >
              <div className="flex flex-col">
                <span className="text-sm text-gray-600">
                  Volume {chapter.volume}, Chapter {chapter.chapter}
                </span>
                <span className="font-medium">
                  {chapter.title}
                </span>
                <span className="text-xs text-gray-500">
                  {chapter.scanlator} - {formatDate(chapter.dateUploaded)}
                </span>
              </div>
              <a
                href={chapter.url}
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-500 text-sm font-medium"
              >
                Read
              </a>
            </li>
          ))}
        </ul>
      )}
      {/* spacer */}
      <div className="h-16"></div>
    </div>
  );
}

export default MangaChapter;
