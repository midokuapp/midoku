import { useEffect, useState } from "react";
import { Manifest } from "../types/manifest.ts";
import {
  getExtensions,
  getRepositoryExtensions,
  installExtension,
  uninstallExtension,
} from "../services/tauri.service.ts";

import { getIconUrl } from "../services/extensions.service.ts";
import { useExtensions } from "../context/extensions.ts";
import { useRepositoryUrl } from "../context/repositoryUrl.ts";

export default function Extensions() {
  const { extensions, setExtensions } = useExtensions();
  const { repositoryUrl, setRepositoryUrl } = useRepositoryUrl();
  const [manifests, setManifests] = useState<Manifest[]>([]);

  // Fetch repository extensions when repositoryUrl changes
  useEffect(() => {
    if (!repositoryUrl) return;
    getRepositoryExtensions(repositoryUrl).then(setManifests);
  }, [repositoryUrl]);

  // Helper to check if an extension is installed
  const isInstalled = (manifestId: string) => {
    return extensions.some((extension) => extension.id === manifestId);
  };

  // Install and uninstall extension actions with real-time update
  const handleInstall = async (manifest: Manifest) => {
    await installExtension(repositoryUrl, manifest);
    const updatedExtensions = await getExtensions();
    setExtensions(updatedExtensions);
  };

  const handleUninstall = async (manifestId: string) => {
    await uninstallExtension(manifestId);
    const updatedExtensions = await getExtensions();
    setExtensions(updatedExtensions);
  };

  // Map over repository extensions and create a list of components
  const manifestList = manifests.map((manifest: Manifest) => {
    const installed = isInstalled(manifest.id);
    return (
      <li
        key={manifest.id}
        className="flex items-center gap-4 p-3 bg-gray-800 rounded-lg shadow-md"
      >
        <figure className="w-12 h-12">
          <img
            src={`${repositoryUrl}/icons/${manifest.icon}`}
            alt={manifest.name}
            className="rounded-full"
          />
        </figure>
        <div className="flex flex-col">
          <h2 className="text-lg font-semibold text-white">{manifest.name}</h2>
          <p className="text-gray-400 text-sm">
            {manifest.version} · {manifest.language}
            {manifest.nsfw && <span className="text-red-500 ml-2">+18</span>}
          </p>
        </div>
        <div className="ml-auto">
          {installed
            ? (
              <button
                className="bg-red-600 text-white py-1 px-3 rounded-lg text-sm hover:bg-red-500 transition"
                onClick={() => handleUninstall(manifest.id)}
              >
                Uninstall
              </button>
            )
            : (
              <button
                className="bg-green-600 text-white py-1 px-3 rounded-lg text-sm hover:bg-green-500 transition"
                onClick={() => handleInstall(manifest)}
              >
                Install
              </button>
            )}
        </div>
      </li>
    );
  });

  return (
    <div className="max-w-lg mx-auto p-6 text-white min-h-screen">
      <h1 className="text-2xl font-bold mb-4">Extension Manager</h1>
      <p className="text-gray-400 mb-4">
        Manage your manga extensions: Install new sources or uninstall those you
        no longer need.
      </p>

      {/* Repository URL Input */}
      <div className="flex items-center mb-4">
        <input
          type="text"
          className="flex-1 p-2 bg-gray-800 border border-gray-700 rounded-lg text-gray-300 placeholder-gray-500 focus:outline-none focus:border-gray-500"
          placeholder="Extension repository URL"
          value={repositoryUrl}
          onChange={(e) => setRepositoryUrl(e.target.value)}
        />
      </div>

      {/* Installed Extensions List */}
      <h2 className="text-xl font-semibold mb-2">Installed Extensions</h2>
      <ul className="space-y-4 mb-8">
        {extensions.map((extension) => (
          <li
            key={extension.id}
            className="flex items-center gap-4 p-3 bg-gray-800 rounded-lg shadow-md"
          >
            <figure className="w-12 h-12">
              <img
                src={getIconUrl(extension.iconPath)}
                alt={extension.source.name}
                className="rounded-full"
              />
            </figure>
            <div className="flex flex-col">
              <h2 className="text-lg font-semibold text-white">
                {extension.source.name}
              </h2>
              <p className="text-gray-400 text-sm">
                {extension.source.version} · {extension.source.language}
                {extension.source.nsfw && (
                  <span className="text-red-500 ml-2">+18</span>
                )}
              </p>
            </div>
            <button
              className="ml-auto bg-red-600 text-white py-1 px-3 rounded-lg text-sm hover:bg-red-500 transition"
              onClick={() => handleUninstall(extension.id)}
            >
              Uninstall
            </button>
          </li>
        ))}
      </ul>

      {/* Available Extensions List */}
      <h2 className="text-xl font-semibold mb-2">Available Extensions</h2>
      <ul className="space-y-4">{manifestList}</ul>
    </div>
  );
}
