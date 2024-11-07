import { useEffect, useState } from "react";
import { Extension } from "../../types/extension";
import { Manga } from "../../types/manga";
import { Link, useParams } from "react-router-dom";
import {
  getExtension,
  getIconUrl,
  getMangaList,
} from "../../services/extensions.service";
import "../../style/loader.css";
import MangaImage from "../../components/MangaImage";

export default function ExtensionBrowse() {
  const { extensionId } = useParams();
  const [extension, setExtension] = useState<Extension | null>(null);
  const [mangas, setMangas] = useState<Array<Manga>>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [pagination, setPagination] = useState<number>(1);

  // Charger l'extension une seule fois au démarrage
  useEffect(() => {
    if (!extensionId) return;
    getExtension(extensionId)
      .then(setExtension)
      .catch(() => setError("Erreur lors du chargement de l'extension."));
  }, [extensionId]);

  // Charger la liste de mangas pour chaque pagination
  useEffect(() => {
    if (!extension) return;

    setLoading(true); // Activer le chargement pendant la requête
    getMangaList(extension.id, [], pagination)
      .then((data) => setMangas([...mangas, ...data[0]]))
      .catch(() => setError("Erreur lors du chargement des mangas."))
      .finally(() => setLoading(false));
  }, [extension, pagination]);

  // Gestion du scroll infini
  useEffect(() => {
    const handleScroll = () => {
      if (
        document.body.scrollHeight - 300 <
          globalThis.scrollY + globalThis.innerHeight
      ) {
        if (!loading) {
          setPagination(pagination + 1);
        }
      }
    };
    globalThis.addEventListener("scroll", handleScroll);
    return () => globalThis.removeEventListener("scroll", handleScroll);
  }, [loading]);

  // Afficher un message d'erreur si nécessaire
  if (error) return <ErrorMessage error={error} />;

  // Si l'extension est encore en chargement
  if (!extension) return <Loader />;

  return (
    <div style={{ padding: "0 2rem" }}>
      <ExtensionHeader extension={extension} />
      <MangaGrid mangas={mangas} extensionId={extension.id} />
      {loading && <Loader />}
    </div>
  );
}

// Composant de message d'erreur
const ErrorMessage = ({ error }: { error: string }) => (
  <div style={{ padding: "0 2rem" }}>
    <p style={{ color: "red" }}>{error}</p>
  </div>
);

// Composant de loader
const Loader = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      justifyContent: "center",
      alignItems: "center",
    }}
  >
    <div style={{ textAlign: "center", padding: "20px 0" }}>Chargement...</div>
    <div className="loader"></div>
  </div>
);

// En-tête de l'extension avec le nom et l'icône
const ExtensionHeader = ({ extension }: { extension: Extension }) => (
  <div
    style={{ display: "flex", alignItems: "center", gap: 12, marginBottom: 20 }}
  >
    <img
      src={getIconUrl(extension.iconPath)}
      alt={extension.source.name}
      style={{ width: 48, height: 48 }}
    />
    <h2 style={{ fontSize: 24 }}>{extension.source.name}</h2>
  </div>
);

// Grille de mangas
const MangaGrid = (
  { mangas, extensionId }: { mangas: Array<Manga>; extensionId: string },
) => (
  <ul
    style={{
      display: "grid",
      gridTemplateColumns: "repeat(auto-fill, minmax(180px, 5fr))",
      gap: "16px",
      listStyle: "none",
      padding: 0,
    }}
  >
    {mangas.map((manga) => (
      <li key={manga.id} style={{ textAlign: "center" }}>
        <Link
          to={{ pathname: `/browse/${extensionId}/${manga.id}` }}
          state={manga}
        >
          <MangaImage src={manga.coverUrl} alt={manga.title} />
        </Link>
        <a
          href={manga.url}
          target="_blank"
          rel="noopener noreferrer"
          style={{ textDecoration: "none", color: "#333" }}
        >
          <p
            style={{
              marginTop: 8,
              fontSize: 14,
              fontWeight: "bold",
              overflow: "hidden",
              textOverflow: "ellipsis",
              whiteSpace: "nowrap",
            }}
          >
            {manga.title}
          </p>
        </a>
      </li>
    ))}
  </ul>
);
