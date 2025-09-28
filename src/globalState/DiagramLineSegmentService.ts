import { DiagramLine, DiagramLineSegment } from '../sharpdia-model/DiagramLine';
import { Root } from '../sharpdia-model/Root';

export const DiagramLineSegmentService = {
  append(root: Root, diagramLineIndex: number, data: DiagramLineSegment): Root {
    return this.insert(
      root,
      diagramLineIndex,
      root.diagramLines[diagramLineIndex].segments.length,
      data,
    );
  },
  delete(
    root: Root,
    diagramLineIndex: number,
    diagramLineSegmentIndex: number,
  ): Root {
    if (diagramLineIndex < 0 || root.diagramLines.length <= diagramLineIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      diagramLineSegmentIndex < 0 ||
      root.diagramLines[diagramLineIndex].segments.length <=
        diagramLineSegmentIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const diagramLineSegments = [
      ...root.diagramLines[diagramLineIndex].segments,
    ];
    diagramLineSegments.splice(diagramLineSegmentIndex, 1);
    const newDiagramLine = {
      ...root.diagramLines[diagramLineIndex],
      segments: diagramLineSegments,
    } satisfies DiagramLine;
    const diagramLines = [...root.diagramLines];
    diagramLines[diagramLineIndex] = newDiagramLine;
    return { ...root, diagramLines };
  },
  findById(
    root: Root,
    diagramLineIndex: number,
    id: string,
  ): DiagramLineSegment | undefined {
    return root.diagramLines[diagramLineIndex].segments.find(
      (s) => s.id === id,
    );
  },
  findIndexById(root: Root, diagramLineIndex: number, id: string): number {
    return root.diagramLines[diagramLineIndex].segments.findIndex(
      (s) => s.id === id,
    );
  },
  insert(
    root: Root,
    diagramLineIndex: number,
    diagramLineSegmentIndex: number,
    data: DiagramLineSegment,
  ): Root {
    if (diagramLineIndex < 0 || root.diagramLines.length <= diagramLineIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      diagramLineSegmentIndex < 0 ||
      root.diagramLines[diagramLineIndex].segments.length <
        diagramLineSegmentIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const diagramLineSegments = [
      ...root.diagramLines[diagramLineIndex].segments,
    ];
    diagramLineSegments.splice(diagramLineSegmentIndex, 0, data);
    const newDiagramLine = {
      ...root.diagramLines[diagramLineIndex],
      segments: diagramLineSegments,
    } satisfies DiagramLine;
    const diagramLines = [...root.diagramLines];
    diagramLines[diagramLineIndex] = newDiagramLine;
    return { ...root, diagramLines };
  },
  update(
    root: Root,
    diagramLineIndex: number,
    diagramLineSegmentIndex: number,
    data: DiagramLineSegment,
  ): Root {
    if (diagramLineIndex < 0 || root.diagramLines.length <= diagramLineIndex) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      diagramLineSegmentIndex < 0 ||
      root.diagramLines[diagramLineIndex].segments.length <=
        diagramLineSegmentIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const diagramLineSegments = [
      ...root.diagramLines[diagramLineIndex].segments,
    ];
    diagramLineSegments[diagramLineSegmentIndex] = data;
    const newDiagramLine = {
      ...root.diagramLines[diagramLineIndex],
      segments: diagramLineSegments,
    } satisfies DiagramLine;
    const diagramLines = [...root.diagramLines];
    diagramLines[diagramLineIndex] = newDiagramLine;
    return { ...root, diagramLines };
  },
};
