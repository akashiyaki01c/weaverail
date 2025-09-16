import { Root } from "../sharpdia-model/Root";
import { DiagramLine } from "../sharpdia-model/DiagramLine";

export class DiagramLineService {
	static update(root: Root, index: number, data: DiagramLine): Root {
		if (index < 0 || root.diagramLines.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const diagramLines = [...root.diagramLines];
		diagramLines[index] = data;
		return { ...root, diagramLines };
	}
	static insert(root: Root, index: number, data: DiagramLine): Root {
		if (index < 0 || root.diagramLines.length < index) {
			throw new RangeError("存在しないインデックス");
		}
		const diagramLines = [...root.diagramLines];
		diagramLines.splice(index, 0, data);
		return { ...root, diagramLines };
	}
	static append(root: Root, data: DiagramLine): Root {
		return this.insert(root, root.diagramLines.length, data);
	}
	static delete(root: Root, index: number): Root {
		if (index < 0 || root.diagramLines.length <= index) {
			throw new RangeError("存在しないインデックス");
		}
		const diagramLines = [...root.diagramLines];
		diagramLines.splice(index, 1);
		return { ...root, diagramLines };
	}
	static findById(root: Root, id: string): DiagramLine | undefined {
		return root.diagramLines.find(s => s.id === id);
	}
	static findIndexById(root: Root, id: string): number {
		return root.diagramLines.findIndex(s => s.id === id);
	}
}