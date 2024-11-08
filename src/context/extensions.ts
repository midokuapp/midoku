import { createContext, useContext, useEffect } from "react";

import { Extension } from "../types/extension.ts";
import { getExtensions } from "../services/tauri.service.ts";

export const ExtensionsContext = createContext<Extension[]>([]);

export function useExtensions(): {
  extensions: Extension[];
  setExtensions: (extensions: Extension[]) => void;
} {
  const { extensions, setExtensions } = useContext(ExtensionsContext);

  useEffect(() => {
    getExtensions().then(setExtensions);
  }, []);

  return { extensions, setExtensions };
}
