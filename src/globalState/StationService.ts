import { Root } from '../sharpdia-model/Root';
import { Station } from '../sharpdia-model/Station';

export const StationService = {
  append(root: Root, data: Station): Root {
    return this.insert(root, root.stations.length, data);
  },
  delete(root: Root, index: number): Root {
    if (index < 0 || root.stations.length <= index) {
      throw new RangeError('存在しないインデックス');
    }
    const stations = [...root.stations];
    stations.splice(index, 1);
    return { ...root, stations };
  },
  findById(root: Root, id: string): Station | undefined {
    return root.stations.find((s) => s.id === id);
  },
  findIndexById(root: Root, id: string): number {
    return root.stations.findIndex((s) => s.id === id);
  },
  insert(root: Root, index: number, data: Station): Root {
    if (index < 0 || root.stations.length < index) {
      throw new RangeError('存在しないインデックス');
    }
    const stations = [...root.stations];
    stations.splice(index, 0, data);
    return { ...root, stations };
  },
  update(root: Root, index: number, data: Station): Root {
    if (index < 0 || root.stations.length <= index) {
      throw new RangeError('存在しないインデックス');
    }
    const stations = [...root.stations];
    stations[index] = data;
    return { ...root, stations };
  },
};
