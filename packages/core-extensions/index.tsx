import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "../api";

export class CoreExtensions implements WeaverailExtension {
	id: string = "weaverail.core-extensions";
	metadata = { name: "コア拡張機能", description: "" };
	init(api: WeaverailApi) {
		api.ui.registerPanel({
			id: "weaverail.core-extensions.main-panel",
			label: "",
			slot: "main",
			render: function (): React.ReactNode {
				return <>めいんだよ〜(拡張機能側)</>
			}
		})
		api.ui.registerPanel({
			id: "weaverail.core-extensions.toolbar-panel",
			label: "",
			slot: "sidebar",
			render: function (): React.ReactNode {
				return <>さぶだよ〜(拡張機能側)</>
			}
		})
	}
	destroy(): void {
	
	}
}