import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ExtensionProvider } from "@weaverail/extensions";
import { createTheme, MantineProvider } from "@mantine/core";

const theme = createTheme({
  fontFamily: 'Roboto, Source Han Sans, sans-serif',
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <MantineProvider theme={theme}>
      <ExtensionProvider>
        <App />
      </ExtensionProvider>
    </MantineProvider>
  </React.StrictMode>,
);
