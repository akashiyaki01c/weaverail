import { createContext, useContext } from "react";
import { ExtensionManager } from "@weaverail/extensions";

const globalManager = new ExtensionManager();

const ExtensionContext = createContext<{ manager: ExtensionManager }>({
  manager: globalManager,
});

export const ExtensionProvider = ({ children }: { children: React.ReactNode }) => {
  return (
    <ExtensionContext.Provider value={{ manager: globalManager }}>
      {children}
    </ExtensionContext.Provider>
  );
};

export const useExtensionManager = () => useContext(ExtensionContext);