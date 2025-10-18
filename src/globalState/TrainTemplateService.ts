import { Root } from '../sharpdia-model/Root';
import { TemplateTrain } from '../sharpdia-model/TemplateTrain';
import { Train, TrainSegment } from '../sharpdia-model/Train';
import { SegmentService } from './SegmentService';

export const TemplateTrainService = {
  append(root: Root, data: TemplateTrain): Root {
    return this.insert(root, root.templateTrains.length, data);
  },
  delete(root: Root, index: number): Root {
    if (index < 0 || root.templateTrains.length <= index) {
      throw new RangeError('存在しないインデックス');
    }
    const templateTrains = [...root.templateTrains];
    templateTrains.splice(index, 1);
    return { ...root, templateTrains };
  },
  findById(root: Root, id: string): TemplateTrain | undefined {
    return root.templateTrains.find((s) => s.id === id);
  },
  findIndexById(root: Root, id: string): number {
    return root.templateTrains.findIndex((s) => s.id === id);
  },
  generateTrain(
    root: Root,
    template: TemplateTrain,
    stationId: string,
    departureTime: number,
    startIndex: number,
    endIndex: number,
  ): Train {
    const resultTrain = Train.default();
    resultTrain.trainTypeId = template.trainTypeId;

    let nowTime = 0;
    for (const segment of template.segments.slice(startIndex, endIndex + 1)) {
      const resultSegment = TrainSegment.default();
      for (const seg of segment.segments) {
        resultSegment.segments.push(seg);
      }
      resultSegment.departureTime = nowTime + 20;
      resultSegment.arrivalTime = nowTime + 20 + segment.time;
      nowTime = resultSegment.arrivalTime;
      resultTrain.segments.push(resultSegment);
    }

    const temporaryDepTime =
      resultTrain.segments.find((v) => {
        const segment = SegmentService.findByIdAll(root, v.segments[0]?.id);
        return v.segments[0]?.isReversed
          ? segment?.endId === stationId
          : segment?.startId === stationId;
      })?.departureTime || 0;
    const diff = departureTime - temporaryDepTime;
    for (let index = 0; index < resultTrain.segments.length; index++) {
      resultTrain.segments[index].arrivalTime += diff;
      resultTrain.segments[index].departureTime += diff;
    }

    return resultTrain;
  },
  insert(root: Root, index: number, data: TemplateTrain): Root {
    if (index < 0 || root.templateTrains.length < index) {
      throw new RangeError('存在しないインデックス');
    }
    const templateTrains = [...root.templateTrains];
    templateTrains.splice(index, 0, data);
    return { ...root, templateTrains };
  },

  update(root: Root, index: number, data: TemplateTrain): Root {
    if (index < 0 || root.templateTrains.length <= index) {
      throw new RangeError('存在しないインデックス');
    }
    const templateTrains = [...root.templateTrains];
    templateTrains[index] = data;
    return { ...root, templateTrains };
  },
};
