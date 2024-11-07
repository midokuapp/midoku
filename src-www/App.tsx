import { createBrowserRouter, RouterProvider } from "react-router-dom";

import Browse from "./pages/Browse/Browse.tsx";
import ExtensionBrowser from "./pages/Browse/ExtensionBrowse.tsx";
import Extensions from "./pages/Extensions.tsx";
import More from "./pages/More.tsx";
import MangaDetails from "./pages/MangaDetails.tsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Browse />,
  },
  {
    path: "/browse/:extensionId",
    element: <ExtensionBrowser />,
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
]);

export default function App() {
  return (
    <div className="flex flex-col h-screen">
      <main className="flex-1 p-2">
        <RouterProvider router={router} />
      </main>
      <ul className="menu menu-horizontal justify-evenly bg-base-200">
        <li>
          <a href="/">Browse</a>
        </li>
        <li>
          <a href="/extensions">Extensions</a>
        </li>
        <li>
          <a href="/more">More</a>
        </li>
      </ul>
    </div>
  );
}
