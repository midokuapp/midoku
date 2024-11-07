import { useEffect, useState } from "react";
import { Link } from "react-router-dom";

import { Extension } from "../../types/extension.ts";
import { getIconUrl } from "../../services/extensions.service.ts";
import { getExtensions } from "../../services/tauri.service.ts";

export default function Browse() {
  const [extensions, setExtensions] = useState<Extension[]>([]);

  useEffect(() => {
    getExtensions().then(setExtensions);
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
            <img
              src={getIconUrl(extension.iconPath)}
              style={{ width: 48, height: 48 }}
            />
            <div>
              <span style={{ display: "block", fontSize: 16 }}>
                {extension.source.name}
              </span>
              <div style={{ opacity: 0.7, fontSize: 14 }}>
                {extension.source.language}
              </div>
            </div>
            <Link
              to={`/browse/${extension.id}`}
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
