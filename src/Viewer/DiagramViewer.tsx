import { useRef, useState } from 'react';

import { SegmentService } from '../globalState/SegmentService';
import { StationService } from '../globalState/StationService';
import { TrainService } from '../globalState/TrainService';
import { TemplateTrainService } from '../globalState/TrainTemplateService';
import { TrainTypeService } from '../globalState/TrainTypeService';
import useGlobalState from '../globalState/useGlobalState';
import { DiagramLine } from '../sharpdia-model/DiagramLine';
import { Root } from '../sharpdia-model/Root';
import { TemplateTrain } from '../sharpdia-model/TemplateTrain';

interface StationChoord {
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
  const dialogReference = useRef<HTMLDialogElement>(null);
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
  const yChoords = getYChoords(globalState.root, diagramLine);

  const [option] = useState(new DiagramViewerOption(0.1, 0.3, 0, 0));

  const yPadding = 50;
  const viewBoxWidth = option.xScale * 60 * 60 * 24;
  const viewBoxHeight =
    option.yScale * (yChoords.at(-1)?.lowerStationYChoord || 0) + yPadding * 2;

  const [targetTemplates, setTargetTemplates] = useState<TemplateTrain[]>([]);
  const [clickStation, setCliCkStation] = useState('');
  const [clickTime, setClickTime] = useState(0);
  const [selectedTemplate, setSelectedTemplate] = useState<TemplateTrain>(
    TemplateTrain.default(),
  );
  const [selectedDepartureStation, setSelectedDepartureStation] =
    useState('-1');
  const [selectedArrivalStation, setSelectedArrivalStation] = useState('-1');

