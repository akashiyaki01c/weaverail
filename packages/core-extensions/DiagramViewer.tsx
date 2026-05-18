import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { useEffect, useState } from "react";
import { ResultSvg, ResultWeftTrain } from "@weaverail/types";

function DiagramViewer() {
  const { manager } = useExtensionManager();
  const [svg, setSvg] = useState<ResultSvg>();

  useEffect(() => {
    (async () => {
      const root = await manager.api.data.getRoot();
      console.log(root);
      const timetableId = Object.keys(root.timetables)[0];

      const diagramViewSettingsId = Object.keys(root.diagram_view_settings)[0];
      const diagramViewSettings =
        root.diagram_view_settings[diagramViewSettingsId];

      setSvg(
        await manager.api.data.getSvg(
          timetableId,
          diagramViewSettings,
          {
            scale_x: 1,
            scale_y: 1,
            offset_x: 0,
            offset_y: 0,
          },
          12 * 60 * 60,
          12 * 60 * 60 + 30 * 60,
        ),
      );
    })();
  }, []);

  return (
    <>
      <svg width={24 * 60 * 60} height={10000}>
        {svg?.trains.map((t) => (
          <path
            stroke="black"
            fill="none"
            key={`${t.train_id}/}`}
            d={t.path_string}
          ></path>
        ))}
      </svg>
    </>
  );
}

export class DiagramViewerExtension implements WeaverailExtension {
  id: string = "weaverail.core.diagram-viewer";
  metadata = { name: "コア拡張機能", description: "" };
  init(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.diagram-viewer.main-panel",
      label: "",
      slot: "main",
      render: function (): React.ReactNode {
        return <DiagramViewer />;
      },
    });
  }
  destroy(): void {}
}
