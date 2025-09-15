import { Root } from "../sharpdia-model/Root";
import { Station } from "../sharpdia-model/Station";

export class StationService {
	static update(root: Root, index: number, data: Station): Root {
		if (index < 0 || root.stations.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const stations = [...root.stations];
		stations[index] = data;
		return { ...root, stations };
	}
	static insert(root: Root, index: number, data: Station): Root {
		if (index < 0 || root.stations.length < index) {
			throw new RangeError("存在しないインデックス");
		}
		const stations = [...root.stations];
		stations.splice(index, 0, data);
		return { ...root, stations };
	}
	static append(root: Root, data: Station): Root {
		return this.insert(root, root.stations.length, data);
	}
	static delete(root: Root, index: number): Root {
		if (index < 0 || root.stations.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const stations = [...root.stations];
		stations.splice(index, 1);
		return { ...root, stations };
	}
	static findById(root: Root, id: string): Station | undefined {
		return root.stations.find(s => s.id === id);
	}
	static findIndexById(root: Root, id: string): number {
		return root.stations.findIndex(s => s.id === id);
	}
}