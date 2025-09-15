import { Line } from "../sharpdia-model/Line";
import { Root } from "../sharpdia-model/Root";

export class LineService {
	static update(root: Root, index: number, data: Line): Root {
		if (index < 0 || root.lines.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const lines = [...root.lines];
		lines[index] = data;
		return { ...root, lines };
	}
	static insert(root: Root, index: number, data: Line): Root {
		if (index < 0 || root.lines.length < index) {
			throw new RangeError("存在しないインデックス");
		}
		const lines = [...root.lines];
		lines.splice(index, 0, data);
		return { ...root, lines };
	}
	static append(root: Root, data: Line): Root {
		return this.insert(root, root.lines.length, data);
	}
	static delete(root: Root, index: number): Root {
		if (index < 0 || root.lines.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const lines = [...root.lines];
		lines.splice(index, 1);
		return { ...root, lines };
	}
	static findById(root: Root, id: string): Line | undefined {
		return root.lines.find(s => s.id === id);
	}
	static findIndexById(root: Root, id: string): number {
		return root.lines.findIndex(s => s.id === id);
	}
}