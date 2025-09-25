import { useState } from "react";
import useGlobalState from "../globalState/useGlobalState";
import { useParams } from "react-router-dom";
import { DiagramLine } from "../sharpdia-model/DiagramLine";
import { Root } from "../sharpdia-model/Root";
import { SegmentService } from "../globalState/SegmentService";
import { StationService } from "../globalState/StationService";

interface StationChoord {
  // 駅間ID
  segmentId: string;
  // ダイヤグラム上での上側の駅ID
  upperStationId: string;
  // ダイヤグラム上での上側の駅のY座標
  upperStationYChoord: number;
  // ダイヤグラム上での下側の駅ID
  lowerStationId: string;
  // ダイヤグラム上での下側の駅のY座標
  lowerStationYChoord: number;
}
function getYChoords(root: Root, diagramLine: DiagramLine) {
  const result = [] as StationChoord[];
  let currentYChoord = 0;

  for (const lineSegment of diagramLine.segments) {
    const segment = SegmentService.findByIdAll(root, lineSegment.id);
    if (segment == null) {
      throw new Error("Error");
    }
		const addSeconds = lineSegment.displaySeconds || 5;
    if (lineSegment.isReversed) {
      result.push({
        segmentId: segment.id,
        lowerStationId: segment.startId,
        lowerStationYChoord: currentYChoord,
        upperStationId: segment.endId,
        upperStationYChoord: currentYChoord + addSeconds,
      } satisfies StationChoord);
    } else {
      result.push({
        segmentId: segment.id,
        upperStationId: segment.startId,
        upperStationYChoord: currentYChoord,
        lowerStationId: segment.endId,
        lowerStationYChoord: currentYChoord + addSeconds,
      } satisfies StationChoord);
    }
    currentYChoord += addSeconds;
  }
  return result;
}

