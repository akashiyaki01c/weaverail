import { Actions, DockLocation } from "flexlayout-react";
import useGlobalState from "../globalState/useGlobalState";
import "./TreeViewer.css";

export function TreeViewer() {
  const globalState = useGlobalState();
  return (
    <div className="tree-view-outer bg-gray-100 p-2">
      <summary>摂播電鉄各線</summary>
      <ul>
        <li>
          <div
            onClick={() => {
              globalState.model.doAction(
                Actions.addNode(
                  {
                    type: "tab",
                    name: "駅一覧",
                    component: "stations",
                    config: {},
                  },
                  "center",
                  DockLocation.CENTER,
                  -1
                )
              );
            }}
          >
            駅一覧
          </div>
        </li>
        <li>
          <div
            onClick={() => {
              globalState.model?.doAction(
                Actions.addNode(
                  {
                    type: "tab",
                    name: "列車種別一覧",
                    component: "train-types",
                    config: {},
                  },
                  "center",
                  DockLocation.CENTER,
                  -1
                )
              );
            }}
          >
            列車種別一覧
          </div>
        </li>
        <li>
          <summary>
            <div
              onClick={() => {
                globalState.model?.doAction(
                  Actions.addNode(
                    {
                      type: "tab",
                      name: "路線一覧",
                      component: "lines",
                      config: {},
                    },
                    "center",
                    DockLocation.CENTER,
                    -1
                  )
                );
              }}
            >
              路線一覧
            </div>
          </summary>
          <ul>
            {globalState.root.lines.map((v) => (
              <li className="h-6" key={v.id}>
                <div
                  onClick={() => {
                    globalState.model?.doAction(
                      Actions.addNode(
                        {
                          type: "tab",
                          name: `路線 - ${v.name}`,
                          component: "line",
                          config: { lineId: v.id },
                        },
                        "center",
                        DockLocation.CENTER,
                        -1
                      )
                    );
                  }}
                >
                  {v.name}
                </div>
              </li>
            ))}
          </ul>
        </li>
        <li>
          <summary>
            <div
              onClick={() => {
                globalState.model?.doAction(
                  Actions.addNode(
                    {
                      type: "tab",
                      name: "時刻表一覧",
                      component: "timetables",
                      config: {},
                    },
                    "center",
                    DockLocation.CENTER,
                    -1
                  )
                );
              }}
            >
              時刻表一覧
            </div>
          </summary>
          <ul>
            {globalState.root.timetables.map((v) => (
              <li key={v.id}>
                <summary>
                  <div
                    onClick={() => {
                      globalState.model?.doAction(
                        Actions.addNode(
                          {
                            type: "tab",
                            name: `時刻表 - ${v.name}`,
                            component: "timetable",
                            config: { timetableId: v.id },
                          },
                          "center",
                          DockLocation.CENTER,
                          -1
                        )
                      );
                    }}
                  >
                    {v.name}
                  </div>
                </summary>
                <ul>
                  {v.trains.map((k) => (
                    <li key={k.id}>
                      <div
                        onClick={() => {
                          console.log(globalState.model);
                          globalState.model?.doAction(
                            Actions.addNode(
                              {
                                type: "tab",
                                name: `列車 - ${k.number}`,
                                component: "train",
                                config: { timetableId: v.id, trainId: k.id },
                              },
                              "center",
                              DockLocation.CENTER,
                              -1
                            )
                          );
                        }}
                      >
                        {k.number}
                      </div>
                    </li>
                  ))}
                </ul>
              </li>
            ))}
          </ul>
        </li>
        <li>
          <summary>
            <div
              onClick={() => {
                globalState.model?.doAction(
                  Actions.addNode(
                    {
                      type: "tab",
                      name: "ダイヤ設定一覧",
                      component: "diagram-lines",
                      config: {},
                    },
                    "center",
                    DockLocation.CENTER,
                    -1
                  )
                );
              }}
            >
              ダイヤ設定一覧
            </div>
          </summary>
          <ul>
            {globalState.root.diagramLines.map((v) => (
              <li>
                <div
                  onClick={() => {
                    globalState.model?.doAction(
                      Actions.addNode(
                        {
                          type: "tab",
                          name: `ダイヤ設定 - ${v.name}`,
                          component: "diagram-line",
                          config: { diagramLineId: v.id },
                        },
                        "center",
                        DockLocation.CENTER,
                        -1
                      )
                    );
                  }}
                >
                  {v.name}
                </div>
              </li>
            ))}
          </ul>
        </li>
        <li>
          <summary>ダイヤグラム一覧</summary>
          <ul>
            {globalState.root.timetables.map((timetable) => (
              <li>
                <summary>{timetable.name}</summary>
                <ul>
                  {globalState.root.diagramLines.map((lines) => (
                    <li>
                      <div
                        onClick={() => {
                          globalState.model?.doAction(
                            Actions.addNode(
                              {
                                type: "tab",
                                name: `ダイヤグラム - ${lines.name} - ${timetable.name}`,
                                component: "diagram",
                                config: {
                                  diagramLineId: lines.id,
                                  timetableId: timetable.id,
                                },
                              },
                              "center",
                              DockLocation.CENTER,
                              -1
                            )
                          );
                        }}
                      >
                        ${lines.name}
                      </div>
                    </li>
                  ))}
                </ul>
              </li>
            ))}
          </ul>
        </li>
        <li>
          <summary>その他</summary>
          <ul>
            <li>一般設定</li>
          </ul>
        </li>
      </ul>
    </div>
  );
}
