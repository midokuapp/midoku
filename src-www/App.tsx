import NavBar from "./components/NavBar.tsx";
import NavItem from "./components/NavItem.tsx";
import Extension from "./pages/Extensions.tsx";

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
        <Extension />
      </main>
      <NavBar>
        <NavItem>
          <span>Extensions</span>
        </NavItem>
        <NavItem>
          <span>More</span>
        </NavItem>
      </NavBar>
    </div>
  );
}
