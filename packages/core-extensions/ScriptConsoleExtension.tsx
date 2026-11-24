import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { ScriptConsole } from "./ScriptConsole";

export class ScriptConsoleExtension implements WeaverailExtension {
  id = "weaverail.core.script-console";
  metadata = { name: "スクリプトコンソール", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.script-console.main-panel",
      label: "スクリプト",
      slot: "main",
      render: () => <ScriptConsole />,
    });
  }
  deactivate(): void {}
}
