import { useEffect, useRef, useState } from 'react';

import { SegmentService } from '../globalState/SegmentService';
import { StationService } from '../globalState/StationService';
import { TimetableService } from '../globalState/TimetableService';
import { TrainSegmentService } from '../globalState/TrainSegmentService';
import { TrainService } from '../globalState/TrainService';
import { TemplateTrainService } from '../globalState/TrainTemplateService';
import { TrainTypeService } from '../globalState/TrainTypeService';
import useGlobalState from '../globalState/useGlobalState';
import { DiagramLine } from '../sharpdia-model/DiagramLine';
import { Root } from '../sharpdia-model/Root';
import { TemplateTrain } from '../sharpdia-model/TemplateTrain';
import { toTimeString } from '../sharpdia-model/TimeParser';
import { Passing, Timetable } from '../sharpdia-model/Timetable';
import { Train, TrainSegment } from '../sharpdia-model/Train';
import { XAxisSvg } from './DiagramViewer.XAxisSvg';
import { TrainViewer } from './TrainViewer';

export interface StationChoord {
  // 駅間が逆転しているか
  isReversed: boolean;
  // ダイヤグラム上での下側の駅ID
  lowerStationId: string;
  // ダイヤグラム上での下側の駅のY座標
  lowerStationYChoord: number;
  // 駅間ID
  segmentId: string;
  // ダイヤグラム上での上側の駅ID
  upperStationId: string;
  // ダイヤグラム上での上側の駅のY座標
  upperStationYChoord: number;
}
export class DiagramViewerOption {
  constructor(
    public xScale: number,
    public yScale: number,
    public xOffset: number,
    public yOffset: number,
  ) {}
}

