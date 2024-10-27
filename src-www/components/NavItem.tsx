export default function NavItem(
  { href, children }: { href: string; children: React.ReactNode },
) {
  return (
    <a
      href={href}
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
