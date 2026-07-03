import { invoke } from "@tauri-apps/api/core";
import { StationId, Station, TrackId, Track, LineSegmentId, LineSegment, LineId, Line, TrainTypeId, TrainType, TemplateTrainId, TemplateTrain, TimetableId, Timetable, TrainId, Train, ResultWeftTrain, DiagramLogicalConvert, DiagramViewSettings, ResultSvg, DiagramRoot, Time, DiagramViewSettingsId } from "@weaverail/types";

/** WeaverailのAPI群を表すオブジェクト */
export interface WeaverailDataApi {
	getRoot(): Promise<DiagramRoot>;
	getStations(): Promise<{ [key in StationId]: Station }>;
	getTracks(): Promise<{ [key in TrackId]: Track }>;
	getSegments(): Promise<{ [key in LineSegmentId]: LineSegment }>;
	getLines(): Promise<{ [key in LineId]: Line }>;
	getTrainTypes(): Promise<{ [key in TrainTypeId]: TrainType }>;
	getTemplateTrains(): Promise<{ [key in TemplateTrainId]: TemplateTrain }>;
	getTimetables(): Promise<{ [key in TimetableId]: Timetable }>;
	getTrains(): Promise<{ [key in TrainId]: Train }>;
	getSvg(timetableId: TimetableId, viewSettings: DiagramViewSettingsId, settings: DiagramLogicalConvert, startTime: Time, endTime: Time): Promise<ResultSvg>;
	getWarpCoords(viewSettings: DiagramViewSettingsId): Promise<{ [key in LineSegmentId]: {
		upper_y: number,
		lower_y: number,
		segment_id: LineSegmentId,
		is_reversed: boolean,
	} }>;
	getWarpStations(viewSettingsId: DiagramViewSettingsId): Promise<{
		y_coord: number;
		station_id: StationId;
		name: string;
	}[]>;
	weave(timetableId: TimetableId): Promise<ResultWeftTrain[]>;
}

/** WeaverailのAPI群を表すオブジェクト */
export class WeaverailDataApiObject implements WeaverailDataApi {

	async getRoot(): Promise<DiagramRoot> {
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

	async weave(timetableId: TimetableId): Promise<ResultWeftTrain[]> {
		return await invoke("weave", { timetableId });
	}

	async getSvg(timetableId: TimetableId, viewSettingsId: DiagramViewSettingsId, settings: DiagramLogicalConvert, startTime: Time, endTime: Time): Promise<ResultSvg> {
		return await invoke("get_svg", { timetableId, viewSettingsId, settings, startTime, endTime });
	}

	async getWarpCoords(viewSettingsId: DiagramViewSettingsId): Promise<{ [key in LineSegmentId]: {
		upper_y: number;
		lower_y: number;
		segment_id: LineSegmentId;
		is_reversed: boolean;
	}; }> {
		return await invoke("get_warp_coords", { viewSettingsId });
	}

	async getWarpStations(viewSettingsId: DiagramViewSettingsId): Promise<{
		y_coord: number;
		station_id: StationId;
		name: string;
	}[]> {
		return await invoke("get_warp_stations", { viewSettingsId });
	}

	constructor() {

	}
}