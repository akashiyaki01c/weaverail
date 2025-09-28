import { useState } from 'react';

import { SegmentService } from '../globalState/SegmentService';
import { StationService } from '../globalState/StationService';
import useGlobalState from '../globalState/useGlobalState';
import { DiagramLine } from '../sharpdia-model/DiagramLine';
import { Root } from '../sharpdia-model/Root';

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

  return (
    <>
      <div className="max-w-[100cqw] max-h-[100cqh] w-[100%] h-[100%] overflow-scroll">
        <div className="sticky top-[0] w-[max-content]">
          <svg
            className="x-axis-svg overflow-scroll sticky top-[0] left-[auto] block"
            height={50}
            viewBox={`0 0 ${viewBoxWidth} ${50}`}
            width={viewBoxWidth}
          >
            <rect
              className="fill-gray-100"
              height={50}
              width={viewBoxWidth}
              x={0}
              y={0}
            ></rect>
          </svg>
        </div>
        <div className="w-[max-content]">
          <div className="h-0">
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
                    .filter((v) => v % 2 === 0)
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
                    .filter((v) => v % 10 === 0)
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
                      let departureTime = segment.departureTime - 4 * 60 * 60;
                      if (departureTime < 0) {
                        departureTime += 24 * 60 * 60;
                      }
                      let arrivalTime = segment.arrivalTime - 4 * 60 * 60;
                      if (arrivalTime < 0) {
                        arrivalTime += 24 * 60 * 60;
                      }
                      return segment.segments[0].isReversed ? (
                        <line
                          stroke="#000"
                          strokeWidth="1"
                          x1={(departureTime + option.xOffset) * option.xScale}
                          x2={(arrivalTime + option.xOffset) * option.xScale}
                          y1={
                            (yChoords.find(
                              (v) => v.segmentId === segment.segments[0].id,
                            )?.lowerStationYChoord || 0 + option.yOffset) *
                              option.yScale +
                            yPadding
                          }
                          y2={
                            (yChoords.find(
                              (v) => v.segmentId === segment.segments[0].id,
                            )?.upperStationYChoord || 0 + option.yOffset) *
                              option.yScale +
                            yPadding
                          }
                        />
                      ) : (
                        <line
                          stroke="#000"
                          strokeWidth="1"
                          x1={(departureTime + option.xOffset) * option.xScale}
                          x2={(arrivalTime + option.xOffset) * option.xScale}
                          y1={
                            (yChoords.find(
                              (v) => v.segmentId === segment.segments[0].id,
                            )?.upperStationYChoord || 0 + option.yOffset) *
                              option.yScale +
                            yPadding
                          }
                          y2={
                            (yChoords.find(
                              (v) => v.segmentId === segment.segments[0].id,
                            )?.lowerStationYChoord || 0 + option.yOffset) *
                              option.yScale +
                            yPadding
                          }
                        />
                      );
                    })}
                  </g>
                ))}
              </g>
            </svg>
          </div>
          <div className="sticky left-[0]">
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
