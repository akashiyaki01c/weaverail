import { invoke } from "@tauri-apps/api/core";
import { StationId, Station, TrackId, Track, LineSegmentId, LineSegment, LineId, Line, TrainTypeId, TrainType, TemplateTrainId, TemplateTrain, TimetableId, Timetable, TrainId, Train } from "../types";

/** WeaverailのAPI群を表すオブジェクト */
export class WeaverailDataApi {

	async getRoot(): Promise<{ [key in StationId]: Station }> {
		return await invoke("get_root");
	}
	
	async getStations(): Promise<{ [key in StationId]: Station }> {
		return await invoke("get_stations");
	}

	async getTracks(): Promise<{ [key in TrackId]: Track }> {
		return await invoke("get_tracks");
	}

	async getSegments(): Promise<{ [key in LineSegmentId]: LineSegment }> {
		return await invoke("get_segments");
	}

	async getLines(): Promise<{ [key in LineId]: Line }> {
		return await invoke("get_lines");
	}

	async getTrainTypes(): Promise<{ [key in TrainTypeId]: TrainType }> {
		return await invoke("get_train_types");
	}

	async getTemplateTrains(): Promise<{ [key in TemplateTrainId]: TemplateTrain }> {
		return await invoke("get_template_trains");
	}

	async getTimetables(): Promise<{ [key in TimetableId]: Timetable }> {
		return await invoke("get_timetables");
	}

	async getTrains(): Promise<{ [key in TrainId]: Train }> {
		return await invoke("get_trains");
	}

	constructor() {

	}
}