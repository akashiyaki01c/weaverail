import { create } from "zustand";
import { Station } from "./sharpdia-model/Station";
import { immer } from "zustand/middleware/immer";
import { OuDia_DiagramFile } from "./oudia-parser/oudia-model/DiagramFile";
import { ParseOud2 } from "./oudia-parser/lexer";
import oudText from "./testdata/kh.oud2?raw";

const root = ParseOud2(oudText);
const diagramFile = new OuDia_DiagramFile(root);
const stations = diagramFile.rosen.eki.map((v) => Station.fromOuDia(v));

export interface Store {
	stations: Station[];
	updateStation: (index: number, data: Station) => void;
	setStations: (fn: (prev: Station[]) => Station[]) => void;
}

let useGlobalState = create(immer<Store>((set, get) => ({
	stations,

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
	})
})));

export default useGlobalState;