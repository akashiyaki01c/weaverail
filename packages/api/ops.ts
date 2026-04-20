import { invoke } from "@tauri-apps/api/core";
import { DiagramRoot } from "../types";
import { StationApi } from "./station";

export interface WeaverailOpsApi {
	getRoot(): Promise<DiagramRoot>;
	redo(): void;
	undo(): void;
	redoable(): Promise<boolean>;
	undoable(): Promise<boolean> ;
	readonly station: StationApi;
}

/** WeaverailのAPI群を表すオブジェクト */
export class WeaverailOpsApiObject {
	public async getRoot(): Promise<DiagramRoot> {
		return await invoke("get_root");
	}
	public async redo() {
		await invoke("redo");
	}
	public async undo() {
		await invoke("undo");
	}
	public async redoable(): Promise<boolean> {
		return await invoke("redoable");
	}
	public async undoable(): Promise<boolean> {
		return await invoke("undoable");
	}

	readonly station: StationApi;

	constructor() {
		this.station = new StationApi();
	}
}