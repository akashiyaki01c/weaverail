import { invoke } from "@tauri-apps/api/core";
import { Station, StationId, Track, TrackId } from "@weaverail/types";

/** 駅に関するAPI群を表すオブジェクト */
export class StationApi {
	public async new_station_id(): Promise<StationId> {
		return await invoke("new_station_id");
	}
	public async add_station(station: Station) {
		return await invoke("add_station", { station });
	}
	public async remove_station(stationId: StationId) {
		return await invoke("remove_station", { stationId });
	}
	public async rename_station(stationId: StationId, name: string) {
		return await invoke("rename_station", { stationId, name });
	}
	public async new_track_id(): Promise<TrackId> {
		return await invoke("new_track_id");
	}
	public async add_track(track: Track) {
		return await invoke("add_track", { track });
	}
	public async remove_track(trackId: TrackId) {
		return await invoke("remove_track", { trackId });
	}
	public async rename_track(trackId: TrackId, name: string) {
		return await invoke("rename_track", { trackId, name });
	}
	
	constructor() {}
}