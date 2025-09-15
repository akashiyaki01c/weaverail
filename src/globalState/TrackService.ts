import { Root } from "../sharpdia-model/Root";
import { Track } from "../sharpdia-model/Station";

export class TrackService {
	static update(root: Root, stationIndex: number, trackIndex: number, data: Track): Root {
		if (stationIndex < 0 || root.stations.length <= stationIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (trackIndex < 0 || root.stations[stationIndex].tracks.length <= trackIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const tracks = [...root.stations[stationIndex].tracks];
		tracks[trackIndex] = data;
		const newStation = { ...root.stations[stationIndex], tracks };
		const stations = [...root.stations];
		stations[stationIndex] = newStation;
		return { ...root, stations };
	}
	static insert(root: Root, stationIndex: number, trackIndex: number, data: Track): Root {
		if (stationIndex < 0 || root.stations.length <= stationIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (trackIndex < 0 || root.stations[stationIndex].tracks.length < trackIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const tracks = [...root.stations[stationIndex].tracks];
		tracks.splice(trackIndex, 0, data);
		const newStation = { ...root.stations[stationIndex], tracks };
		const stations = [...root.stations];
		stations[stationIndex] = newStation;
		return { ...root, stations };
	}
	static append(root: Root, stationIndex: number, data: Track): Root {
		return this.insert(root, stationIndex, root.stations[stationIndex].tracks.length, data);
	}
	static delete(root: Root, stationIndex: number, trackIndex: number): Root {
		if (stationIndex < 0 || root.stations.length <= stationIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (trackIndex < 0 || root.stations[stationIndex].tracks.length <= trackIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const tracks = [...root.stations[stationIndex].tracks];
		tracks.splice(trackIndex, 1);
		const newStation = { ...root.stations[stationIndex], tracks };
		const stations = [...root.stations];
		stations[stationIndex] = newStation;
		return { ...root, stations };
	}
	static findById(root: Root, stationIndex: number, id: string): Track | undefined {
		return root.stations[stationIndex].tracks.find(s => s.id === id);
	}
	static findIndexById(root: Root, stationIndex: number, id: string): number {
		return root.stations[stationIndex].tracks.findIndex(s => s.id === id);
	}
}