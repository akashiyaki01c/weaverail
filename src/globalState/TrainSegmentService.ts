import { Root } from "../sharpdia-model/Root";
import { TrainSegment } from "../sharpdia-model/Train";

export class TrainSegmentService {
	static update(root: Root, timetableIndex: number, trainIndex: number, segmentIndex: number, data: TrainSegment): Root {
		if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const timetbale = root.timetables[timetableIndex];
		if (trainIndex < 0 || timetbale.trains.length <= trainIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const train = timetbale.trains[trainIndex];

		const segments = [...train.segments];
		segments[segmentIndex] = data;

		const newTrain = { ...train, segments };
		const newTimetables = [...root.timetables];
		newTimetables[timetableIndex].trains[trainIndex] = newTrain;

		return { ...root, timetables: newTimetables };
	}
	static insert(root: Root, timetableIndex: number, trainIndex: number, segmentIndex: number, data: TrainSegment): Root {
		if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const timetbale = root.timetables[timetableIndex];
		if (trainIndex < 0 || timetbale.trains.length <= trainIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const train = timetbale.trains[trainIndex];

		const segments = [...train.segments];
		segments.splice(segmentIndex, 0, data);

		const newTrain = { ...train, segments };
		const newTimetables = [...root.timetables];
		newTimetables[timetableIndex].trains[trainIndex] = newTrain;

		return { ...root, timetables: newTimetables };
	}
	static append(root: Root, timetableIndex: number, trainIndex: number, data: TrainSegment): Root {
		return this.insert(root, timetableIndex, trainIndex, root.timetables[timetableIndex].trains[trainIndex].segments.length, data);
	}
	static delete(root: Root, timetableIndex: number, trainIndex: number, segmentIndex: number): Root {
		if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const timetable = root.timetables[timetableIndex];
		if (trainIndex < 0 || timetable.trains.length <= trainIndex) {
			throw new RangeError("存在しないインデックス");
		}
		const train = timetable.trains[trainIndex];
		if (segmentIndex < 0 || train.segments.length <= segmentIndex) {
			throw new RangeError("存在しないインデックス");
		}

		const segments = [...train.segments];
		segments.splice(segmentIndex, 1);

		const newTrain = { ...train, segments };
		const newTimetables = [...root.timetables];
		newTimetables[timetableIndex].trains[trainIndex] = newTrain;

		return { ...root, timetables: newTimetables };
	}
}