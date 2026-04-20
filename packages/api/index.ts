import { WeaverailDataApi, WeaverailDataApiObject } from "./data";
import { WeaverailOpsApi, WeaverailOpsApiObject } from "./ops";
import { WeaverailUiApi, WeaverailUiApiObject } from "./ui";

export * from "./data";
export * from "./extensions";
export * from "./ops";

export interface WeaverailExtension {
	registerView(slot: "siderbar" | "main" | "inspector", component: React.ReactNode): void;
}

export interface WeaverailApi {
	readonly ops: WeaverailOpsApi;
	readonly data: WeaverailDataApi;
	readonly ui: WeaverailUiApi;
}

export class WeaverailApiObject implements WeaverailApi {
	readonly ops: WeaverailOpsApi;
	readonly data: WeaverailDataApi;
	readonly ui: WeaverailUiApi;

	constructor() {
		this.ops = new WeaverailOpsApiObject();
		this.data = new WeaverailDataApiObject();
		this.ui = new WeaverailUiApiObject();
	}
}