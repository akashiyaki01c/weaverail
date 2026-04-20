import { invoke } from "@tauri-apps/api/core";
import { DiagramRoot } from "../types";
import { StationApi } from "./station";

/** WeaverailのAPI群を表すオブジェクト */
export class WeaverailOpsApi {
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

	station: StationApi;

	constructor() {
		this.station = new StationApi();
	}
}