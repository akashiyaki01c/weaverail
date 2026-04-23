import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "../api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { useEffect, useState } from "react";
import { Station } from "@weaverail/types";

function StationViewer() {
  const { manager } = useExtensionManager();
  const [stations, setStations] = useState<{ [x: string]: Station }>();

  useEffect(() => {
    (async () => {
      setStations(await manager.api.data.getStations());
    })();
  }, []);

  const components = [];
  for (const key in stations) {
    if (!Object.hasOwn(stations, key)) continue;

    const element = stations[key];

    components.push(<div key={element.id}>{element.name} ({element.id})</div>);
  }

  return <div>{components}</div>;
}

export class StationViewerExtension implements WeaverailExtension {
  id: string = "weaverail.core.station-viewer";
  metadata = { name: "コア拡張機能", description: "" };
  init(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.station-viewer.sidebar-panel",
      label: "",
      slot: "sidebar",
      render: function (): React.ReactNode {
        return <StationViewer />;
      },
    });
  }
  destroy(): void {}
}
