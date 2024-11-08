import { createContext, useContext } from "react";
import { storeService } from "../services/store.service.ts";

export const RepositoryUrlContext = createContext<string>([]);

export function useRepositoryUrl(): {
  repositoryUrl: string;
  setRepositoryUrl: (repositoryUrl: string) => void;
} {
  const { repositoryUrl, setRepositoryUrl } = useContext(RepositoryUrlContext);

  storeService.get<string>("extensionRepositoryUrl").then((repositoryUrl) => {
    setRepositoryUrl(repositoryUrl);
  });

  return {
    repositoryUrl: repositoryUrl,
    setRepositoryUrl: (repositoryUrl: string) => {
      setRepositoryUrl(repositoryUrl);
      storeService.set("extensionRepositoryUrl", repositoryUrl);
    },
  };
}
