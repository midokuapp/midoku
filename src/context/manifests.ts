import { createContext, useContext } from "react";
import { Manifest } from "../types/manifest.ts";

export const ManifestsContext = createContext<Manifest[]>([]);

export function useManifests(): {
  manifests: Manifest[];
  setManifests: (extensions: Manifest[]) => void;
} {
  return useContext(ManifestsContext);
}