export function DiagramViewer({
  diagramLineId,
  timetableId,
}: {
  diagramLineId: string;
  timetableId: string;
}) {
  const globalState = useGlobalState();
  const trainGenerateDialogReference = useRef<HTMLDialogElement>(null);
  const trainDetailDialogReference = useRef<HTMLDialogElement>(null);
  if (!diagramLineId) {
    throw new Error('diagram line id null');
  }
  const diagramLineIndex = globalState.root.diagramLines.findIndex(
    (v) => v.id === diagramLineId,
  );
  const diagramLine = globalState.root.diagramLines[diagramLineIndex];
  if (!diagramLine) {
    throw new Error('diagram line is null');
  }
  if (!timetableId) {
    throw new Error('timetable id null');
  }
  const timetableIndex = globalState.root.timetables.findIndex(
    (v) => v.id === timetableId,
  );
  const timetable = globalState.root.timetables[timetableIndex];
  if (!timetable) {
    throw new Error('timetable is null');
  }
  const [yChoords, setYChoords] = useState<StationChoord[]>([]);
  useEffect(() => {
    setYChoords(getYChoords(globalState.root, diagramLine));
  }, [diagramLine, globalState.root]);

  const [option] = useState(new DiagramViewerOption(0.15, 0.3, 0, 0));

  const viewBoxWidth = option.xScale * 60 * 60 * 24;
  const viewBoxHeight =
    option.yScale * (yChoords.at(-1)?.lowerStationYChoord || 0);

  const [targetTemplates, setTargetTemplates] = useState<TemplateTrain[]>([]);
  const [clickStation, setClickStation] = useState('');
  const [clickTime, setClickTime] = useState(0);
  const [selectedTemplate, setSelectedTemplate] = useState<TemplateTrain>(
    TemplateTrain.default(),
  );
  const [selectedDepartureStation, setSelectedDepartureStation] =
    useState('-1');
  const [selectedArrivalStation, setSelectedArrivalStation] = useState('-1');

  const [clickTrainId, setClickTrainId] = useState('');

  const [targetPassingTrainId, setTargetPassingTrainId] = useState('');
  const [targetStoppingTrainId, setTargetStoppingTrainId] = useState('');

  return (
    <>
      <div className="max-w-[100cqw] max-h-[100cqh] w-[100%] h-[100%] overflow-scroll z-0">
        <div className="sticky top-[0] w-[max-content] z-10 bg-gray-100">
          <div className="left-[120px] relative">
            <XAxisSvg option={option} />
          </div>
        </div>
        <div className="w-[max-content] z-0">
          <div className="z-10">
            <svg
              className="diagram-main-svg overflow-x-scroll overflow-y-hidden relative top-0 left-[120px] block"
              height={viewBoxHeight}
              viewBox={`0 0 ${viewBoxWidth} ${viewBoxHeight}`}
              width={viewBoxWidth}
            >
              <g className="axis">
                <g className="y-axis">
                  {yChoords.map((v) => (
                    <g
                      data-key={`${v.segmentId}`}
                      id={`${v.segmentId}`}
                      key={`${v.segmentId}`}
                    >
                      {StationService.findById(
                        globalState.root,
                        v.upperStationId,
                      )?.kibo === 'Syuyou' && (
                        <rect
                          className="fill-gray-100"
                          data-key={`${v.segmentId}-upper-toucher`}
                          height={10}
                          id={`${v.segmentId}-upper-toucher`}
                          key={`${v.segmentId}-upper-toucher`}
                          onClick={(event) => {
                            const svg = (event.target as SVGRectElement)
                              .ownerSVGElement;
                            if (!svg) return;

                            const pt = new DOMPoint(
                              event.clientX,
                              event.clientY,
                            );
                            const svgPoint = pt.matrixTransform(
                              svg.getScreenCTM()?.inverse(),
                            );
                            const svgX =
                              svgPoint.x / option.xScale - option.xOffset;
                            // 下り追加
                            setClickTime(svgX - (svgX % 10) + 60 * 60 * 4);
                            setClickStation(v.upperStationId);
                            setTargetTemplates(
                              globalState.root.templateTrains.filter(
                                (template) =>
                                  template.segments.some(
                                    (segment) =>
                                      segment.segments[0]?.id === v.segmentId &&
                                      !segment.segments[0]?.isReversed,
                                  ),
                              ),
                            );
                            trainGenerateDialogReference.current?.showModal();
                          }}
                          width={60 * 60 * 24 * option.xScale}
                          x={0}
                          y={
                            (v.upperStationYChoord + option.yOffset) *
                            option.yScale
                          }
                        />
                      )}
                      <line
                        data-key={`${v.segmentId}-upper`}
                        id={`${v.segmentId}-upper`}
                        key={`${v.segmentId}-upper`}
                        stroke="#000"
                        strokeWidth={
                          StationService.findById(
                            globalState.root,
                            v.upperStationId,
                          )?.kibo === 'Syuyou'
                            ? 1.5
                            : 0.5
                        }
                        x1={0}
                        x2={60 * 60 * 24 * option.xScale}
                        y1={
                          (v.upperStationYChoord + option.yOffset) *
                          option.yScale
                        }
                        y2={
                          (v.upperStationYChoord + option.yOffset) *
                          option.yScale
                        }
                      />
                      {StationService.findById(
                        globalState.root,
                        v.lowerStationId,
                      )?.kibo === 'Syuyou' && (
                        <rect
                          className="fill-gray-100"
                          data-key={`yaxis-${v.segmentId}-lower-toucher`}
                          height={10}
                          id={`yaxis-${v.segmentId}-lower-toucher`}
                          key={`yaxis-${v.segmentId}-lower-toucher`}
                          onClick={(event) => {
                            const svg = (event.target as SVGRectElement)
                              .ownerSVGElement;
                            if (!svg) return;

                            const pt = new DOMPoint(
                              event.clientX,
                              event.clientY,
                            );
                            const svgPoint = pt.matrixTransform(
                              svg.getScreenCTM()?.inverse(),
                            );
                            const svgX =
                              svgPoint.x / option.xScale - option.xOffset;
                            // 上り追加
                            setClickTime(svgX - (svgX % 10) + 60 * 60 * 4);
                            setClickStation(v.lowerStationId);
                            setTargetTemplates(
                              globalState.root.templateTrains.filter(
                                (template) =>
                                  template.segments.some(
                                    (segment) =>
                                      segment.segments[0]?.id === v.segmentId &&
                                      segment.segments[0]?.isReversed,
                                  ),
                              ),
                            );
                            console.log(targetTemplates);
                            trainGenerateDialogReference.current?.showModal();
                          }}
                          width={60 * 60 * 24 * option.xScale}
                          x={0}
                          y={
                            (v.lowerStationYChoord + option.yOffset) *
                              option.yScale -
                            10
                          }
                        />
                      )}
                      <line
                        data-key={`yaxis-${v.segmentId}-lower`}
                        id={`yaxis-${v.segmentId}-lower`}
                        key={`yaxis-${v.segmentId}-lower`}
                        stroke="#000"
                        strokeWidth={
                          StationService.findById(
                            globalState.root,
                            v.lowerStationId,
                          )?.kibo === 'Syuyou'
                            ? 1.5
                            : 0.5
                        }
                        x1={0}
                        x2={60 * 60 * 24 * option.xScale}
                        y1={
                          (v.lowerStationYChoord + option.yOffset) *
                          option.yScale
                        }
                        y2={
                          (v.lowerStationYChoord + option.yOffset) *
                          option.yScale
                        }
                      />
                    </g>
                  ))}
                </g>
                <g className="x-axis">
                  {Array.from({ length: 24 * 60 })
                    .map((_, index) => index)
                    .filter((v) => v % 1 === 0)
                    .map((v) => (
                      <line
                        data-key={v}
                        key={v}
                        stroke="#000"
                        strokeWidth="0.25"
                        x1={(option.xOffset + v * 60) * option.xScale}
                        x2={(option.xOffset + v * 60) * option.xScale}
                        y1={0}
                        y2={viewBoxHeight}
                      />
                    ))}
                  {Array.from({ length: 24 * 60 })
                    .map((_, index) => index)
                    .filter((v) => v % 5 === 0)
                    .map((v) => (
                      <line
                        data-key={`${v}-1`}
                        id={`${v}-1`}
                        key={`${v}-1`}
                        stroke="#000"
                        strokeWidth="1"
                        x1={(option.xOffset + v * 60) * option.xScale}
                        x2={(option.xOffset + v * 60) * option.xScale}
                        y1={0}
                        y2={viewBoxHeight}
                      />
                    ))}
                  {Array.from({ length: 24 * 60 })
                    .map((_, index) => index)
                    .filter((v) => v % 30 === 0)
                    .map((v) => (
                      <line
                        data-key={`${v}-2`}
                        id={`${v}-2`}
                        key={`${v}-2`}
                        stroke="#000"
                        strokeWidth="2"
                        x1={(option.xOffset + v * 60) * option.xScale}
                        x2={(option.xOffset + v * 60) * option.xScale}
                        y1={0}
                        y2={viewBoxHeight}
                      />
                    ))}
                </g>
              </g>
              <g className="trains">
                {timetable.trains.map((train) => (
                  <g
                    data-key={`train-${train.id}`}
                    id={`train-${train.id}`}
                    key={`train-${train.id}`}
                  >
                    {getSimpleTrain(globalState.root, train).segments.map(
                      (segment) => {
                        const trainType = TrainTypeService.findById(
                          globalState.root,
                          train.trainTypeId,
                        )!;
                        let departureTime = segment.departureTime - 4 * 60 * 60;
                        if (departureTime < 0) {
                          departureTime += 24 * 60 * 60;
                        }
                        let arrivalTime = segment.arrivalTime - 4 * 60 * 60;
                        if (arrivalTime < 0) {
                          arrivalTime += 24 * 60 * 60;
                        }
                        const result = [];
                        const totalTime = segment.segments
                          .map((v) => {
                            const yChoord = yChoords.find(
                              (c) => c.segmentId === v.id,
                            );
                            if (yChoord == undefined) {
                              return 0;
                            }
                            return (
                              yChoord.lowerStationYChoord -
                              yChoord.upperStationYChoord
                            );
                          })
                          .reduce((p, c) => p + c, 0);
                        let startTime = 0;
                        for (const sg of segment.segments) {
                          const yChoord = yChoords.find(
                            (v) => v.segmentId === sg.id,
                          );
                          if (yChoord == undefined) {
                            continue;
                          }
                          const currentTime =
                            yChoord.lowerStationYChoord -
                            yChoord.upperStationYChoord;
                          const startRatio = startTime / totalTime;
                          const lastRatio =
                            (currentTime + startTime) / totalTime;
                          startTime += currentTime;
                          const departure =
                            departureTime +
                            (arrivalTime - departureTime) * startRatio;
                          const arrival =
                            departureTime +
                            (arrivalTime - departureTime) * lastRatio;
                          result.push(
                            sg.isReversed ? (
                              <>
                                <line
                                  data-key={`${train.id}-${segment.id}-1`}
                                  key={`${train.id}-${segment.id}-1`}
                                  stroke={trainType?.color || '#000'}
                                  strokeWidth="1"
                                  x1={
                                    (departure + option.xOffset) * option.xScale
                                  }
                                  x2={
                                    (arrival + option.xOffset) * option.xScale
                                  }
                                  y1={
                                    (yChoords.find((v) => v.segmentId === sg.id)
                                      ?.lowerStationYChoord ||
                                      0 + option.yOffset) * option.yScale
                                  }
                                  y2={
                                    (yChoords.find((v) => v.segmentId === sg.id)
                                      ?.upperStationYChoord ||
                                      0 + option.yOffset) * option.yScale
                                  }
                                />
                                <line
                                  data-key={`${train.id}-${segment.id}-2`}
                                  key={`${train.id}-${segment.id}-2`}
                                  onClick={(event) => {
                                    const svg = (event.target as SVGRectElement)
                                      .ownerSVGElement;
                                    if (!svg) return;

                                    const pt = new DOMPoint(
                                      event.clientX,
                                      event.clientY,
                                    );
                                    const svgPoint = pt.matrixTransform(
                                      svg.getScreenCTM()?.inverse(),
                                    );
                                    const svgY = svgPoint.y;
                                    for (const choord of yChoords) {
                                      const lower =
                                        choord.lowerStationYChoord *
                                          option.yScale +
                                        option.yOffset;
                                      const upper =
                                        choord.upperStationYChoord *
                                          option.yScale +
                                        option.yOffset;
                                      if (Math.abs(lower - svgY) < 10) {
                                        setClickStation(choord.lowerStationId);
                                        break;
                                      }
                                      if (Math.abs(upper - svgY) < 10) {
                                        setClickStation(choord.upperStationId);
                                        break;
                                      }
                                    }
                                    setClickTrainId(train.id);
                                    trainDetailDialogReference.current?.showModal();
                                  }}
                                  stroke="transparent"
                                  strokeWidth="5"
                                  style={{
                                    cursor: 'pointer',
                                  }}
                                  x1={
                                    (departure + option.xOffset) * option.xScale
                                  }
                                  x2={
                                    (arrival + option.xOffset) * option.xScale
                                  }
                                  y1={
                                    (yChoords.find((v) => v.segmentId === sg.id)
                                      ?.lowerStationYChoord ||
                                      0 + option.yOffset) * option.yScale
                                  }
                                  y2={
                                    (yChoords.find((v) => v.segmentId === sg.id)
                                      ?.upperStationYChoord ||
                                      0 + option.yOffset) * option.yScale
                                  }
                                />
                              </>
                            ) : (
                              <>
                                <line
                                  data-key={`${train.id}-${segment.id}-3`}
                                  key={`${train.id}-${segment.id}-3`}
                                  stroke={trainType?.color || '#000'}
                                  strokeWidth="1"
                                  x1={
                                    (departure + option.xOffset) * option.xScale
                                  }
                                  x2={
                                    (arrival + option.xOffset) * option.xScale
                                  }
                                  y1={
                                    (yChoords.find((v) => v.segmentId === sg.id)
                                      ?.upperStationYChoord ||
                                      0 + option.yOffset) * option.yScale
                                  }
                                  y2={
                                    (yChoords.find((v) => v.segmentId === sg.id)
                                      ?.lowerStationYChoord ||
                                      0 + option.yOffset) * option.yScale
                                  }
                                />
                                <line
                                  data-key={`${train.id}-${segment.id}-4`}
                                  key={`${train.id}-${segment.id}-4`}
                                  onClick={(event) => {
                                    const svg = (event.target as SVGRectElement)
                                      .ownerSVGElement;
                                    if (!svg) return;

                                    const pt = new DOMPoint(
                                      event.clientX,
                                      event.clientY,
                                    );
                                    const svgPoint = pt.matrixTransform(
                                      svg.getScreenCTM()?.inverse(),
                                    );
                                    const svgY = svgPoint.y;
                                    for (const choord of yChoords) {
                                      const lower =
                                        choord.lowerStationYChoord *
                                          option.yScale +
                                        option.yOffset;
                                      const upper =
                                        choord.upperStationYChoord *
                                          option.yScale +
                                        option.yOffset;
                                      if (Math.abs(lower - svgY) < 10) {
                                        setClickStation(choord.lowerStationId);
                                        break;
                                      }
                                      if (Math.abs(upper - svgY) < 10) {
                                        setClickStation(choord.upperStationId);
                                        break;
                                      }
                                    }

                                    setClickTrainId(train.id);
                                    trainDetailDialogReference.current?.showModal();
                                  }}
                                  stroke="transparent"
                                  strokeWidth="5"
                                  style={{
                                    cursor: 'pointer',
                                  }}
                                  x1={
                                    (departure + option.xOffset) * option.xScale
                                  }
                                  x2={
                                    (arrival + option.xOffset) * option.xScale
                                  }
                                  y1={
                                    (yChoords.find((v) => v.segmentId === sg.id)
                                      ?.upperStationYChoord ||
                                      0 + option.yOffset) * option.yScale
                                  }
                                  y2={
                                    (yChoords.find((v) => v.segmentId === sg.id)
                                      ?.lowerStationYChoord ||
                                      0 + option.yOffset) * option.yScale
                                  }
                                />
                              </>
                            ),
                          );
                        }
                        return result;
                      },
                    )}
                  </g>
                ))}
              </g>
            </svg>
          </div>
          <div className="absolute left-[0] top-[50px] z-0">
            <svg
              className="y-axis-svg overflow-scroll sticky left-[0] top-[auto] block"
              height={viewBoxHeight}
              viewBox={`0 0 ${120} ${viewBoxHeight}`}
              width={120}
            >
              <rect
                className="fill-gray-100"
                height={viewBoxHeight}
                width={120}
                x={0}
                y={0}
              ></rect>
              {yChoords.map((v) => (
                <>
                  <text
                    data-key={`yaxis2-${v.segmentId}-upper-text`}
                    dominantBaseline="text-before-edge"
                    key={`yaxis2-${v.segmentId}-upper-text`}
                    textAnchor="middle"
                    x={60}
                    y={(v.upperStationYChoord + option.yOffset) * option.yScale}
                  >
                    {
                      StationService.findById(
                        globalState.root,
                        v.upperStationId,
                      )?.name
                    }
                  </text>
                  <line
                    data-key={`yaxis2-${v.segmentId}-upper`}
                    id={`yaxis2-${v.segmentId}-upper`}
                    key={`yaxis2-${v.segmentId}-upper`}
                    stroke="#000"
                    strokeWidth="1"
                    x1={0}
                    x2={60 * 60 * 24 * option.xScale}
                    y1={
                      (v.upperStationYChoord + option.yOffset) * option.yScale
                    }
                    y2={
                      (v.upperStationYChoord + option.yOffset) * option.yScale
                    }
                  />
                  <text
                    data-key={`yaxis2-${v.segmentId}-lower-text`}
                    dominantBaseline="text-before-edge"
                    key={`yaxis2-${v.segmentId}-lower-text`}
                    textAnchor="middle"
                    x={60}
                    y={(v.lowerStationYChoord + option.yOffset) * option.yScale}
                  >
                    {
                      StationService.findById(
                        globalState.root,
                        v.lowerStationId,
                      )?.name
                    }
                  </text>
                  <line
                    data-key={`yaxis2-${v.segmentId}-lower`}
                    id={`yaxis2-${v.segmentId}-lower`}
                    key={`yaxis2-${v.segmentId}-lower`}
                    stroke="#000"
                    strokeWidth="1"
                    x1={0}
                    x2={60 * 24 * option.xScale}
                    y1={
                      (v.lowerStationYChoord + option.yOffset) * option.yScale
                    }
                    y2={
                      (v.lowerStationYChoord + option.yOffset) * option.yScale
                    }
                  />
                </>
              ))}
            </svg>
          </div>
        </div>
      </div>
      <div className="fixed z-100 top-0 left-0">
        <dialog
          className="m-auto p-[1ic] rounded shadow-xl"
          ref={trainGenerateDialogReference}
        >
          <form
            className="flex flex-col gap-4"
            onSubmit={(event) => {
              event.preventDefault();
            }}
          >
            <label>
              列車テンプレート
              <select
                onChange={(event) => {
                  setSelectedTemplate(
                    TemplateTrainService.findById(
                      globalState.root,
                      event.target.value,
                    ) || TemplateTrain.default(),
                  );
                }}
                value={selectedTemplate?.id}
              >
                <option value=""></option>
                {targetTemplates.map((v) => (
                  <option value={v.id}>{v.name}</option>
                ))}
              </select>
            </label>
            <label>
              始発駅
              <select
                onChange={(event) =>
                  setSelectedDepartureStation(event.target.value)
                }
                value={selectedDepartureStation}
              >
                <option value="-1"></option>
                {selectedTemplate.segments.map((segment, index) => (
                  <option value={index}>
                    {
                      StationService.findById(
                        globalState.root,
                        segment.segments[0]?.isReversed
                          ? SegmentService.findByIdAll(
                              globalState.root,
                              segment.segments[0].id,
                            )?.endId || ''
                          : SegmentService.findByIdAll(
                              globalState.root,
                              segment.segments[0].id,
                            )?.startId || '',
                      )?.name
                    }
                  </option>
                ))}
              </select>
            </label>
            <label>
              終着駅
              <select
                onChange={(event) =>
                  setSelectedArrivalStation(event.target.value)
                }
                value={selectedArrivalStation}
              >
                <option value="-1"></option>
                {selectedTemplate.segments.map((segment, index) => (
                  <option
                    disabled={index < Number(selectedDepartureStation)}
                    value={index}
                  >
                    {
                      StationService.findById(
                        globalState.root,
                        segment.segments.at(-1)?.isReversed
                          ? SegmentService.findByIdAll(
                              globalState.root,
                              segment.segments.at(-1)?.id || '',
                            )?.startId || ''
                          : SegmentService.findByIdAll(
                              globalState.root,
                              segment.segments.at(-1)?.id || '',
                            )?.endId || '',
                      )?.name
                    }
                  </option>
                ))}
              </select>
            </label>
            <div className="mt-[1ic] flex justify-end gap-2">
              <button
                className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded"
                onClick={() => {
                  trainGenerateDialogReference.current?.close();
                }}
                type="button"
              >
                キャンセル
              </button>
              <button
                className="bg-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] text-gray-50 rounded"
                onClick={() => {
                  const train = TemplateTrainService.generateTrain(
                    globalState.root,
                    selectedTemplate,
                    clickStation,
                    clickTime,
                    Number(selectedDepartureStation),
                    Number(selectedArrivalStation),
                  );
                  console.log(train);
                  console.log('generated!');
                  globalState.setRoot((root) =>
                    TrainService.append(root, timetableIndex, train),
                  );
                  trainGenerateDialogReference.current?.close();
                }}
              >
                適用
              </button>
            </div>
          </form>
        </dialog>
      </div>
      <div className="fixed z-100 top-0 left-0">
        <dialog
          className="m-auto p-[1ic] rounded shadow-xl"
          ref={trainDetailDialogReference}
        >
          <form
            className="flex flex-col gap-4"
            onSubmit={(event) => {
              event.preventDefault();
            }}
          >
            選択駅:{' '}
            {StationService.findById(globalState.root, clickStation)?.name}
            <TrainViewer timetableId={timetableId} trainId={clickTrainId} />
            <div className="mt-[1ic] flex justify-end gap-2">
              <button
                className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded"
                onClick={() => {
                  trainDetailDialogReference.current?.close();
                  globalState.setRoot((root) =>
                    TrainService.delete(
                      root,
                      timetableIndex,
                      TrainService.findIndexById(
                        root,
                        timetableIndex,
                        clickTrainId,
                      ),
                    ),
                  );
                }}
                type="button"
              >
                削除する
              </button>
              <button
                className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded"
                onClick={() => {
                  const value =
                    TrainService.findById(
                      globalState.root,
                      timetableIndex,
                      clickTrainId,
                    )?.id || '';
                  setTargetStoppingTrainId(value);
                  console.log(
                    `通過側列車: ${timetable.trains.find((v) => v.id === targetPassingTrainId)?.number}列車 ${StationService.findById(globalState.root, TrainSegmentService.getStartingStationId(globalState.root, timetable.trains.find((v) => v.id === targetPassingTrainId)?.segments[0] || TrainSegment.default()) || '')?.name}${toTimeString(timetable.trains.find((v) => v.id === targetPassingTrainId)?.segments[0].departureTime || 0)}発`,
                    `停車側列車: ${timetable.trains.find((v) => v.id === value)?.number}列車 ${StationService.findById(globalState.root, TrainSegmentService.getStartingStationId(globalState.root, timetable.trains.find((v) => v.id === value)?.segments[0] || TrainSegment.default()) || '')?.name}${toTimeString(timetable.trains.find((v) => v.id === value)?.segments[0].departureTime || 0)}発`,
                  );
                  trainDetailDialogReference.current?.close();
                }}
                type="button"
              >
                待避列車に設定
              </button>
              <button
                className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded"
                onClick={() => {
                  const value =
                    TrainService.findById(
                      globalState.root,
                      timetableIndex,
                      clickTrainId,
                    )?.id || '';
                  setTargetPassingTrainId(value);
                  console.log(
                    `通過側列車: ${timetable.trains.find((v) => v.id === value)?.number}列車 ${StationService.findById(globalState.root, TrainSegmentService.getStartingStationId(globalState.root, timetable.trains.find((v) => v.id === value)?.segments[0] || TrainSegment.default()) || '')?.name}${toTimeString(timetable.trains.find((v) => v.id === value)?.segments[0].departureTime || 0)}発`,
                    `停車側列車: ${timetable.trains.find((v) => v.id === targetStoppingTrainId)?.number}列車 ${StationService.findById(globalState.root, TrainSegmentService.getStartingStationId(globalState.root, timetable.trains.find((v) => v.id === targetStoppingTrainId)?.segments[0] || TrainSegment.default()) || '')?.name}${toTimeString(timetable.trains.find((v) => v.id === targetStoppingTrainId)?.segments[0].departureTime || 0)}発`,
                  );
                  trainDetailDialogReference.current?.close();
                }}
                type="button"
              >
                待避対象列車に設定
              </button>
              <button
                className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded"
                onClick={() => {
                  if (
                    targetStoppingTrainId === '' ||
                    targetPassingTrainId === ''
                  ) {
                    console.error('対象列車が存在しません。');
                  }

                  const passing = new Passing(
                    clickStation,
                    targetStoppingTrainId,
                    targetPassingTrainId,
                  );
                  const newPassings = [...(timetable.passings || [])];
                  newPassings.push(passing);
                  const _timetable = applyPassing(
                    globalState.root,
                    timetable,
                    passing,
                  );
                  globalState.setRoot((root) =>
                    TimetableService.update(root, timetableIndex, {
                      ..._timetable[1],
                      passings: newPassings,
                    }),
                  );
                  setTargetPassingTrainId('');
                  setTargetStoppingTrainId('');
                  setClickStation('');
                }}
                type="button"
              >
                待避設定
              </button>
              <button
                className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded"
                onClick={() => {
                  trainDetailDialogReference.current?.close();
                }}
                type="button"
              >
                閉じる
              </button>
            </div>
          </form>
        </dialog>
      </div>
    </>
  );
}

