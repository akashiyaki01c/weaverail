import ReactDOM from "react-dom/client";
import App from "./App";
import { createTheme, MantineProvider } from "@mantine/core";
import { ExtensionProvider } from "./ExtensionContext";
import "@mantine/core/styles.css";

const theme = createTheme({
  fontFamily: "Roboto, Source Han Sans, sans-serif",
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <MantineProvider theme={theme}>
    <ExtensionProvider>
      <App />
    </ExtensionProvider>
  </MantineProvider>,
);
