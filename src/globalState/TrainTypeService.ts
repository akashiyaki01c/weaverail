import { Root } from "../sharpdia-model/Root";
import { TrainType } from "../sharpdia-model/TrainType";

export class TrainTypeService {
	static update(root: Root, index: number, data: TrainType): Root {
		if (index < 0 || root.trainTypes.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const trainTypes = [...root.trainTypes];
		trainTypes[index] = data;
		return { ...root, trainTypes };
	}
	static insert(root: Root, index: number, data: TrainType): Root {
		if (index < 0 || root.trainTypes.length < index) {
			throw new RangeError("存在しないインデックス");
		}
		const trainTypes = [...root.trainTypes];
		trainTypes.splice(index, 0, data);
		return { ...root, trainTypes };
	}
	static append(root: Root, data: TrainType): Root {
		return this.insert(root, root.trainTypes.length, data);
	}
	static delete(root: Root, index: number): Root {
		if (index < 0 || root.trainTypes.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const trainTypes = [...root.trainTypes];
		trainTypes.splice(index, 1);
		return { ...root, trainTypes };
	}
	static findById(root: Root, id: string): TrainType | undefined {
		return root.trainTypes.find(s => s.id === id);
	}
	static findIndexById(root: Root, id: string): number {
		return root.trainTypes.findIndex(s => s.id === id);
	}
}