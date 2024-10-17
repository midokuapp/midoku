import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

interface Source {
  name: string;
  language: string;
  version: string;
  url: string;
  nsfw: boolean;
}

interface Extension {
  source: Source;
  iconPath: string;
}

async function getExtensions(): Promise<Array<Extension>> {
  return await invoke("get_extensions");
}

export default function App() {
  const [extensions, setExtensions] = useState<Array<Extension>>([]);

  useEffect(() => {
    getExtensions().then(setExtensions);
  }, []);

  const listExtensions = extensions.map((extension: Extension) => {
    const name = extension.source.name;
    const language = extension.source.language;
    const version = extension.source.version;
    const nsfw = extension.source.nsfw;

    const iconUrl = convertFileSrc(extension.iconPath);

    return (
      <li key={name} style={{ display: "flex", alignItems: "center", gap: 12 }}>
        <img src={iconUrl} style={{ width: 48, height: 48 }} />
        <div>
          <span style={{ display: "block", fontSize: 16 }}>{name}</span>
          <div style={{ opacity: 0.7, fontSize: 14 }}>
            {language} {version}
            {nsfw && <span style={{ color: "red" }}>{" "}18+</span>}
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
