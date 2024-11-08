import { useState } from "react";
import {
  createBrowserRouter,
  NavLink,
  Outlet,
  RouterProvider,
} from "react-router-dom";
import { FiBook, FiGlobe, FiGrid, FiSettings } from "react-icons/fi"; // Icônes
import Browse from "./pages/Browse/Browse.tsx";
import ExtensionBrowse from "./pages/Browse/ExtensionBrowse.tsx";
import Extensions from "./pages/Extensions.tsx";
import More from "./pages/More.tsx";
import MangaDetails from "./pages/MangaDetails.tsx";
import { ExtensionsContext } from "./context/extensions.ts";
import { Extension } from "./types/extension.ts";
import { RepositoryUrlContext } from "./context/repositoryUrl.ts";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        path: "/browse",
        element: <Browse />,
      },
      {
        path: "/browse/:extensionId",
        element: <ExtensionBrowse />,
      },
      {
        path: "/browse/:extensionId/:mangaId",
        element: <MangaDetails />,
      },
      {
        path: "/extensions",
        element: <Extensions />,
      },
      {
        path: "/more",
        element: <More />,
      },
    ],
  },
]);

// Fonction pour gérer l'état d'onglet actif
function Layout() {
  // const location = useLocation();

  // Définir les onglets et leurs routes
  const navItems = [
    { path: "/", icon: <FiBook />, label: "Library" },
    { path: "/browse", icon: <FiGlobe />, label: "Browse" },
    { path: "/extensions", icon: <FiGrid />, label: "Extensions" },
    { path: "/more", icon: <FiSettings />, label: "More" },
  ];

  return (
    <div>
      <main className="flex-1 p-2 overflow-y-auto">
        <Outlet />
      </main>
      {/* Barre de navigation, shadow vers le haut */}
      <nav className="fixed bottom-0 w-full bg-base-200 p-2 border-t border-base-100 flex justify-around shadow-[rgba(0,_0,_0,_0.4)_0px_30px_90px]">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) =>
              `flex flex-col items-center p-2 text-sm ${
                isActive ? "text-primary" : "text-gray-500"
              }`}
          >
            {item.icon}
            <span>{item.label}</span>
          </NavLink>
        ))}
      </nav>
    </div>
  );
}

export default function App() {
  const [extensions, setExtensions] = useState<Extension[]>([]);
  const [repositoryUrl, setRepositoryUrl] = useState<string>("");

  return (
    <ExtensionsContext.Provider
      value={{
        extensions,
        setExtensions,
      }}
    >
      <RepositoryUrlContext.Provider
        value={{
          repositoryUrl,
          setRepositoryUrl,
        }}
      >
        <div className="flex flex-col h-screen">
          <RouterProvider router={router} />
        </div>
      </RepositoryUrlContext.Provider>
    </ExtensionsContext.Provider>
  );
}
