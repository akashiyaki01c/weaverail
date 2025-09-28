import { Root } from '../sharpdia-model/Root';
import { Timetable } from '../sharpdia-model/Timetable';

export const TimetableService = {
  append(root: Root, data: Timetable): Root {
    return this.insert(root, root.timetables.length, data);
  },
  delete(root: Root, index: number): Root {
    if (index < 0 || root.timetables.length <= index) {
      throw new RangeError('存在しないインデックス');
    }
    const timetables = [...root.timetables];
    timetables.splice(index, 1);
    return { ...root, timetables };
  },
  findById(root: Root, id: string): Timetable | undefined {
    return root.timetables.find((s) => s.id === id);
  },
  findIndexById(root: Root, id: string): number {
    return root.timetables.findIndex((s) => s.id === id);
  },
  insert(root: Root, index: number, data: Timetable): Root {
    if (index < 0 || root.timetables.length < index) {
      throw new RangeError('存在しないインデックス');
    }
    const timetables = [...root.timetables];
    timetables.splice(index, 0, data);
    return { ...root, timetables };
  },
  update(root: Root, index: number, data: Timetable): Root {
    if (index < 0 || root.timetables.length <= index) {
      throw new RangeError('存在しないインデックス');
    }
    const timetables = [...root.timetables];
    timetables[index] = data;
    return { ...root, timetables };
  },
};
