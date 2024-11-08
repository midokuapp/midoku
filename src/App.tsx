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
    <>
      <main className="flex-1 p-2 overflow-y-auto">
        <Outlet />
      </main>
      {/* Barre de navigation, shadow vers le haut */}
      <nav className="w-full bg-base-200 px-2 border-t border-base-100 shadow-[rgba(0,_0,_0,_0.4)_0px_30px_90px]">
        <div className="grid grid-cols-4 max-w-xl mx-auto py-3">
          {navItems.map((item) => (
            <NavLink
              key={item.path}
              to={item.path}
              className={({ isActive }) =>
                `flex flex-col items-center p-2 text-sm ${
                  isActive ? "text-primary" : "text-neutral"
                }`}
            >
              {item.icon}
              <span>{item.label}</span>
            </NavLink>
          ))}
        </div>
      </nav>
    </>
  );
}

export default function App() {
  return (
    <div className="flex flex-col h-screen">
      <RouterProvider router={router} />
    </div>
  );
}
