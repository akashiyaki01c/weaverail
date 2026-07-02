import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { useEffect, useMemo, useRef, useState } from "react";
import { LineSegmentId, ResultSvg } from "@weaverail/types";

function DiagramViewer() {
  const { manager } = useExtensionManager();
  const outerRef = useRef<HTMLDivElement>(undefined);
  const svgRef = useRef<SVGElement>(undefined);
  const [timetableId, setTimetableId] = useState("");
  const [diagramViewSettingsId, setDiagramViewSettingsId] = useState("");
  const [svg, setSvg] = useState<ResultSvg>();
  const [warp, setWarp] = useState<{
    [x: LineSegmentId]: {
      upper_y: number;
      lower_y: number;
      segment_id: LineSegmentId;
      is_reversed: boolean;
    };
  }>();
  const maxY = useMemo(() => {
    let max = 0;
    for (const key in warp) {
      if (!Object.hasOwn(warp, key)) continue;
      const element = warp[key];
      max = Math.max(max, element.lower_y);
    }
    console.log(max);

    return max;
  }, [warp]);

  const [viewSettings, _setViewSettings] = useState({
    scale_x: 0.125,
    scale_y: 0.2,
    offset_x: 0,
    offset_y: 0,
  });
  const [timeRange, setTimeRange] = useState<[number, number]>([
    0,
    24 * 60 * 60,
  ]);
  const waiting = useRef(false);
  const [inited, setInited] = useState(false);

  useEffect(() => {
    (async () => {
      const root = await manager.api.data.getRoot();
      console.log(root);
      const timetableId = Object.keys(root.timetables)[0];
      setTimetableId(timetableId);

      const diagramViewSettingsId = Object.keys(root.diagram_view_settings)[0];
      setDiagramViewSettingsId(diagramViewSettingsId);
      setSvg(
        await manager.api.data.getSvg(
          timetableId,
          diagramViewSettingsId,
          viewSettings,
          Math.max(0, timeRange[0] - 30 * 60),
          Math.max(0, timeRange[1] + 30 * 60),
        ),
      );
      const warp = await manager.api.data.getWarpCoords(diagramViewSettingsId);
      console.log("WARP: ", warp);
      setWarp(warp);
      setInited(true);
    })();
  }, []);

  useEffect(() => {
    (async () => {
      if (!inited) {
        return;
      }
      setSvg(
        await manager.api.data.getSvg(
          timetableId,
          diagramViewSettingsId,
          viewSettings,
          Math.max(0, timeRange[0] - 30 * 60),
          Math.max(0, timeRange[1] + 30 * 60),
        ),
      );
    })();
  }, [timeRange]);

  const stationAxis = [];
  for (const key in warp) {
    if (!Object.hasOwn(warp, key)) continue;

    const warpV = warp[key as any];
    stationAxis.push(
      <line
        key={`${key}-upper`}
        x1={0}
        x2={24 * 60 * 60 * viewSettings.scale_x}
        y1={warpV.upper_y * viewSettings.scale_y}
        y2={warpV.upper_y * viewSettings.scale_y}
        stroke="black"
      />,
    );
    stationAxis.push(
      <line
        key={`${key}-lower`}
        x1={0}
        x2={24 * 60 * 60 * viewSettings.scale_x}
        y1={warpV.lower_y * viewSettings.scale_y}
        y2={warpV.lower_y * viewSettings.scale_y}
        stroke="black"
      />,
    );
  }

  const timeAxis = [];
  for (let i = timeRange[0]; i < timeRange[1]; i++) {
    if ((i / 60) % 60 === 0) {
      timeAxis.push(
        <line
          key={i}
          x1={i * viewSettings.scale_x}
          x2={i * viewSettings.scale_x}
          y1={0}
          y2={maxY * viewSettings.scale_y}
          stroke="black"
        />,
      );
    } else if ((i / 60) % 30 === 0) {
      timeAxis.push(
        <line
          key={i}
          x1={i * viewSettings.scale_x}
          x2={i * viewSettings.scale_x}
          y1={0}
          y2={maxY * viewSettings.scale_y}
          stroke="black"
          strokeWidth={0.5}
        />,
      );
    } else if ((i / 60) % 10 === 0) {
      timeAxis.push(
        <line
          key={i}
          x1={i * viewSettings.scale_x}
          x2={i * viewSettings.scale_x}
          y1={0}
          y2={maxY * viewSettings.scale_y}
          stroke="black"
          strokeWidth={0.25}
        />,
      );
    } else if ((i / 60) % 2 === 0) {
      timeAxis.push(
        <line
          key={i}
          x1={i * viewSettings.scale_x}
          x2={i * viewSettings.scale_x}
          y1={0}
          y2={maxY * viewSettings.scale_y}
          stroke="black"
          strokeWidth={0.25}
          strokeDasharray={"4 4"}
        />,
      );
    }
  }

  return (
    <>
      <div
        onScroll={() => {
          if (waiting.current) return;

          waiting.current = true;

          const toTimeString = (num: number) => {
            const hour = Math.floor(num / 60 / 60);
            const minute = Math.floor(num / 60) % 60;
            const second = num % 60;
            return `${hour.toString().padStart(2, "0")}:${minute.toString().padStart(2, "0")}:${second.toString().padStart(2, "0")}`;
          };

          const outerRect = outerRef.current?.getBoundingClientRect();
          const svgRect = svgRef.current?.getBoundingClientRect();
          const startCoords = (outerRect?.x || 0) - (svgRect?.x || 0);
          const startTime = startCoords / viewSettings.scale_x;
          const endTime =
            startTime + (outerRect?.width || 0) / viewSettings.scale_x;
          console.log([toTimeString(startTime), toTimeString(endTime)]);
          setTimeRange([Math.floor(startTime), Math.ceil(endTime)]);

          setTimeout(() => {
            waiting.current = false;
          }, 50);
        }}
        style={{ width: "100%", height: "100%", overflow: "scroll" }}
        ref={outerRef as any}
      >
        <div style={{ display: "flex" }}>
          <div
            style={{
              width: "100px",
              height: `${maxY * viewSettings.scale_y}px`,
              position: "fixed",
            }}
          >
          </div>
          <div style={{ marginLeft: "100px" }}>
            <svg
              width={24 * 60 * 60 * viewSettings.scale_x}
              height={maxY * viewSettings.scale_y}
              ref={svgRef as any}
            >
              <g className="station-axis">{stationAxis}</g>
              <g className="time-axis">{timeAxis}</g>
              <g className="trains">
                {svg?.trains.map((t) => (
                  <>
                    <path
                      stroke="black"
                      strokeWidth={2}
                      fill="none"
                      key={`${t.train_id}/}`}
                      d={t.path_string}
                      onClick={(e) => {
                        const rect = e.currentTarget.getBoundingClientRect();
                        const rawY = e.clientY - rect.top;

                        let segment = "";
                        for (const key in warp) {
                          if (!Object.hasOwn(warp, key)) continue;

                          const element = warp[key as any];
                          if (
                            element.upper_y * viewSettings.scale_y <= rawY &&
                            rawY <= element.lower_y * viewSettings.scale_y
                          ) {
                            segment = key;
                            break;
                          }
                        }
                        console.log(segment);
                      }}
                    ></path>
                  </>
                ))}
              </g>
            </svg>
          </div>
        </div>
      </div>
    </>
  );
}

export class DiagramViewerExtension implements WeaverailExtension {
  id: string = "weaverail.core.diagram-viewer";
  metadata = { name: "コア拡張機能", description: "" };
  init(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.diagram-viewer.main-panel",
      label: "",
      slot: "main",
      render: function (): React.ReactNode {
        return <DiagramViewer />;
      },
    });
  }
  destroy(): void {}
}