export function DiagramViewer() {
  const globalState = useGlobalState();
  const params = useParams();
  const diagramLineId = params.diagramLineId;
  if (!diagramLineId) {
    throw new Error("diagram line id null");
  }
  const diagramLineIndex = globalState.root.diagramLines.findIndex(
    (v) => v.id === diagramLineId
  );
  const diagramLine = globalState.root.diagramLines[diagramLineIndex];
  if (!diagramLine) {
    throw new Error("diagram line is null");
  }
  const timetableId = params.timetableId;
  if (!timetableId) {
    throw new Error("timetable id null");
  }
  const timetableIndex = globalState.root.timetables.findIndex(
    (v) => v.id === timetableId
  );
  const timetable = globalState.root.timetables[timetableIndex];
  if (!timetable) {
    throw new Error("timetable is null");
  }
  const yChoords = getYChoords(globalState.root, diagramLine);

  const [option, _setOption] = useState(new DiagramViewerOption(.1, .3, 0, 0));

  const yPadding = 50;
  const viewBoxWidth = option.xScale * 60 * 60 * 24;
  const viewBoxHeight =
    option.yScale * yChoords[yChoords.length - 1]?.lowerStationYChoord + yPadding * 2;

  return (
    <>
      <div className="max-w-[100cqw] max-h-[100cqh] w-[100%] h-[100%] overflow-scroll">
        <div className="sticky top-[0] w-[max-content]">
          <svg
            className="x-axis-svg overflow-scroll sticky top-[0] left-[auto] block"
            width={viewBoxWidth}
            height={50}
            viewBox={`0 0 ${viewBoxWidth} ${50}`}
          >
            <rect
              x={0}
              y={0}
              width={viewBoxWidth}
              height={50}
              className="fill-gray-100"
            ></rect>
          </svg>
        </div>
        <div className="w-[max-content]">
          <div className="h-0">
            <svg
              className="diagram-main-svg overflow-x-scroll overflow-y-hidden relative top-0 left-[120px] block"
              viewBox={`0 0 ${viewBoxWidth} ${viewBoxHeight}`}
              width={viewBoxWidth}
              height={viewBoxHeight}
            >
              <g className="axis">
                <g className="y-axis">
                  {yChoords.map((v) => (
                    <>
                      <line
                        key={`${v.segmentId}-upper`}
                        id={`${v.segmentId}-upper`}
                        x1={0}
                        x2={60 * 60 * 24 * option.xScale}
                        y1={
                          (v.upperStationYChoord + option.yOffset) *
                          option.yScale + yPadding
                        }
                        y2={
                          (v.upperStationYChoord + option.yOffset) *
                          option.yScale + yPadding
                        }
                        stroke="#000"
                        strokeWidth="0.5"
                      />
                      <line
                        key={`${v.segmentId}-lower`}
                        id={`${v.segmentId}-lower`}
                        x1={0}
                        x2={60 * 60 * 24 * option.xScale}
                        y1={
                          (v.lowerStationYChoord + option.yOffset) *
                          option.yScale + yPadding
                        }
                        y2={
                          (v.lowerStationYChoord + option.yOffset) *
                          option.yScale + yPadding
                        }
                        stroke="#000"
                        strokeWidth="0.5"
                      />
                    </>
                  ))}
                </g>
                <g className="x-axis">
                  {[...Array(24 * 60)].map((_, i) => i).filter((v) => v % 2 === 0).map((v) => (
                    <line
                      key={v}
                      x1={(option.xOffset + v * 60) * option.xScale}
                      x2={(option.xOffset + v * 60) * option.xScale}
                      y1={yPadding}
                      y2={viewBoxHeight + yPadding}
                      stroke="#000"
                      strokeWidth="0.5"
                      strokeDasharray="1 1"
                    />
                  ))}
                  {[...Array(24 * 60)].map((_, i) => i).filter((v) => v % 10 === 0).map((v) => (
                    <line
                      id={`${v}-1`}
                      x1={(option.xOffset + v * 60) * option.xScale}
                      x2={(option.xOffset + v * 60) * option.xScale}
                      y1={yPadding}
                      y2={viewBoxHeight + yPadding}
                      stroke="#000"
                      strokeWidth="1"
                    />
                  ))}
                  {[...Array(24 * 60)].map((_, i) => i).filter((v) => v % 30 === 0).map((v) => (
                    <line
                      id={`${v}-1`}
                      x1={(option.xOffset + v * 60) * option.xScale}
                      x2={(option.xOffset + v * 60) * option.xScale}
                      y1={yPadding}
                      y2={viewBoxHeight + yPadding}
                      stroke="#000"
                      strokeWidth="2"
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
											if (segment.segments[0].isReversed) {
												return (
                        <line
                          x1={(departureTime + option.xOffset) * option.xScale}
                          x2={(arrivalTime + option.xOffset) * option.xScale}
                          y2={
                            (yChoords.find(
                              (v) => v.segmentId === segment.segments[0].id
                            )?.upperStationYChoord || 0 + option.yOffset) *
                            option.yScale + yPadding
                          }
                          y1={
                            (yChoords.find(
                              (v) => v.segmentId === segment.segments[0].id
                            )?.lowerStationYChoord || 0 + option.yOffset) *
                            option.yScale + yPadding
                          }
                          stroke="#000"
                          strokeWidth="1"
                        />
                      );
											}
                      else {
												return (
                        <line
                          x1={(departureTime + option.xOffset) * option.xScale}
                          x2={(arrivalTime + option.xOffset) * option.xScale}
                          y1={
                            (yChoords.find(
                              (v) => v.segmentId === segment.segments[0].id
                            )?.upperStationYChoord || 0 + option.yOffset) *
                            option.yScale + yPadding
                          }
                          y2={
                            (yChoords.find(
                              (v) => v.segmentId === segment.segments[0].id
                            )?.lowerStationYChoord || 0 + option.yOffset) *
                            option.yScale + yPadding
                          }
                          stroke="#000"
                          strokeWidth="1"
                        />
                      );
											}
                    })}
                  </g>
                ))}
              </g>
            </svg>
          </div>
          <div className="sticky left-[0]">
            <svg
              className="y-axis-svg overflow-scroll sticky left-[0] top-[auto] block"
              width={120}
              height={viewBoxHeight}
              viewBox={`0 0 ${120} ${viewBoxHeight}`}
            >
              <rect
                x={0}
                y={0}
                width={120}
                height={viewBoxHeight}
                className="fill-gray-100"
              ></rect>
              {yChoords.map((v) => (
                <>
                  <text
                    key={`${v.segmentId}-upper-text`}
                    x={60}
                    y={(v.upperStationYChoord + option.yOffset) * option.yScale + yPadding}
                    textAnchor="middle"
                    dominantBaseline="text-before-edge"
                  >
                    {
                      StationService.findById(
                        globalState.root,
                        v.upperStationId
                      )?.name
                    }
                  </text>
                  <line
                    key={`${v.segmentId}-upper`}
                    id={`${v.segmentId}-upper`}
                    x1={0}
                    x2={60 * 60 * 24 * option.xScale}
                    y1={
                      (v.upperStationYChoord + option.yOffset) * option.yScale + yPadding
                    }
                    y2={
                      (v.upperStationYChoord + option.yOffset) * option.yScale + yPadding
                    }
                    stroke="#000"
                    strokeWidth="1"
                  />
									<text
                    key={`${v.segmentId}-lower-text`}
                    x={60}
                    y={(v.lowerStationYChoord + option.yOffset) * option.yScale + yPadding}
                    textAnchor="middle"
                    dominantBaseline="text-before-edge"
                  >
                    {
                      StationService.findById(
                        globalState.root,
                        v.lowerStationId
                      )?.name
                    }
                  </text>
                  <line
                    key={`${v.segmentId}-lower`}
                    id={`${v.segmentId}-lower`}
                    x1={0}
                    x2={60 * 24 * option.xScale}
                    y1={
                      (v.lowerStationYChoord + option.yOffset) * option.yScale + yPadding
                    }
                    y2={
                      (v.lowerStationYChoord + option.yOffset) * option.yScale + yPadding
                    }
                    stroke="#000"
                    strokeWidth="1"
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

export class DiagramViewerOption {
  constructor(
    public xScale: number,
    public yScale: number,
    public xOffset: number,
    public yOffset: number
  ) {}
}
