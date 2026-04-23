import { WeaverailDataApi, WeaverailDataApiObject } from "./data";
import { WeaverailOpsApi, WeaverailOpsApiObject } from "./ops";
import { WeaverailUiApi } from "./ui";

export * from "./data";
export * from "./ops";
export * from "./ui";

export interface WeaverailApi {
	readonly ops: WeaverailOpsApi;
	readonly data: WeaverailDataApi;
	readonly ui: WeaverailUiApi;
}

export class WeaverailApiObject implements WeaverailApi {
	readonly ops: WeaverailOpsApi;
	readonly data: WeaverailDataApi;
	readonly ui: WeaverailUiApi;

	constructor(ui: WeaverailUiApi) {
		this.ops = new WeaverailOpsApiObject();
		this.data = new WeaverailDataApiObject();
		this.ui = ui;
	}
}