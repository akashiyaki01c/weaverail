import { Segment } from "../sharpdia-model/Line";
import { Root } from "../sharpdia-model/Root";

export class SegmentService {
	static update(root: Root, lineIndex: number, segmentIndex: number, data: Segment): Root {
		if (lineIndex < 0 || root.lines.length <= lineIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (segmentIndex < 0 || root.lines[lineIndex].segments.length <= segmentIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const segments = [...root.lines[lineIndex].segments];
		segments[segmentIndex] = data;
		const newLine = { ...root.lines[lineIndex], segments: segments };
		const lines = [...root.lines];
		lines[lineIndex] = newLine;
		return { ...root, lines };
	}
	static insert(root: Root, lineIndex: number, segmentIndex: number, data: Segment): Root {
		if (lineIndex < 0 || root.lines.length <= lineIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (segmentIndex < 0 || root.lines[lineIndex].segments.length <= segmentIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const segments = [...root.lines[lineIndex].segments];
		segments.splice(segmentIndex, 0, data);
		const newLine = { ...root.lines[lineIndex], segments: segments };
		const lines = [...root.lines];
		lines[lineIndex] = newLine;
		return { ...root, lines };
	}
	static append(root: Root, lineIndex: number, data: Segment): Root {
		return this.insert(root, lineIndex, root.lines.length, data);
	}
	static delete(root: Root, lineIndex: number, segmentIndex: number): Root {
		if (lineIndex < 0 || root.lines.length <= lineIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (segmentIndex < 0 || root.lines[lineIndex].segments.length <= segmentIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const segments = [...root.lines[lineIndex].segments];
		segments.splice(segmentIndex, 1);
		const newLine = { ...root.lines[lineIndex], segments: segments };
		const lines = [...root.lines];
		lines[lineIndex] = newLine;
		return { ...root, lines };
	}
	static findById(root: Root, lineIndex: number, id: string): Segment | undefined {
		return root.lines[lineIndex].segments.find(s => s.id === id);
	}
	static findIndexById(root: Root, lineIndex: number, id: string): number {
		return root.lines[lineIndex].segments.findIndex(s => s.id === id);
	}
}