  return (
    <>
      <div className="max-w-[100cqw] max-h-[100cqh] w-[100%] h-[100%] overflow-scroll z-0">
        <div className="sticky top-[0] w-[max-content] z-10 bg-gray-100">
          <div className="left-[120px] relative">
            <svg
              className="x-axis-svg overflow-scroll sticky top-[0] block"
              height={50}
              viewBox={`0 0 ${viewBoxWidth} ${50}`}
              width={viewBoxWidth}
            >
              {Array.from({ length: 24 * 60 + 1 })
                .map((_, index) => index)
                .filter((v) => v % 60 === 0)
                .map((v) => {
                  let time = v + 4 * 60;
                  if (time < 0) {
                    time += 24 * 60;
                  }
                  return (
                    <text
                      dominantBaseline="middle"
                      fill="#000"
                      fontSize={32}
                      id={`${v}-1`}
                      textAnchor="middle"
                      x={(option.xOffset + v * 60) * option.xScale}
                      y={yPadding / 2}
                    >
                      {(time / 60) % 24}
                    </text>
                  );
                })}
            </svg>
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
                    <>
                      <rect
                        className="fill-gray-100"
                        height={10}
                        id={`${v.segmentId}-upper-toucher`}
                        key={`${v.segmentId}-upper-toucher`}
                        onClick={(event) => {
                          const svg = (event.target as SVGRectElement)
                            .ownerSVGElement;
                          if (!svg) return;

                          const pt = new DOMPoint(event.clientX, event.clientY);
                          const svgPoint = pt.matrixTransform(
                            svg.getScreenCTM()?.inverse(),
                          );
                          const svgX =
                            svgPoint.x / option.xScale - option.xOffset;
                          // 下り追加
                          setClickTime(svgX - (svgX % 10) + 60 * 60 * 4);
                          setCliCkStation(v.upperStationId);
                          setTargetTemplates(
                            globalState.root.templateTrains.filter((template) =>
                              template.segments.some(
                                (segment) =>
                                  segment.segments[0]?.id === v.segmentId &&
                                  !segment.segments[0]?.isReversed,
                              ),
                            ),
                          );
                          console.log(targetTemplates);
                          dialogReference.current?.show();
                        }}
                        width={60 * 60 * 24 * option.xScale}
                        x={0}
                        y={
                          (v.upperStationYChoord + option.yOffset) *
                            option.yScale +
                          yPadding
                        }
                      />
                      <line
                        id={`${v.segmentId}-upper`}
                        key={`${v.segmentId}-upper`}
                        stroke="#000"
                        strokeWidth="0.5"
                        x1={0}
                        x2={60 * 60 * 24 * option.xScale}
                        y1={
                          (v.upperStationYChoord + option.yOffset) *
                            option.yScale +
                          yPadding
                        }
                        y2={
                          (v.upperStationYChoord + option.yOffset) *
                            option.yScale +
                          yPadding
                        }
                      />
                      <rect
                        className="fill-gray-100"
                        height={10}
                        id={`${v.segmentId}-lower-toucher`}
                        key={`${v.segmentId}-lower-toucher`}
                        onClick={(event) => {
                          const svg = (event.target as SVGRectElement)
                            .ownerSVGElement;
                          if (!svg) return;

                          const pt = new DOMPoint(event.clientX, event.clientY);
                          const svgPoint = pt.matrixTransform(
                            svg.getScreenCTM()?.inverse(),
                          );
                          const svgX =
                            svgPoint.x / option.xScale - option.xOffset;
                          // 上り追加
                          setClickTime(svgX - (svgX % 10) + 60 * 60 * 4);
                          setCliCkStation(v.lowerStationId);
                          setTargetTemplates(
                            globalState.root.templateTrains.filter((template) =>
                              template.segments.some(
                                (segment) =>
                                  segment.segments[0]?.id === v.segmentId &&
                                  segment.segments[0]?.isReversed,
                              ),
                            ),
                          );
                          console.log(targetTemplates);
                          dialogReference.current?.show();
                        }}
                        width={60 * 60 * 24 * option.xScale}
                        x={0}
                        y={
                          (v.lowerStationYChoord + option.yOffset) *
                            option.yScale +
                          yPadding -
                          10
                        }
                      />
                      <line
                        id={`${v.segmentId}-lower`}
                        key={`${v.segmentId}-lower`}
                        stroke="#000"
                        strokeWidth="0.5"
                        x1={0}
                        x2={60 * 60 * 24 * option.xScale}
                        y1={
                          (v.lowerStationYChoord + option.yOffset) *
                            option.yScale +
                          yPadding
                        }
                        y2={
                          (v.lowerStationYChoord + option.yOffset) *
                            option.yScale +
                          yPadding
                        }
                      />
                    </>
                  ))}
                </g>
                <g className="x-axis">
                  {Array.from({ length: 24 * 60 })
                    .map((_, index) => index)
                    .filter((v) => v % 1 === 0)
                    .map((v) => (
                      <line
                        key={v}
                        stroke="#000"
                        strokeDasharray="1 1"
                        strokeWidth="0.5"
                        x1={(option.xOffset + v * 60) * option.xScale}
                        x2={(option.xOffset + v * 60) * option.xScale}
                        y1={yPadding}
                        y2={viewBoxHeight + yPadding}
                      />
                    ))}
                  {Array.from({ length: 24 * 60 })
                    .map((_, index) => index)
                    .filter((v) => v % 5 === 0)
                    .map((v) => (
                      <line
                        id={`${v}-1`}
                        stroke="#000"
                        strokeWidth="1"
                        x1={(option.xOffset + v * 60) * option.xScale}
                        x2={(option.xOffset + v * 60) * option.xScale}
                        y1={yPadding}
                        y2={viewBoxHeight + yPadding}
                      />
                    ))}
                  {Array.from({ length: 24 * 60 })
                    .map((_, index) => index)
                    .filter((v) => v % 30 === 0)
                    .map((v) => (
                      <line
                        id={`${v}-1`}
                        stroke="#000"
                        strokeWidth="2"
                        x1={(option.xOffset + v * 60) * option.xScale}
                        x2={(option.xOffset + v * 60) * option.xScale}
                        y1={yPadding}
                        y2={viewBoxHeight + yPadding}
                      />
                    ))}
                </g>
              </g>
              <g className="trains">
                {timetable.trains.map((train) => (
                  <g id={`train-${train.id}`}>
                    {train.segments.map((segment) => {
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
                        const lastRatio = (currentTime + startTime) / totalTime;
                        startTime += currentTime;
                        const departure =
                          departureTime +
                          (arrivalTime - departureTime) * startRatio;
                        const arrival =
                          departureTime +
                          (arrivalTime - departureTime) * lastRatio;
                        result.push(
                          segment.segments[0].isReversed ? (
                            <line
                              stroke={trainType?.color || '#000'}
                              strokeWidth="1"
                              x1={(departure + option.xOffset) * option.xScale}
                              x2={(arrival + option.xOffset) * option.xScale}
                              y1={
                                (yChoords.find((v) => v.segmentId === sg.id)
                                  ?.lowerStationYChoord || 0 + option.yOffset) *
                                  option.yScale +
                                yPadding
                              }
                              y2={
                                (yChoords.find((v) => v.segmentId === sg.id)
                                  ?.upperStationYChoord || 0 + option.yOffset) *
                                  option.yScale +
                                yPadding
                              }
                            />
                          ) : (
                            <line
                              stroke={trainType?.color || '#000'}
                              strokeWidth="1"
                              x1={(departure + option.xOffset) * option.xScale}
                              x2={(arrival + option.xOffset) * option.xScale}
                              y1={
                                (yChoords.find((v) => v.segmentId === sg.id)
                                  ?.upperStationYChoord || 0 + option.yOffset) *
                                  option.yScale +
                                yPadding
                              }
                              y2={
                                (yChoords.find((v) => v.segmentId === sg.id)
                                  ?.lowerStationYChoord || 0 + option.yOffset) *
                                  option.yScale +
                                yPadding
                              }
                            />
                          ),
                        );
                      }
                      return result;
                    })}
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
                    dominantBaseline="text-before-edge"
                    key={`${v.segmentId}-upper-text`}
                    textAnchor="middle"
                    x={60}
                    y={
                      (v.upperStationYChoord + option.yOffset) * option.yScale +
                      yPadding
                    }
                  >
                    {
                      StationService.findById(
                        globalState.root,
                        v.upperStationId,
                      )?.name
                    }
                  </text>
                  <line
                    id={`${v.segmentId}-upper`}
                    key={`${v.segmentId}-upper`}
                    stroke="#000"
                    strokeWidth="1"
                    x1={0}
                    x2={60 * 60 * 24 * option.xScale}
                    y1={
                      (v.upperStationYChoord + option.yOffset) * option.yScale +
                      yPadding
                    }
                    y2={
                      (v.upperStationYChoord + option.yOffset) * option.yScale +
                      yPadding
                    }
                  />
                  <text
                    dominantBaseline="text-before-edge"
                    key={`${v.segmentId}-lower-text`}
                    textAnchor="middle"
                    x={60}
                    y={
                      (v.lowerStationYChoord + option.yOffset) * option.yScale +
                      yPadding
                    }
                  >
                    {
                      StationService.findById(
                        globalState.root,
                        v.lowerStationId,
                      )?.name
                    }
                  </text>
                  <line
                    id={`${v.segmentId}-lower`}
                    key={`${v.segmentId}-lower`}
                    stroke="#000"
                    strokeWidth="1"
                    x1={0}
                    x2={60 * 24 * option.xScale}
                    y1={
                      (v.lowerStationYChoord + option.yOffset) * option.yScale +
                      yPadding
                    }
                    y2={
                      (v.lowerStationYChoord + option.yOffset) * option.yScale +
                      yPadding
                    }
                  />
                </>
              ))}
            </svg>
          </div>
        </div>
      </div>
      <div className="fixed z-50 top-0 left-0">
        <dialog
          className="m-auto p-[1ic] rounded shadow-xl"
          ref={dialogReference}
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
                  dialogReference.current?.close();
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
                  dialogReference.current?.close();
                }}
              >
                適用
              </button>
            </div>
          </form>
        </dialog>
      </div>
    </>
  );
}

function getYChoords(root: Root, diagramLine: DiagramLine) {
  const result = [] as StationChoord[];
  let currentYChoord = 0;

  for (const lineSegment of diagramLine.segments) {
    const segment = SegmentService.findByIdAll(root, lineSegment.id);
    if (segment == undefined) {
      throw new Error('Error');
    }
    const addSeconds = lineSegment.displaySeconds || 5;
    if (lineSegment.isReversed) {
      result.push({
        lowerStationId: segment.startId,
        lowerStationYChoord: currentYChoord,
        segmentId: segment.id,
        upperStationId: segment.endId,
        upperStationYChoord: currentYChoord + addSeconds,
      } satisfies StationChoord);
    } else {
      result.push({
        lowerStationId: segment.endId,
        lowerStationYChoord: currentYChoord + addSeconds,
        segmentId: segment.id,
        upperStationId: segment.startId,
        upperStationYChoord: currentYChoord,
      } satisfies StationChoord);
    }
    currentYChoord += addSeconds;
  }
  return result;
}
