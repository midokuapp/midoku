import { useEffect, useState } from "react";

import { Extension } from "../types/extension.ts";
import { store } from "../store.ts";
import { getExtensions } from "../tauri.ts";

export default function Extensions() {
  const [extensions, setExtensions] = useState<Extension[]>([]);
  const [repositoryUrl, setRepositoryUrl] = useState<string>("");

  useEffect(() => {
    getExtensions().then(setExtensions);

    store.get<string>("extensionRepositoryUrl").then((data) => {
      if (data) setRepositoryUrl(data);
    });
  }, []);

  useEffect(() => {
    store.set("extensionRepositoryUrl", repositoryUrl);
  }, [repositoryUrl]);

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
      </li>
    );
  });

  return (
    <>
      <input
        type="text"
        style={{
          // 0.5rem is padding, 4px is border width on either side
          width: "calc(100% - 0.5rem - 4px)",
          padding: "0.25rem",
          borderWidth: "2px",
        }}
        placeholder="Extension repository URL"
        value={repositoryUrl}
        onChange={(e) => setRepositoryUrl(e.target.value)}
      />
      <ul
        style={{
          display: "flex",
          flexDirection: "column",
          gap: 8,
        }}
      >
        {listExtensions}
      </ul>
    </>
  );
}
