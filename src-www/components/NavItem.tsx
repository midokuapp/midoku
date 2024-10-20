export default function NavItem({ children }: { children: React.ReactNode }) {
  return (
    <a
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: "0.25rem",
      }}
    >
      {children}
    </a>
  );
}
