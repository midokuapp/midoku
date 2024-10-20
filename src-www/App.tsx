import { useState } from "react";

import NavBar from "./components/NavBar.tsx";
import NavItem from "./components/NavItem.tsx";
import Extension from "./pages/Extensions.tsx";
import More from "./pages/More.tsx";

export default function App() {
  const [page, setPage] = useState("extension");

  const pages: { [key: string]: React.ReactNode } = {
    extension: <Extension />,
    more: <More />,
  };

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
        {pages[page]}
      </main>
      <NavBar>
        <NavItem onClick={() => setPage("extension")}>
          <span>Extensions</span>
        </NavItem>
        <NavItem onClick={() => setPage("more")}>
          <span>More</span>
        </NavItem>
      </NavBar>
    </div>
  );
}
