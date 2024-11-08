import {
  createContext,
  createElement,
  useContext,
  useEffect,
  useState,
} from "react";
import { storeService } from "../services/store.service.ts";

export const RepositoryUrlContext = createContext<string>([]);

export const RepositoryUrlContextProvider: React.FC = (
  props: React.PropsWithChildren,
) => {
  const [repositoryUrl, setRepositoryUrl] = useState<string>("");

  return createElement(
    RepositoryUrlContext.Provider,
    { value: { repositoryUrl, setRepositoryUrl } },
    props.children,
  );
};

export function useRepositoryUrl(): {
  repositoryUrl: string;
  setRepositoryUrl: (repositoryUrl: string) => void;
} {
  const { repositoryUrl, setRepositoryUrl } = useContext(RepositoryUrlContext);

  useEffect(() => {
    storeService.get<string>("extensionRepositoryUrl").then((repositoryUrl) => {
      setRepositoryUrl(repositoryUrl);
    });
  }, []);

  return {
    repositoryUrl,
    setRepositoryUrl: (repositoryUrl: string) => {
      setRepositoryUrl(repositoryUrl);
      storeService.set("extensionRepositoryUrl", repositoryUrl);
    },
  };
}
