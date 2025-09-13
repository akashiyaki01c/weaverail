import { create } from "zustand";
import { Station } from "./sharpdia-model/Station";
import { immer } from "zustand/middleware/immer";
import { Line, Segment } from "./sharpdia-model/Line";

export interface Store {
	stations: Station[];
	updateStation: (index: number, data: Station) => void;
	setStations: (fn: (prev: Station[]) => Station[]) => void;
	deleteStation: (index: number) => void;

	lines: Line[];
	updateLine: (index: number, data: Line) => void;
	setLines: (fn: (prev: Line[]) => Line[]) => void;
	setSegments: (index: number, fn: (prev: Segment[]) => Segment[]) => void;
}

let useGlobalState = create(immer<Store>((set, _) => ({
	stations: [],

	updateStation: (index: number, data: Station) => set((state) => {
		if (state.stations[index]) {
			Object.assign(state.stations[index], data);
		} else {
			state.stations.push(Station.default());
			Object.assign(state.stations[index], data);
		}
		state.stations = [...state.stations];
	}),
	setStations: (data: (prev: Station[]) => Station[]) => set((state) => {
		state.stations = [...data(state.stations)];
	}),
	deleteStation: (index: number) => set((state) => {
		const sta = state.stations[index];
		// 路線から参照
		const isReferenced = state.lines
			.some(v => v.segments.some(v => v.startId === sta.id || v.endId === sta.id))
		if (isReferenced) {
			return;
		}
		state.stations.splice(index, 1);
	}),

	lines: [],
	updateLine: (index: number, data: Line) => set((state) => {
		if (state.lines[index]) {
			Object.assign(state.lines[index], data);
		} else {
			state.lines.push(Line.default());
			Object.assign(state.lines[index], data);
		}
		state.lines = [...state.lines];
	}),
	setLines: (data: (prev: Line[]) => Line[]) => set((state) => {
		state.lines = [...data(state.lines)];
	}),
	setSegments: (index: number, data: (prev: Segment[]) => Segment[]) => set((state) => {
		state.lines[index].segments = [...data(state.lines[index].segments)];
	}),
})));

export default useGlobalState;