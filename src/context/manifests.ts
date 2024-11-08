import { createContext, createElement, useContext, useState } from "react";
import { Manifest } from "../types/manifest.ts";

export const ManifestsContext = createContext<Manifest[]>([]);

export const ManifestsContextProvider: React.FC = (
  props: React.PropsWithChildren,
) => {
  const [manifests, setManifests] = useState<Manifest[]>([]);

  return createElement(
    ManifestsContext.Provider,
    { value: { manifests, setManifests } },
    props.children,
  );
};

export function useManifests(): {
  manifests: Manifest[];
  setManifests: (extensions: Manifest[]) => void;
} {
  return useContext(ManifestsContext);
}
