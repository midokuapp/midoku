import { useInView } from "react-intersection-observer";

type LazyImageProps = {
  src: string;
  alt: string;
  onLoad?: () => void;
  onChange?: (inView: boolean) => void;
  className?: string;
  offset?: string;
};

export default function LazyImage({
  src,
  alt,
  onLoad,
  onChange,
  className,
  offset = "0px",
}: LazyImageProps) {
  const { ref, inView } = useInView({ onChange });

  return (
    <div className="w-full h-full skeleton relative">
      {inView && (
        <img
          src={src}
          alt={alt}
          onLoad={onLoad}
          className={className}
        />
      )}
      <div
        ref={ref}
        image-lazy-loading-detector=""
        className="absolute"
        style={{
          height: `calc(100% + ${offset} * 2)`,
          top: `calc(${offset} * -1)`,
        }}
      />
    </div>
  );
}
