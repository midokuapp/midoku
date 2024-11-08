import { createContext, useContext } from "react";

import { Extension } from "../types/extension.ts";

export const ExtensionsContext = createContext<Extension[]>([]);

export function useExtensions(): {
  extensions: Extension[];
  setExtensions: (extensions: Extension[]) => void;
} {
  return useContext(ExtensionsContext);
}
