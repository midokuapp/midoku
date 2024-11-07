import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";

import MangaImage from "../components/MangaImage.tsx";
import { Manga, ReadingMode } from "../types/manga.ts";
import { getMangaDetails } from "../services/extensions.service.ts";

import "../style/loader.css";

export default function MangaDetails() {
  const { extensionId, mangaId } = useParams<
    { extensionId: string; mangaId: string }
  >();

  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [manga, setManga] = useState<Manga | null>(null);

  // Récupérer le manga
  useEffect(() => {
    if (!extensionId || !mangaId) {
      setError("Erreur lors de la récupération des détails du manga.");
      setLoading(false);
      return;
    }
    getMangaDetails(extensionId, mangaId)
      .then((data) => {
        setManga(data);
        setLoading(false);
      })
      .catch((err) => {
        setError(
          "Erreur lors de la récupération des détails du manga. : " + err,
        );
        setLoading(false);
      });
  }, [extensionId, mangaId]);

  if (loading) {
    return (
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
          padding: "2rem",
        }}
      >
        <div>Chargement...</div>
        <div className="loader"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div style={{ padding: "0 2rem", color: "red" }}>
        {error}
      </div>
    );
  }

  if (!manga) {
    return null;
  }

  return (
    <div style={{ padding: "2rem", maxWidth: 800, margin: "0 auto" }}>
      {/* Titre et couverture du manga */}
      <div style={{ display: "flex", gap: "1.5rem", alignItems: "flex-start" }}>
        <MangaImage src={manga.coverUrl} alt={manga.title} />
        <div>
          <h1 style={{ fontSize: "1.8rem", marginBottom: "0.5rem" }}>
            {manga.title}
          </h1>
          <div style={{ opacity: 0.7, marginBottom: "0.5rem" }}>
            Statut : {manga.status}
          </div>
          <div style={{ opacity: 0.7 }}>
            Lecture : {manga.readingMode === ReadingMode.RightToLeft
              ? "Droite à Gauche"
              : "Gauche à Droite"}
          </div>
        </div>
      </div>

      {/* Informations sur le manga */}
      <div style={{ marginTop: "1.5rem" }}>
        <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>Détails</h2>
        <p style={{ marginBottom: "0.5rem" }}>
          <strong>Auteur :</strong> {manga.authorName}
        </p>
        <p style={{ marginBottom: "0.5rem" }}>
          <strong>Artiste :</strong> {manga.artistName}
        </p>
        <p style={{ marginBottom: "0.5rem" }}>
          <strong>Catégories :</strong> {manga.categories.join(", ")}
        </p>
        <p>
          <strong>Note de Contenu :</strong> {manga.contentRating}
        </p>
      </div>

      {/* Description */}
      <div style={{ marginTop: "1.5rem" }}>
        <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
          Description
        </h2>
        <p>{manga.description}</p>
      </div>

      {/* Lien vers le site source */}
      <div style={{ marginTop: "2rem", textAlign: "center" }}>
        <Link
          to={{ pathname: manga.url }}
          target="_blank"
          style={{
            padding: "0.8rem 2rem",
            backgroundColor: "#007bff",
            color: "white",
            borderRadius: "5px",
            textDecoration: "none",
          }}
        >
          Lire sur le site source
        </Link>
      </div>
    </div>
  );
}
