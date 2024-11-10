import { LegacyRef, useEffect, useRef, useState } from "react";

export default function useInfiniteScroll({
  hasMore,
  onLoadMore,
  offset = "0px",
}: {
  hasMore: boolean;
  onLoadMore: () => Promise<unknown>;
  offset?: string;
}) {
  const [loading, setLoading] = useState(false);
  const observerRef = useRef<IntersectionObserver>();
  const targetRef = useRef((() => {
    const target = document.createElement("div");
    target.setAttribute("data-infinite-scroll-detector", "");
    target.style.position = "absolute";
    target.style.bottom = "0px";
    target.style.width = "0px";
    target.style.height = offset;
    return target;
  })());

  const containerRef: LegacyRef<HTMLElement> = (container: HTMLElement) => {
    if (container) {
      container.append(targetRef.current);
      container.style.position = "relative";
    }
  };

  useEffect(() => {
    const observer = observerRef.current;
    if (observer) {
      observer.disconnect();
    }

    observerRef.current = new IntersectionObserver(async ([entry]) => {
      if (entry.isIntersecting && !loading && hasMore) {
        setLoading(true);
        await onLoadMore();
        setLoading(false);
      }
    });

    observerRef.current.observe(targetRef.current);

    return () => {
      observerRef.current?.disconnect();
    };
  }, [hasMore, onLoadMore, loading]);

  return { containerRef, loading };
}
