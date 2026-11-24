import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "../api";
import { TestViwer } from "./TestViewer";

export * from "./StationViewer";
export * from "./LineViewer";
export * from "./LineSegmentInspector";
export * from "./TrainTypeViewer";
export * from "./TemplateTrainViewer";
export * from "./TimetableViewer";

export class CoreExtensions implements WeaverailExtension {
	id: string = "weaverail.core-extensions";
	metadata = { name: "コア拡張機能", description: "" };
	activate(api: WeaverailApi) {
		api.ui.registerPanel({
			id: "weaverail.core-extensions.main-panel",
			label: "メイン",
			slot: "main",
			render: function (): React.ReactNode {
				return <></>
			}
		})
		api.ui.registerPanel({
			id: "weaverail.core-extensions.toolbar-panel",
			label: "ツール",
			slot: "sidebar",
			render: function (): React.ReactNode {
				return <><TestViwer /></>
			}
		})
		api.ui.registerPanel({
			id: "weaverail.core-extensions.toolbar2-panel",
			label: "サブ",
			slot: "sidebar",
			render: function (): React.ReactNode {
				return <>さぶ2だよ〜(拡張機能側)</>
			}
		})
	}
	deactivate(): void {
	
	}
}