/// 待避設定を適用させる関数
function applyPassing(
  root: Root,
  _timetable: Timetable,
  targetPassing: Passing,
): [boolean, Timetable] {
  let timetable = structuredClone(_timetable);
  console.log(timetable.passings);
  console.log(
    `待避照査: ${StationService.findById(root, targetPassing.stationId)?.name}`,
    `通過側列車: ${timetable.trains.find((v) => v.id === targetPassing.passingTrainId)?.number}列車 ${StationService.findById(root, TrainSegmentService.getStartingStationId(root, timetable.trains.find((v) => v.id === targetPassing.passingTrainId)?.segments[0] || TrainSegment.default()) || '')?.name}${toTimeString(timetable.trains.find((v) => v.id === targetPassing.passingTrainId)?.segments[0].departureTime || 0)}発`,
    `停車側列車: ${timetable.trains.find((v) => v.id === targetPassing.stoppingTrainId)?.number}列車 ${StationService.findById(root, TrainSegmentService.getStartingStationId(root, timetable.trains.find((v) => v.id === targetPassing.stoppingTrainId)?.segments[0] || TrainSegment.default()) || '')?.name}${toTimeString(timetable.trains.find((v) => v.id === targetPassing.stoppingTrainId)?.segments[0].departureTime || 0)}発`,
  );
  // 通過列車
  const passingTrain = timetable.trains.find(
    (v) => v.id === targetPassing.passingTrainId,
  );
  // 待避列車
  const stoppingTrain = timetable.trains.find(
    (v) => v.id === targetPassing.stoppingTrainId,
  );
  if (!passingTrain || !stoppingTrain) {
    // 継続必要なし
    return [false, timetable];
  }
  // 待避列車到着時刻〜通過列車到着時刻の差が2分以内の場合
  // 通過列車の到着時刻を待避列車到着時刻+2分に設定する
  const passingArriveTime =
    passingTrain?.segments.find(
      (v) =>
        TrainSegmentService.getTerminalStationId(root, v) ===
        targetPassing.stationId,
    )?.arrivalTime || 0;
  const stoppingArriveTime =
    stoppingTrain?.segments.find(
      (v) =>
        TrainSegmentService.getTerminalStationId(root, v) ===
        targetPassing.stationId,
    )?.arrivalTime || 0;
  console.log(
    `到着時刻 停車${toTimeString(stoppingArriveTime)} 通過${toTimeString(passingArriveTime)}`,
  );
  const arrivalShift = passingArriveTime < stoppingArriveTime + 120;
  if (arrivalShift) {
    // 通過列車のシフト動作
    const shiftTime = stoppingArriveTime + 120 - passingArriveTime;
    console.log(`通過列車シフト発生 ${shiftTime}秒`);
    const startIndex = passingTrain?.segments.findIndex(
      (v) =>
        TrainSegmentService.getTerminalStationId(root, v) ===
        targetPassing.stationId,
    );
    for (
      let index = startIndex;
      index < passingTrain.segments.length;
      index++
    ) {
      if (index === startIndex) {
        passingTrain.segments[index].arrivalTime += shiftTime;
      } else {
        passingTrain.segments[index].arrivalTime += shiftTime;
        passingTrain.segments[index].departureTime += shiftTime;
      }
    }
    console.log(`到着時隔: ${passingArriveTime - stoppingArriveTime}秒`);
  }

  // 通過列車発車時刻〜待避列車発車時刻の差が2分以内の場合
  // 待避列車の発車時刻を通過列車発車時刻+2分に設定する
  const passingDepartureTime =
    passingTrain?.segments.find(
      (v) =>
        TrainSegmentService.getStartingStationId(root, v) ===
        targetPassing.stationId,
    )?.departureTime || 0;
  const stoppingDepartureTime =
    stoppingTrain?.segments.find(
      (v) =>
        TrainSegmentService.getStartingStationId(root, v) ===
        targetPassing.stationId,
    )?.departureTime || 0;
  const departureShift = stoppingDepartureTime < passingDepartureTime + 120;
  if (departureShift) {
    // 待避列車のシフト動作
    const shiftTime = passingDepartureTime + 120 - stoppingDepartureTime;
    console.log(`待避列車シフト発生 ${shiftTime}秒`);
    const startIndex = stoppingTrain.segments.findIndex(
      (v) =>
        TrainSegmentService.getStartingStationId(root, v) ===
        targetPassing.stationId,
    );
    for (
      let index = startIndex;
      index < stoppingTrain.segments.length;
      index++
    ) {
      stoppingTrain.segments[index].arrivalTime += shiftTime;
      stoppingTrain.segments[index].departureTime += shiftTime;
    }
    console.log(`出発時隔: ${passingDepartureTime - stoppingDepartureTime}秒`);
  }

  if (arrivalShift) {
    // 通過列車の当駅以降の待避についても確認
    const startIndex = passingTrain?.segments.findIndex(
      (v) =>
        TrainSegmentService.getTerminalStationId(root, v) ===
        targetPassing.stationId,
    );
    for (const segment of passingTrain.segments.slice(startIndex + 1)) {
      const passing = timetable.passings.find(
        (v) =>
          v.stationId ===
            TrainSegmentService.getTerminalStationId(root, segment) &&
          (v.passingTrainId === passingTrain.id ||
            v.stoppingTrainId === passingTrain.id),
      );
      if (!passing) {
        continue;
      }
      const result = applyPassing(root, timetable, passing);
      timetable = result[1];
      if (!result[0]) {
        // これ以上更新の必要なし
        // break;
      }
    }
  }
  if (departureShift) {
    // 待避列車の当駅以降の待避についても確認
    const startIndex = stoppingTrain.segments.findIndex(
      (v) =>
        TrainSegmentService.getStartingStationId(root, v) ===
        targetPassing.stationId,
    );
    for (const segment of stoppingTrain.segments.slice(startIndex + 1)) {
      const passing = timetable.passings.find(
        (v) =>
          v.stationId ===
            TrainSegmentService.getTerminalStationId(root, segment) &&
          (v.passingTrainId === stoppingTrain.id ||
            v.stoppingTrainId === stoppingTrain.id),
      );
      if (!passing) {
        continue;
      }
      const result = applyPassing(root, timetable, passing);
      timetable = result[1];
      if (!result[0]) {
        // これ以上更新の必要なし
        // break;
      }
    }
  }

  return [true, timetable];
}

