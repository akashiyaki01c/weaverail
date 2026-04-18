import { invoke } from "@tauri-apps/api/core";
import { Station, StationId } from "../types";

/** 駅に関するAPI群を表すオブジェクト */
export class StationApi {
	public async new_station_id(): Promise<StationId> {
		return await invoke("new_station_id");
	}
	public async add_station(station: Station) {
		return await invoke("add_station", { station });
	}
	
	constructor() {}
}