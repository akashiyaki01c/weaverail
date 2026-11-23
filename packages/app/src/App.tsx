import "flexlayout-react/style/light.css";
import { useEffect, useState } from "react";
import {
  IJsonTabNode,
  Layout,
  Model,
  TabNode,
} from "flexlayout-react";
import { InitLayout } from "./initLayout";
import {
  CoreExtensions,
  LineViewerExtension,
  StationViewerExtension,
  TrackInspectorExtension,
  TimetableViewerExtension,
  TemplateTrainViewerExtension,
  TrainTypeViewerExtension,
} from "@weaverail/core-extensions";
import { useExtensionManager } from "./ExtensionContext";
import { DiagramViewerExtension } from "@weaverail/core-extensions/DiagramViewer";

const FlexLayout = Layout as any;

function App() {
  const [model, setModel] = useState<Model>(InitLayout);
  const { manager } = useExtensionManager();

  useEffect(() => {
    const unbindPanelRegistered = manager.on("panel-registered", (payload) => {
      setModel((currentModel) => {
        if (payload.slot === "main") {
          const json = currentModel.toJson();
          json.layout.children[0].children.push({
            type: "tab",
            component: payload.panelId,
            id: payload.panelId,
            children: [],
          });
          return Model.fromJson(json);
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
    });

    const unbindPanelUnregistered = manager.on("panel-unregistered", (payload) => {
      setModel((currentModel) => {
        const json = currentModel.toJson();
        json.layout.children[0].children =
          json.layout.children[0].children.filter(
            (node) => node.id !== payload.panelId,
          );
        json.borders?.forEach((border) => {
          border.children = border.children.filter(
            (node) => node.id !== payload.panelId,
          );
        });
        return Model.fromJson(json);
      });
    });

    void Promise.all([
      manager.addExtension(new CoreExtensions()),
      manager.addExtension(new StationViewerExtension()),
      manager.addExtension(new TrackInspectorExtension()),
      manager.addExtension(new LineViewerExtension()),
      manager.addExtension(new TrainTypeViewerExtension()),
      manager.addExtension(new TemplateTrainViewerExtension()),
      manager.addExtension(new TimetableViewerExtension()),
      manager.addExtension(new DiagramViewerExtension()),
    ]).catch((error: unknown) => {
      console.error("Failed to activate extension", error);
    });

    return () => {
      unbindPanelRegistered();
      unbindPanelUnregistered();
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
