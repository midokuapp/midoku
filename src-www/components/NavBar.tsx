export default function NavBar({ children }: { children: React.ReactNode }) {
  return (
    <nav
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        justifyContent: "space-around",
        padding: "1rem 0.75rem",
      }}
    >
      {children}
    </nav>
  );
}
