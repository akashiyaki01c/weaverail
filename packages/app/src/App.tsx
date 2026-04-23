import "flexlayout-react/style/light.css";
import { useEffect, useState } from "react";
import {
  Actions,
  DockLocation,
  IJsonTabNode,
  Layout,
  Model,
  TabNode,
} from "flexlayout-react";
import { InitLayout } from "./initLayout";
import { CoreExtensions, StationViewerExtension } from "@weaverail/core-extensions";
import { useExtensionManager } from "./ExtensionContext";

const FlexLayout = Layout as any;

function App() {
  const [model, setModel] = useState<Model>(InitLayout);
  const { manager } = useExtensionManager();

  useEffect(() => {
    const unbindPanelRegistered = manager.on("panel-registered", (payload) => {
      console.log(payload);
      setModel((currentModel) => {
        if (payload.slot === "main") {
          currentModel.doAction(
            Actions.addNode(
              {
                type: "tab",
                component: payload.panelId,
                id: payload.panelId,
              },
              "main",
              DockLocation.CENTER,
              -1,
            ),
          );
          return Model.fromJson(currentModel.toJson());
        } else {
          // border
          const json = currentModel.toJson();
          switch (payload.slot) {
            case "sidebar": {
              const tabs = json.borders?.find((v) => v.location === "left");
              tabs?.children.push({
                type: "tab",
                component: payload.panelId,
                id: payload.panelId,
              } satisfies IJsonTabNode);
              break;
            }
            case "inspector": {
              const tabs = json.borders?.find((v) => v.location === "right");
              tabs?.children.push({
                type: "tab",
                component: payload.panelId,
                id: payload.panelId,
              } satisfies IJsonTabNode);
              break;
            }
            case "bottom": {
              const tabs = json.borders?.find((v) => v.location === "bottom");
              tabs?.children.push({
                type: "tab",
                component: payload.panelId,
                id: payload.panelId,
              } satisfies IJsonTabNode);
              break;
            }
          }
          return Model.fromJson(json);
        }
      });
      console.log(model.toJson());
    });

    manager.addExtension(new CoreExtensions());
    manager.addExtension(new StationViewerExtension());

    return () => {
      unbindPanelRegistered();
    };
  }, [manager]);

  const factory = (node: TabNode): React.ReactNode => {
    const name = node.getComponent();
    const panel = manager.api.ui.getPanelById(name!);
    if (!panel) {
      return <>unknown {name}</>;
    }
    return panel.render();
  };

  return (
    <div className="h-screen w-screen overflow-hidden">
      <FlexLayout model={model} factory={factory} />
    </div>
  );
}

export default App;
