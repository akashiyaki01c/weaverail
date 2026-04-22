import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ExtensionProvider } from "@weaverail/extensions";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ExtensionProvider>
      <App />
    </ExtensionProvider>
  </React.StrictMode>,
);
