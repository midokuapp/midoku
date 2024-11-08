function MangaImage(
  {
    src,
    alt,
    fallBackSrc = "/fallback-img.jpg",
    height = 200,
    width = 100,
  }: {
    src: string;
    alt: string;
    fallBackSrc?: string;
    height?: number;
    width?: number;
  },
) {
  return (
    <img
      src={src}
      alt={alt}
      style={{
        width: `${width}%`,
        height: `${height}px`,
        objectFit: "cover",
        borderRadius: "8px",
        boxShadow: "0 4px 8px rgba(0, 0, 0, 0.1)",
      }}
      onError={(e) => (e.currentTarget.src = fallBackSrc)}
    />
  );
}

export default MangaImage;
