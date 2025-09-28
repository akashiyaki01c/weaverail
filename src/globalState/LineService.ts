import { Line } from '../sharpdia-model/Line';
import { Root } from '../sharpdia-model/Root';

export const LineService = {
  append(root: Root, data: Line): Root {
    return this.insert(root, root.lines.length, data);
  },
  delete(root: Root, index: number): Root {
    if (index < 0 || root.lines.length <= index) {
      throw new RangeError('存在しないインデックス');
    }
    const lines = [...root.lines];
    lines.splice(index, 1);
    return { ...root, lines };
  },
  findById(root: Root, id: string): Line | undefined {
    return root.lines.find((s) => s.id === id);
  },
  findIndexById(root: Root, id: string): number {
    return root.lines.findIndex((s) => s.id === id);
  },
  insert(root: Root, index: number, data: Line): Root {
    if (index < 0 || root.lines.length < index) {
      throw new RangeError('存在しないインデックス');
    }
    const lines = [...root.lines];
    lines.splice(index, 0, data);
    return { ...root, lines };
  },
  update(root: Root, index: number, data: Line): Root {
    if (index < 0 || root.lines.length <= index) {
      throw new RangeError('存在しないインデックス');
    }
    const lines = [...root.lines];
    lines[index] = data;
    return { ...root, lines };
  },
};
