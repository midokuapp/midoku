export default function NavItem(
  { onClick, children }: { onClick: () => void; children: React.ReactNode },
) {
  return (
    <a
      onClick={onClick}
      style={{
        cursor: "pointer",
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
