import { useEffect, useState } from "react";
import { Manifest } from "../types/manifest.ts";
import { storeService } from "../services/store.service.ts";
import {
    getRepositoryExtensions,
    installExtension,
    uninstallExtension,
    getExtensions,
} from "../services/tauri.service.ts";

export default function Extensions() {
    const [repositoryUrl, setRepositoryUrl] = useState<string>("");
    const [manifests, setManifests] = useState<Manifest[]>([]);
    const [installedExtensions, setInstalledExtensions] = useState<Extension[]>([]);

    // Load installed extensions on component mount
    useEffect(() => {
        getExtensions().then(setInstalledExtensions);
    }, []);

    // Load repository URL from storage and fetch manifests if URL is available
    useEffect(() => {
        storeService.get<string>("extensionRepositoryUrl").then((data) => {
            if (data) {
                setRepositoryUrl(data);
            }
        });
    }, []);

    // Fetch repository extensions when repositoryUrl changes
    useEffect(() => {
        if (!repositoryUrl) return;

        storeService.set("extensionRepositoryUrl", repositoryUrl);
        getRepositoryExtensions(repositoryUrl).then(setManifests);
    }, [repositoryUrl]);

    // Helper to check if an extension is installed
    const isInstalled = (manifestId: string) => {
        return installedExtensions.some((ext) => ext.id === manifestId);
    };

    // Install and uninstall extension actions with real-time update
    const handleInstall = async (manifest: Manifest) => {
        await installExtension(repositoryUrl, manifest);
        const updatedExtensions = await getExtensions();
        setInstalledExtensions(updatedExtensions);
    };

    const handleUninstall = async (manifestId: string) => {
        await uninstallExtension(manifestId);
        const updatedExtensions = await getExtensions();
        setInstalledExtensions(updatedExtensions);
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
                    {installed ? (
                        <button
                            className="bg-red-600 text-white py-1 px-3 rounded-lg text-sm hover:bg-red-500 transition"
                            onClick={() => handleUninstall(manifest.id)}
                        >
                            Uninstall
                        </button>
                    ) : (
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
                Manage your manga extensions: Install new sources or uninstall those
                you no longer need.
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
                <button
                    className="ml-2 text-gray-400 hover:text-gray-200"
                    onClick={() => window.open("https://sehnryr.github.io/midoku-community-extensions/", "_blank")}
                    aria-label="Learn more about the official repository"
                >
                    ?
                </button>
            </div>

            {/* Installed Extensions List */}
            <h2 className="text-xl font-semibold mb-2">Installed Extensions</h2>
            <ul className="space-y-4 mb-8">
                {installedExtensions.map((ext) => (
                    <li
                        key={ext.id}
                        className="flex items-center gap-4 p-3 bg-gray-800 rounded-lg shadow-md"
                    >
                        <figure className="w-12 h-12">
                            <img
                                src={ext.iconPath}
                                alt={ext.source.name}
                                className="rounded-full"
                            />
                        </figure>
                        <div className="flex flex-col">
                            <h2 className="text-lg font-semibold text-white">
                                {ext.source.name}
                            </h2>
                            <p className="text-gray-400 text-sm">
                                {ext.source.version} · {ext.source.language}
                                {ext.source.nsfw && <span className="text-red-500 ml-2">+18</span>}
                            </p>
                        </div>
                        <button
                            className="ml-auto bg-red-600 text-white py-1 px-3 rounded-lg text-sm hover:bg-red-500 transition"
                            onClick={() => handleUninstall(ext.id)}
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
