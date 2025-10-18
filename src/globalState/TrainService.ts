import { Root } from '../sharpdia-model/Root';
import { Train } from '../sharpdia-model/Train';
import { SegmentService } from './SegmentService';

export const TrainService = {
  append(root: Root, timetableIndex: number, data: Train): Root {
    return this.insert(
      root,
      timetableIndex,
      root.timetables[timetableIndex].trains.length,
      data,
    );
  },
  delete(root: Root, timetableIndex: number, trainIndex: number): Root {
    if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      trainIndex < 0 ||
      root.timetables[timetableIndex].trains.length <= trainIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const trains = [...root.timetables[timetableIndex].trains];
    trains.splice(trainIndex, 1);
    const newStation = { ...root.timetables[timetableIndex], trains };
    const timetables = [...root.timetables];
    timetables[timetableIndex] = newStation;
    return { ...root, timetables };
  },
  findById(root: Root, timetableIndex: number, id: string): Train | undefined {
    return root.timetables[timetableIndex].trains.find((s) => s.id === id);
  },
  findIndexById(root: Root, timetableIndex: number, id: string): number {
    return root.timetables[timetableIndex].trains.findIndex((s) => s.id === id);
  },
  getDestinationStation(root: Root, data: Train) {
    if (data.segments.length === 0) {
      return;
    }
    if (data.segments.at(-1) === undefined) {
      return;
    }
    if (data.segments.at(-1)!.segments.length === 0) {
      return;
    }
    const lastSegment = SegmentService.findByIdAll(
      root,
      data.segments.at(-1)!.segments[data.segments.at(-1)!.segments.length - 1]
        .id,
    );
    if (
      data.segments.at(-1)!.segments[data.segments.at(-1)!.segments.length - 1]
        .isReversed
    ) {
      const r = {
        endId: lastSegment?.startId,
        id: lastSegment?.id,
        startId: lastSegment?.endId,
      };
      return r;
    }
    return lastSegment;
  },
  getStartingStation(root: Root, data: Train) {
    if (data.segments.length === 0) {
      return;
    }
    if (data.segments[0].segments.length === 0) {
      return;
    }
    const result = SegmentService.findByIdAll(
      root,
      data.segments[0].segments[0].id,
    );
    if (data.segments[0].segments[0].isReversed) {
      const r = {
        endId: result?.startId,
        id: result?.id,
        startId: result?.endId,
      };
      return r;
    }
    return result;
  },

  insert(
    root: Root,
    timetableIndex: number,
    trainIndex: number,
    data: Train,
  ): Root {
    if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      trainIndex < 0 ||
      root.timetables[timetableIndex].trains.length < trainIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const trains = [...root.timetables[timetableIndex].trains];
    trains.splice(trainIndex, 0, data);
    const newStation = { ...root.timetables[timetableIndex], trains };
    const timetables = [...root.timetables];
    timetables[timetableIndex] = newStation;
    return { ...root, timetables };
  },
  update(
    root: Root,
    timetableIndex: number,
    trainIndex: number,
    data: Train,
  ): Root {
    if (timetableIndex < 0 || root.timetables.length <= timetableIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      trainIndex < 0 ||
      root.timetables[timetableIndex].trains.length <= trainIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const trains = [...root.timetables[timetableIndex].trains];
    trains[trainIndex] = data;
    const newStation = { ...root.timetables[timetableIndex], trains };
    const timetables = [...root.timetables];
    timetables[timetableIndex] = newStation;
    return { ...root, timetables };
  },
};
