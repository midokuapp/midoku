import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

interface Extension {
  name: string;
  language: string;
  version: string;
  url: string;
  nsfw: boolean;
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
    return (
      <li>
        <span style={{ display: "block", fontSize: 16 }}>{extension.name}</span>
        <div style={{ opacity: 0.7, fontSize: 14 }}>
          {extension.language} {extension.version}
          {extension.nsfw && <span style={{ color: "red" }}>18+</span>}
        </div>
      </li>
    );
  });

  return <ul>{listExtensions}</ul>;
}
