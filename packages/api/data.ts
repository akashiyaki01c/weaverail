import { invoke } from "@tauri-apps/api/core";
import { StationId, Station, TrackId, Track, LineSegmentId, LineSegment, LineId, Line, TrainTypeId, TrainType, TemplateTrainId, TemplateTrain, TemplateTrainSegmentId, TemplateTrainSegment, TemplateTrainStationId, TemplateTrainStation, TemplateTrainSection, TimetableId, Timetable, TrainId, Train, ResultWeftTrain, DiagramLogicalConvert, ResultSvg, DiagramRoot, Time, DiagramViewSettingsId } from "@weaverail/types";

/** WeaverailのAPI群を表すオブジェクト */
export interface WeaverailDataApi {
	getRoot(): Promise<DiagramRoot>;
	getStations(): Promise<{ [key in StationId]: Station }>;
	getTracks(): Promise<{ [key in TrackId]: Track }>;
	getSegments(): Promise<{ [key in LineSegmentId]: LineSegment }>;
	getLines(): Promise<{ [key in LineId]: Line }>;
	getTrainTypes(): Promise<{ [key in TrainTypeId]: TrainType }>;
	getTemplateTrains(): Promise<{ [key in TemplateTrainId]: TemplateTrain }>;
	newTemplateTrainSegmentId(): Promise<TemplateTrainSegmentId>;
	newTemplateTrainStationId(): Promise<TemplateTrainStationId>;
	addTemplateTrainSegment(templateTrainId: TemplateTrainId, segment: TemplateTrainSegment, station: TemplateTrainStation): Promise<void>;
	popFrontTemplateTrainSegment(templateTrainId: TemplateTrainId): Promise<void>;
	popBackTemplateTrainSegment(templateTrainId: TemplateTrainId): Promise<void>;
	updateTemplateTrainSection(templateTrainId: TemplateTrainId, index: number, section: TemplateTrainSection): Promise<void>;
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
	debugInsertTrain(timetableId: TimetableId, departureTime: Time): Promise<void>;
	runScript(source: string): Promise<ScriptExecutionResult>;
	resetScriptSession(): Promise<void>;
}

export interface ScriptExecutionResult {
	success: boolean;
	output: string;
	changed_objects: number;
	project_version: number;
	elapsed_ms: number;
	error: ScriptError | null;
}

export interface ScriptError {
	kind: string;
	message: string;
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

	async newTemplateTrainSegmentId(): Promise<TemplateTrainSegmentId> {
		return await invoke("new_template_train_segment_id");
	}

	async newTemplateTrainStationId(): Promise<TemplateTrainStationId> {
		return await invoke("new_template_train_station_id");
	}

	async addTemplateTrainSegment(templateTrainId: TemplateTrainId, segment: TemplateTrainSegment, station: TemplateTrainStation): Promise<void> {
		return await invoke("add_template_train_segment", { templateTrainId, segment, station });
	}

	async popFrontTemplateTrainSegment(templateTrainId: TemplateTrainId): Promise<void> {
		return await invoke("pop_front_template_train_segment", { templateTrainId });
	}

	async popBackTemplateTrainSegment(templateTrainId: TemplateTrainId): Promise<void> {
		return await invoke("pop_back_template_train_segment", { templateTrainId });
	}

	async updateTemplateTrainSection(templateTrainId: TemplateTrainId, index: number, section: TemplateTrainSection): Promise<void> {
		return await invoke("update_template_train_section", { templateTrainId, index, section });
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

	async debugInsertTrain(timetableId: TimetableId, startingDepartureTime: Time): Promise<void> {
		return await invoke("debug_insert_train", { timetableId, startingDepartureTime });
	}

	async runScript(source: string): Promise<ScriptExecutionResult> {
		return await invoke("run_script", { source });
	}

	async resetScriptSession(): Promise<void> {
		return await invoke("reset_script_session");
	}

	constructor() {

	}
}