import { useEffect, useState } from "react";
import { Extension } from "../../types/extension";
import { Manga } from "../../types/manga";
import { useLocation } from "react-router-dom";

import "../../style/loader.css";

export default function ExtensionBrowse() {
  const { state } = useLocation();
  const extension = state.extension as Extension;
  globalThis.addEventListener("scroll", handleScroll);

  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  const [mangas, setMangas] = useState<Array<Manga>>([]);
  const [pagination, setPagination] = useState<number>(1);

  // Récupérer l'extension
  useEffect(() => {
    console.log("Extension :", extension);
  }, []);

  useEffect(() => {
    if (!extension || !extension.getMangaList) {
      return setError(
        "L'extension n'est pas valide. Veuillez passer par la page d'accueil.",
      );
    }

    // Récupérer la liste des mangas pour l'extension
    extension.getMangaList([], pagination).then((data) => {
      setMangas([...mangas, ...data[0]]);
      setLoading(false);
      console.log("Mangas chargés :", data);
    }).catch((error) => {
      console.error("Erreur lors du chargement des mangas :", error);
      setError("Une erreur est survenue lors du chargement des mangas.");
    });
  }, [extension, pagination]);

  function handleScroll() {
    if (
      document.body.scrollHeight - 300 <
        globalThis.scrollY + globalThis.innerHeight
    ) {
      setLoading(true);
      setPagination(pagination + 1);
    }
  }

  // Afficher un message d'erreur si nécessaire
  if (error) {
    return (
      <div style={{ padding: "0 2rem" }}>
        <p style={{ color: "red" }}>{error}</p>
      </div>
    );
  }

  return (
    <div style={{ padding: "0 2rem" }}>
      {/* En-tête de l'extension */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 12,
          marginBottom: 20,
        }}
      >
        <img
          src={extension.iconUrl}
          alt={extension.name}
          style={{ width: 48, height: 48 }}
        />
        <h2 style={{ fontSize: 24 }}>{extension.name}</h2>
      </div>

      {/* Grille des mangas avec image et titre */}
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
            {/* Image de couverture */}
            <a href={manga.url} target="_blank" rel="noopener noreferrer">
              <img
                src={manga.coverUrl}
                alt={manga.title}
                style={{
                  width: "100%",
                  height: "280px",
                  objectFit: "cover",
                  borderRadius: "8px",
                  boxShadow: "0 4px 8px rgba(0, 0, 0, 0.1)",
                }}
              />
            </a>
            {/* Titre du manga */}
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

      {/* infinie scroll loading */}
      {loading &&
        // center the loader

        <div
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            alignItems: "center",
          }}
        >
          <div style={{ textAlign: "center", padding: "20px 0" }}>
            Chargement...
          </div>
          <div className="loader"></div>
        </div>}
    </div>
  );
}
