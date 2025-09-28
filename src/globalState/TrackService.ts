import { Root } from '../sharpdia-model/Root';
import { Track } from '../sharpdia-model/Station';

export const TrackService = {
  append(root: Root, stationIndex: number, data: Track): Root {
    return this.insert(
      root,
      stationIndex,
      root.stations[stationIndex].tracks.length,
      data,
    );
  },
  delete(root: Root, stationIndex: number, trackIndex: number): Root {
    if (stationIndex < 0 || root.stations.length <= stationIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      trackIndex < 0 ||
      root.stations[stationIndex].tracks.length <= trackIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const tracks = [...root.stations[stationIndex].tracks];
    tracks.splice(trackIndex, 1);
    const newStation = { ...root.stations[stationIndex], tracks };
    const stations = [...root.stations];
    stations[stationIndex] = newStation;
    return { ...root, stations };
  },
  findById(root: Root, stationIndex: number, id: string): Track | undefined {
    return root.stations[stationIndex].tracks.find((s) => s.id === id);
  },
  findIndexById(root: Root, stationIndex: number, id: string): number {
    return root.stations[stationIndex].tracks.findIndex((s) => s.id === id);
  },
  insert(
    root: Root,
    stationIndex: number,
    trackIndex: number,
    data: Track,
  ): Root {
    if (stationIndex < 0 || root.stations.length <= stationIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      trackIndex < 0 ||
      root.stations[stationIndex].tracks.length < trackIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const tracks = [...root.stations[stationIndex].tracks];
    tracks.splice(trackIndex, 0, data);
    const newStation = { ...root.stations[stationIndex], tracks };
    const stations = [...root.stations];
    stations[stationIndex] = newStation;
    return { ...root, stations };
  },
  update(
    root: Root,
    stationIndex: number,
    trackIndex: number,
    data: Track,
  ): Root {
    if (stationIndex < 0 || root.stations.length <= stationIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      trackIndex < 0 ||
      root.stations[stationIndex].tracks.length <= trackIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const tracks = [...root.stations[stationIndex].tracks];
    tracks[trackIndex] = data;
    const newStation = { ...root.stations[stationIndex], tracks };
    const stations = [...root.stations];
    stations[stationIndex] = newStation;
    return { ...root, stations };
  },
};
