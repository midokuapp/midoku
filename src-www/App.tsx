import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import { Extension, Source } from "./types/extension.ts";
import { Manga } from "./types/manga.ts";

async function getExtensions(): Promise<Array<Extension>> {
  const extensionsData: Array<[string, Source, string]> = await invoke(
    "get_extensions",
  );
  return extensionsData.map(([id, source, iconPath]) => {
    return new Extension(id, source, iconPath);
  });
}

export default function App() {
  const [extensions, setExtensions] = useState<Array<Extension>>([]);
  const [mangaList, _setMangaList] = useState<Array<Manga>>([]);

  useEffect(() => {
    getExtensions().then(setExtensions);
  }, []);

  const listExtensions = extensions.map((extension: Extension) => {
    return (
      <li
        key={extension.id}
        style={{ display: "flex", alignItems: "center", gap: 12 }}
      >
        <img src={extension.iconUrl} style={{ width: 48, height: 48 }} />
        <div>
          <span style={{ display: "block", fontSize: 16 }}>
            {extension.name}
          </span>
          <div style={{ opacity: 0.7, fontSize: 14 }}>
            {extension.language} {extension.version}
            {extension.nsfw && <span style={{ color: "red" }}>{" "}18+</span>}
          </div>
        </div>
        <button
          onClick={() => {
            extension.getMangaList([], 0).then((mangaList) =>
              console.log(mangaList)
            );
          }}
        >
          Manga List
        </button>
      </li>
    );
  });

  return (
    <>
      <ul
        style={{
          display: "flex",
          flexDirection: "column",
          gap: 8,
        }}
      >
        {listExtensions}
      </ul>
      <div>
        {mangaList.toString()}
      </div>
    </>
  );
}
