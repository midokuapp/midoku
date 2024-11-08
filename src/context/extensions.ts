import {
  createContext,
  createElement,
  useContext,
  useEffect,
  useState,
} from "react";
import { Extension } from "../types/extension.ts";
import { getExtensions } from "../services/tauri.service.ts";

export const ExtensionsContext = createContext<Extension[]>([]);

export const ExtensionsContextProvider: React.FC = (
  props: React.PropsWithChildren,
) => {
  const [extensions, setExtensions] = useState<Extension[]>([]);

  return createElement(
    ExtensionsContext.Provider,
    { value: { extensions, setExtensions } },
    props.children,
  );
};

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
