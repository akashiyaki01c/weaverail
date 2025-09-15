import { Root } from "../sharpdia-model/Root";
import { Timetable } from "../sharpdia-model/Timetable";

export class TimetableService {
	static update(root: Root, index: number, data: Timetable): Root {
		if (index < 0 || root.timetables.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const timetables = [...root.timetables];
		timetables[index] = data;
		return { ...root, timetables };
	}
	static insert(root: Root, index: number, data: Timetable): Root {
		if (index < 0 || root.timetables.length < index) {
			throw new RangeError("存在しないインデックス");
		}
		const timetables = [...root.timetables];
		timetables.splice(index, 0, data);
		return { ...root, timetables };
	}
	static append(root: Root, data: Timetable): Root {
		return this.insert(root, root.timetables.length, data);
	}
	static delete(root: Root, index: number): Root {
		if (index < 0 || root.timetables.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const timetables = [...root.timetables];
		timetables.splice(index, 1);
		return { ...root, timetables };
	}
	static findById(root: Root, id: string): Timetable | undefined {
		return root.timetables.find(s => s.id === id);
	}
	static findIndexById(root: Root, id: string): number {
		return root.timetables.findIndex(s => s.id === id);
	}
}