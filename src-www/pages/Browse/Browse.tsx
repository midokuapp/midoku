import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import { Extension, Source } from "../../types/extension";
import { Link } from "react-router-dom";

export default function Browse() {
  const [extensions, setExtensions] = useState<Array<Extension>>([]);

  useEffect(() => {
    invoke<Array<[string, Source, string]>>("get_extensions").then((data) => {
      setExtensions(data.map(([id, source, iconPath]) => {
        return new Extension(id, source, iconPath);
      }));
    });
  }, []);

  return (
    <ul
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 8,
        padding: "0 2rem",
      }}
    >
      {extensions.map((extension: Extension) => {
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
                {extension.language}
              </div>
            </div>
            <Link
              to={{
                pathname: "/browse/" + extension.id,
              }}
              state={{ extension: extension }}
              style={{
                marginLeft: "auto",
              }}
            >
              Browse
            </Link>
          </li>
        );
      })}
    </ul>
  );
}
