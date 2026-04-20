import { WeaverailDataApi } from "./data";
import { WeaverailOpsApi } from "./ops";

export * from "./ops";

export interface WeaverailExtension {
	registerView(slot: "siderbar" | "main" | "inspector", component: React.ReactNode): void;
}

export class WeaverailApi {
	ops: WeaverailOpsApi;
	data: WeaverailDataApi;

	constructor() {
		this.ops = new WeaverailOpsApi();
		this.data = new WeaverailDataApi();
	}
}