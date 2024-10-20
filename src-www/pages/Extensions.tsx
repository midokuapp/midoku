import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import { Extension, Source } from "../types/extension.ts";

export default function Extensions() {
  const [extensions, setExtensions] = useState<Array<Extension>>([]);

  useEffect(() => {
    invoke<Array<[string, Source, string]>>("get_extensions").then(
      (data) => {
        setExtensions(data.map(([id, source, iconPath]) => {
          return new Extension(id, source, iconPath);
        }));
      },
    );
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
      </li>
    );
  });

  return (
    <ul
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 8,
      }}
    >
      {listExtensions}
    </ul>
  );
}
