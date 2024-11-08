import { createContext } from "react";

import { Extension } from "../types/extension.ts";

export const ExtensionsContext = createContext<Extension[]>([]);
