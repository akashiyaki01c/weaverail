import { invoke } from "@tauri-apps/api/core";
import { DiagramRoot, Station, StationId } from "../types";

/** WeaverailのAPI群を表すオブジェクト */
export class WeaverailApi {
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
	public async new_station_id(): Promise<StationId> {
		return await invoke("new_station_id");
	}
	public async add_station(station: Station) {
		return await invoke("add_station", { station });
	}

	constructor() { }
}