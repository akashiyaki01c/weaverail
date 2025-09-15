import { Root } from "../sharpdia-model/Root";
import { Train } from "../sharpdia-model/Train";
import { SegmentService } from "./SegmentService";

export class TrainService {
	static update(root: Root, timetableIndex: number, trainIndex: number, data: Train): Root {
		if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (trainIndex < 0 || root.timetables[timetableIndex].trains.length <= trainIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const trains = [...root.timetables[timetableIndex].trains];
		trains[trainIndex] = data;
		const newStation = { ...root.timetables[timetableIndex], trains };
		const timetables = [...root.timetables];
		timetables[timetableIndex] = newStation;
		return { ...root, timetables };
	}
	static insert(root: Root, timetableIndex: number, trainIndex: number, data: Train): Root {
		if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (trainIndex < 0 || root.timetables[timetableIndex].trains.length < trainIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const trains = [...root.timetables[timetableIndex].trains];
		trains.splice(trainIndex, 0, data);
		const newStation = { ...root.timetables[timetableIndex], trains };
		const timetables = [...root.timetables];
		timetables[timetableIndex] = newStation;
		return { ...root, timetables };
	}
	static append(root: Root, timetableIndex: number, data: Train): Root {
		return this.insert(root, timetableIndex, root.timetables[timetableIndex].trains.length, data);
	}
	static delete(root: Root, timetableIndex: number, trainIndex: number): Root {
		if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
			throw new RangeError("存在しないインデックス");
		}
		if (trainIndex < 0 || root.timetables[timetableIndex].trains.length <= trainIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const trains = [...root.timetables[timetableIndex].trains];
		trains.splice(trainIndex, 1);
		const newStation = { ...root.timetables[timetableIndex], trains };
		const timetables = [...root.timetables];
		timetables[timetableIndex] = newStation;
		return { ...root, timetables };
	}
	static findById(root: Root, timetableIndex: number, id: string): Train | undefined {
		return root.timetables[timetableIndex].trains.find(s => s.id === id);
	}
	static findIndexById(root: Root, timetableIndex: number, id: string): number {
		return root.timetables[timetableIndex].trains.findIndex(s => s.id === id);
	}

	static getStartingStation(root: Root, data: Train) {
		if (data.segments.length === 0) {
			return undefined;
		}
		if (data.segments[0].segments.length === 0) {
			return undefined;
		}
		if (data.segments[0].segments[0].isReversed) {
			return SegmentService.findByIdAll(root, data.segments[0].segments[0].id);
		} else {
			return SegmentService.findByIdAll(root, data.segments[0].segments[0].id);
		}
	}
	static getDestinationStation(root: Root, data: Train) {
		if (data.segments.length === 0) {
			return undefined;
		}
		if (data.segments[data.segments.length - 1].segments.length === 0) {
			return undefined;
		}
		const lastSegment = data
			.segments[data.segments.length - 1]
			.segments[data.segments[data.segments.length - 1].segments.length - 1];
		if (lastSegment.isReversed) {
			return SegmentService.findByIdAll(root, lastSegment.id);
		} else {
			return SegmentService.findByIdAll(root, lastSegment.id);
		}
	}
}