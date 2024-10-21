import { createBrowserRouter, RouterProvider } from "react-router-dom";

import NavBar from "./components/NavBar.tsx";
import NavItem from "./components/NavItem.tsx";
import Extension from "./pages/Extensions.tsx";
import More from "./pages/More.tsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Extension />,
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
          <span>Extensions</span>
        </NavItem>
        <NavItem href="/more">
          <span>More</span>
        </NavItem>
      </NavBar>
    </div>
  );
}
