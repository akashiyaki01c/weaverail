import { Root } from '../sharpdia-model/Root';
import { TemplateTrainSegment } from '../sharpdia-model/TemplateTrain';

export const TemplateTrainSegmentService = {
  append(
    root: Root,
    templateTrainIndex: number,
    data: TemplateTrainSegment,
  ): Root {
    return this.insert(
      root,
      templateTrainIndex,
      root.templateTrains[templateTrainIndex].segments.length,
      data,
    );
  },
  delete(root: Root, templateTrainIndex: number, segmentIndex: number): Root {
    if (
      templateTrainIndex < 0 ||
      root.templateTrains.length <= templateTrainIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      segmentIndex < 0 ||
      root.templateTrains[templateTrainIndex].segments.length <= segmentIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const segments = [...root.templateTrains[templateTrainIndex].segments];
    segments.splice(segmentIndex, 1);
    const newLine = {
      ...root.templateTrains[templateTrainIndex],
      segments: segments,
    };
    const templateTrains = [...root.templateTrains];
    templateTrains[templateTrainIndex] = newLine;
    return { ...root, templateTrains };
  },
  findById(
    root: Root,
    templateTrainIndex: number,
    id: string,
  ): TemplateTrainSegment | undefined {
    return root.templateTrains[templateTrainIndex].segments.find(
      (s) => s.id === id,
    );
  },
  findByIdAll(root: Root, id: string): TemplateTrainSegment | undefined {
    return root.templateTrains
      .flatMap((v) => v.segments)
      .find((s) => s.id === id);
  },
  findIndexById(root: Root, templateTrainIndex: number, id: string): number {
    return root.templateTrains[templateTrainIndex].segments.findIndex(
      (s) => s.id === id,
    );
  },
  insert(
    root: Root,
    templateTrainIndex: number,
    segmentIndex: number,
    data: TemplateTrainSegment,
  ): Root {
    if (
      templateTrainIndex < 0 ||
      root.templateTrains.length <= templateTrainIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      segmentIndex < 0 ||
      root.templateTrains[templateTrainIndex].segments.length < segmentIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const segments = [...root.templateTrains[templateTrainIndex].segments];
    segments.splice(segmentIndex, 0, data);
    const newLine = {
      ...root.templateTrains[templateTrainIndex],
      segments: segments,
    };
    const templateTrains = [...root.templateTrains];
    templateTrains[templateTrainIndex] = newLine;
    return { ...root, templateTrains };
  },
  update(
    root: Root,
    templateTrainIndex: number,
    segmentIndex: number,
    data: TemplateTrainSegment,
  ): Root {
    if (
      templateTrainIndex < 0 ||
      root.templateTrains.length <= templateTrainIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    if (
      segmentIndex < 0 ||
      root.templateTrains[templateTrainIndex].segments.length <= segmentIndex
    ) {
      throw new RangeError('存在しないインデックス');
    }
    const segments = [...root.templateTrains[templateTrainIndex].segments];
    segments[segmentIndex] = data;
    const newLine = {
      ...root.templateTrains[templateTrainIndex],
      segments: segments,
    };
    const templateTrains = [...root.templateTrains];
    templateTrains[templateTrainIndex] = newLine;
    return { ...root, templateTrains };
  },
};
