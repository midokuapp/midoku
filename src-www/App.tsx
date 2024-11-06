import { createBrowserRouter, RouterProvider } from "react-router-dom";

import Browse from "./pages/Browse/Browse";
import ExtensionBrowser from "./pages/Browse/ExtensionBrowse";
import NavBar from "./components/NavBar";
import NavItem from "./components/NavItem";
import Extensions from "./pages/Extensions";
import More from "./pages/More";
import Youn from "./pages/Youn";

import "./style/global.css";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Browse />,
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
