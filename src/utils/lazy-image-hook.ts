import { LegacyRef, useEffect, useRef, useState } from "react";

export default function useLazyImage({
  onChange = (_inView: boolean) => {},
  offset = "0px",
}: {
  onChange?: (inView: boolean) => void;
  offset?: string;
}) {
  const [inView, setInView] = useState(false);
  const observerRef = useRef<IntersectionObserver>();
  const targetRef = useRef((() => {
    const target = document.createElement("div");
    target.setAttribute("image-lazy-loading-detector", "");
    target.style.position = "absolute";
    target.style.bottom = "0px";
    target.style.width = "0px";
    target.style.height = `calc(100% + ${offset} * 2)`;
    target.style.top = `calc(${offset} * -1)`;
    return target;
  })());

  const containerRef: LegacyRef<HTMLElement> = (container: HTMLElement) => {
    if (container) {
      container.append(targetRef.current);
      container.style.position = "relative";
    }
  };

  useEffect(() => {
    onChange(inView);
  }, [inView]);

  useEffect(() => {
    const observer = observerRef.current;

    if (observer) {
      observer.disconnect();
    }

    observerRef.current = new IntersectionObserver(([entry]) => {
      setInView(entry.isIntersecting);
    });

    observerRef.current.observe(targetRef.current);

    return () => {
      observerRef.current?.disconnect();
    };
  }, []);

  return { containerRef, inView };
}