function getSimpleTrain(root: Root, train: Train) {
  return train;
  const newTrain = structuredClone(train) satisfies Train;
  for (let index = train.segments.length - 2; index >= 0; index--) {
    const segment = train.segments[index];
    const endStationId = segment.segments[0]?.isReversed
      ? SegmentService.findByIdAll(root, segment.segments[0]?.id)?.startId
      : SegmentService.findByIdAll(root, segment.segments[0]?.id)?.endId;
    if (endStationId == undefined) {
      return newTrain;
    }
    const endStation = StationService.findById(root, endStationId!);
    if (endStation?.kibo === 'Ippan') {
      // 駅結合作業
      const newSegment = structuredClone(segment) satisfies TrainSegment;
      const nextSegment = newTrain.segments[index + 1];
      newSegment.arrivalTime = nextSegment.arrivalTime;
      newSegment.segments.push(...nextSegment.segments);
      newTrain.segments.splice(index + 1, 1);
      newTrain.segments[index] = newSegment;
    }
  }
  return newTrain;
}

function getYChoords(root: Root, diagramLine: DiagramLine) {
  const result = [] as StationChoord[];
  let currentYChoord = 0;
  let beforeLastStationId = diagramLine.segments[0].isReversed
    ? SegmentService.findByIdAll(root, diagramLine.segments[0].id)?.endId
    : SegmentService.findByIdAll(root, diagramLine.segments[0].id)?.startId;

  for (const lineSegment of diagramLine.segments) {
    const segment = SegmentService.findByIdAll(root, lineSegment.id);
    if (segment == undefined) {
      throw new Error('Error');
    }
    const addSeconds = lineSegment.displaySeconds || 60;
    if (lineSegment.isReversed) {
      if (beforeLastStationId !== segment.endId) {
        currentYChoord += 120;
      }
      result.push({
        isReversed: true,
        lowerStationId: segment.endId,
        lowerStationYChoord: currentYChoord,
        segmentId: segment.id,
        upperStationId: segment.startId,
        upperStationYChoord: currentYChoord + addSeconds,
      } satisfies StationChoord);
      beforeLastStationId = segment.startId;
    } else {
      if (beforeLastStationId !== segment.startId) {
        currentYChoord += 120;
      }
      result.push({
        isReversed: false,
        lowerStationId: segment.endId,
        lowerStationYChoord: currentYChoord + addSeconds,
        segmentId: segment.id,
        upperStationId: segment.startId,
        upperStationYChoord: currentYChoord,
      } satisfies StationChoord);
      beforeLastStationId = segment.endId;
    }
    currentYChoord += addSeconds;
  }
  return result;
}
