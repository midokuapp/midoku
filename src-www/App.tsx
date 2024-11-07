import { createBrowserRouter, RouterProvider } from "react-router-dom";

import Browse from "./pages/Browse/Browse.tsx";
import ExtensionBrowser from "./pages/Browse/ExtensionBrowse.tsx";
import NavBar from "./components/NavBar.tsx";
import NavItem from "./components/NavItem.tsx";
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
    <div
      style={{
        height: "100vh",
        width: "100vw",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <main style={{ flex: 1 }}>
        <RouterProvider router={router} />
      </main>
      <NavBar>
        <NavItem href="/">
          <span>Browse</span>
        </NavItem>
        <NavItem href="/extensions">
          <span>Extensions</span>
        </NavItem>
        <NavItem href="/more">
          <span>More</span>
        </NavItem>
      </NavBar>
    </div>
  );
}
