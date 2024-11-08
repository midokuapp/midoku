import { useEffect, useState } from "react";
import { Manifest } from "../types/manifest.ts";
import {
  getExtensions,
  getRepositoryExtensions,
  installExtension,
  uninstallExtension,
} from "../services/tauri.service.ts";
import { FiDownload, FiTrash2 } from "react-icons/fi";
import { useStore } from "../services/store.service.ts";

export default function Extensions() {
  const extensions = useStore((state) => state.extensions);
  const setExtensions = useStore((state) => state.setExtensions);
  const repositoryUrl = useStore((state) => state.repositoryUrl);
  const setRepositoryUrl = useStore((state) => state.setRepositoryUrl);
  const manifests = useStore((state) => state.manifests);
  const setManifests = useStore((state) => state.setManifests);

  // Fetch repository extensions when repositoryUrl changes
  useEffect(() => {
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

  const Icon = ({ src, alt }: { src: string; alt: string }) => (
    <figure className="w-12 h-12">
      <img src={src} alt={alt} className="rounded-md" />
    </figure>
  );

  const Detail = ({ children }: { children: React.ReactNode }) => (
    <div className="flex flex-col">{children}</div>
  );

  const Title = ({ title }: { title: string }) => (
    <h2 className="text-lg font-semibold">{title}</h2>
  );

  const Description = ({ language, version, nsfw }: {
    language: string;
    version: string;
    nsfw: boolean;
  }) => (
    <p className="text-neutral text-sm">
      {language} {version}
      {nsfw && <span className="text-error">{" "}+18</span>}
    </p>
  );

  const InstallButton = ({ manifest }: { manifest: Manifest }) => {
    const [downloading, setDownloading] = useState(false);

    return (
      <button
        className="ml-auto btn btn-circle text-lg hover:btn-success"
        onClick={() => {
          handleInstall(manifest);
          setDownloading(true);
        }}
      >
        {downloading
          ? <span className="loading loading-spinner"></span>
          : <FiDownload />}
      </button>
    );
  };

  const UninstallButton = ({ extensionId }: { extensionId: string }) => {
    return (
      <button
        className="ml-auto btn btn-circle text-lg hover:btn-error"
        onClick={() => handleUninstall(extensionId)}
      >
        <FiTrash2 />
      </button>
    );
  };

  const Item = (
    { key, children }: {
      key: string;
      children: React.ReactNode;
    },
  ) => (
    <li key={key} className="flex items-center gap-4 p-3 rounded-lg shadow-md">
      {children}
    </li>
  );

  const Group = ({ children }: { children: React.ReactNode }) => (
    <div className="space-y-4 mb-8">{children}</div>
  );

  return (
    <div className="max-w-xl mx-auto p-3">
      <h1 className="text-2xl font-bold mb-4">Extension Manager</h1>
      <p className="text-neutral  mb-4">
        Manage your manga extensions: Install new sources or uninstall those you
        no longer need.
      </p>

      {/* Repository URL Input */}
      <div className="flex items-center mb-4">
        <input
          type="text"
          className="input input-bordered w-full"
          placeholder="Extension repository URL"
          value={repositoryUrl}
          onChange={(e) => setRepositoryUrl(e.target.value)}
        />
      </div>

      {/* Installed Extensions List */}
      <h2 className="text-xl font-semibold mb-2">Installed</h2>
      <Group>
        {extensions.map((extension) => (
          <Item key={extension.id}>
            <Icon
              src={extension.iconUrl}
              alt={extension.source.name}
            />
            <Detail>
              <Title title={extension.source.name} />
              <Description {...extension.source} />
            </Detail>
            <UninstallButton extensionId={extension.id} />
          </Item>
        ))}
      </Group>

      {/* Available Extensions List */}
      <h2 className="text-xl font-semibold mb-2">Available</h2>
      <Group>
        {manifests.map((manifest) =>
          !isInstalled(manifest.id) && (
            <Item key={manifest.id}>
              <Icon
                src={`${repositoryUrl}/icons/${manifest.icon}`}
                alt={manifest.name}
              />
              <Detail>
                <Title title={manifest.name} />
                <Description {...manifest} />
              </Detail>
              <InstallButton manifest={manifest} />
            </Item>
          )
        )}
      </Group>
    </div>
  );
}
