function MangaImage({ src, alt, fallBackSrc = "/fallback-img.jpg" }: { src: string; alt: string; fallBackSrc?: string }) {
    return (
        <img
            src={src}
            alt={alt}
            style={{
                width: "100%",
                height: "280px",
                objectFit: "cover",
                borderRadius: "8px",
                boxShadow: "0 4px 8px rgba(0, 0, 0, 0.1)",
            }}
            onError={(e) => (e.currentTarget.src = fallBackSrc)}
        />
    );
}

export default MangaImage;
