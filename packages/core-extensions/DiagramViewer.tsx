import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { useEffect, useState } from "react";
import { ResultWeftTrain } from "@weaverail/types";

function DiagramViewer() {
	const { manager } = useExtensionManager();
	const [trains, setTrains] = useState<ResultWeftTrain[]>([]);
	useEffect(() => {
		(async () => {
			const timetables = await manager.api.data.getTimetables();
			console.log(timetables);
			const key = Object.keys(timetables)[0];
			setTrains(await manager.api.data.weave(Number.parseInt(key)));
		})();
	}, []);

	return <>{JSON.stringify(trains)}</>;
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
