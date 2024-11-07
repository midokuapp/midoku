import { useEffect, useState } from "react";

import { Manifest } from "../types/manifest";
import { store } from "../store";
import {
  getRepositoryExtensions,
  installExtension,
  uninstallExtension,
} from "../services/tauri.service";

export default function Extensions() {
  const [repositoryUrl, setRepositoryUrl] = useState<string>("");
  const [manifests, setManifests] = useState<Manifest[]>([]);

  useEffect(() => {
    store.get<string>("extensionRepositoryUrl").then((data) => {
      if (data) {
        setRepositoryUrl(data);
      }
    });
  }, []);

  useEffect(() => {
    if (!repositoryUrl) return;

    store.set("extensionRepositoryUrl", repositoryUrl);

    getRepositoryExtensions(repositoryUrl).then(setManifests);
  }, [repositoryUrl]);

  const manifestList = manifests.map((manifest: Manifest) => (
    <li key={manifest.id}>
      <p>{manifest.id}</p>
      <p>{manifest.name}</p>
      <p>{manifest.version}</p>
      <p>{manifest.language}</p>
      <p>NSFW: {manifest.nsfw ? "Yes" : "No"}</p>
      <p>
        <a href={repositoryUrl + "/extensions/" + manifest.extension}>
          {manifest.extension}
        </a>
      </p>
      <p>
        <img
          src={repositoryUrl + "/icons/" + manifest.icon}
          alt={manifest.name}
        />
      </p>
      <button
        onClick={() =>
          installExtension(repositoryUrl, manifest)}
      >
        Install
      </button>
      <button
        onClick={() => uninstallExtension(manifest.id)}
      >
        Uninstall
      </button>
    </li>
  ));

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
      <ul>
        {manifestList}
      </ul>
    </>
  );
}
