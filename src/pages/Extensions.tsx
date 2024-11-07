import { useEffect, useState } from "react";

import { Manifest } from "../types/manifest.ts";
import { store } from "../store.ts";
import {
  getRepositoryExtensions,
  installExtension,
  uninstallExtension,
} from "../services/tauri.service.ts";

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
    <li key={manifest.id} className="flex items-center gap-2">
      <figure className="size-16">
        <img
          src={repositoryUrl + "/icons/" + manifest.icon}
          alt={manifest.name}
        />
      </figure>
      <div>
        <h2 className="">{manifest.name}</h2>
        <p>
          {manifest.version} {manifest.language}
          {manifest.nsfw && <span className="text-error">{" "}+18</span>}
        </p>
      </div>
      <div className="flex gap-2 ml-auto">
        <button
          className="btn btn-success"
          onClick={() =>
            installExtension(repositoryUrl, manifest)}
        >
          Install
        </button>
        <button
          className="btn btn-error"
          onClick={() =>
            uninstallExtension(manifest.id)}
        >
          Uninstall
        </button>
      </div>
    </li>
  ));

  return (
    <>
      <input
        type="text"
        className="input input-bordered w-full"
        placeholder="Extension repository URL"
        value={repositoryUrl}
        onChange={(e) => setRepositoryUrl(e.target.value)}
      />
      <ul className="py-3">
        {manifestList}
      </ul>
    </>
  );
}
