import { Actions, DockLocation } from 'flexlayout-react';

import useGlobalState from '../globalState/useGlobalState';
import './TreeViewer.css';

export function TreeViewer() {
  const globalState = useGlobalState();

  const setCenter = () => {
    const center = globalState.model?.getNodeById('center');
    if (!center && globalState.model) {
      globalState.model.doAction(
        Actions.addNode(
          {
            id: 'center',
            type: 'tabset',
            weight: 100,
          },
          globalState.model.getRoot().getId(),
          DockLocation.CENTER,
          -1,
        ),
      );
    }
  };

  return (
    <div className="tree-view-outer bg-gray-100 p-2">
      <summary>プロジェクトファイル</summary>
      <ul>
        <li>
          <div
            onClick={() => {
              setCenter();
              globalState.model.doAction(
                Actions.addNode(
                  {
                    component: 'stations',
                    config: {},
                    name: '駅一覧',
                    type: 'tab',
                  },
                  'center',
                  DockLocation.CENTER,
                  -1,
                ),
              );
            }}
          >
            駅一覧
          </div>
        </li>
        <li>
          <div
            onClick={() => {
              setCenter();
              globalState.model.doAction(
                Actions.addNode(
                  {
                    component: 'train-types',
                    config: {},
                    name: '列車種別一覧',
                    type: 'tab',
                  },
                  'center',
                  DockLocation.CENTER,
                  -1,
                ),
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
                setCenter();
                globalState.model.doAction(
                  Actions.addNode(
                    {
                      component: 'lines',
                      config: {},
                      name: '路線一覧',
                      type: 'tab',
                    },
                    'center',
                    DockLocation.CENTER,
                    -1,
                  ),
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
                    setCenter();
                    globalState.model.doAction(
                      Actions.addNode(
                        {
                          component: 'line',
                          config: { lineId: v.id },
                          name: `路線 - ${v.name}`,
                          type: 'tab',
                        },
                        'center',
                        DockLocation.CENTER,
                        -1,
                      ),
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
                setCenter();
                globalState.model.doAction(
                  Actions.addNode(
                    {
                      component: 'templates',
                      config: {},
                      name: '列車テンプレート一覧',
                      type: 'tab',
                    },
                    'center',
                    DockLocation.CENTER,
                    -1,
                  ),
                );
              }}
            >
              列車テンプレート一覧
            </div>
          </summary>
          <ul>
            {globalState.root.templateTrains.map((v) => (
              <li className="h-6" key={v.id}>
                <div
                  onClick={() => {
                    setCenter();
                    globalState.model.doAction(
                      Actions.addNode(
                        {
                          component: 'template',
                          config: { templateId: v.id },
                          name: `列車テンプレート - ${v.name}`,
                          type: 'tab',
                        },
                        'center',
                        DockLocation.CENTER,
                        -1,
                      ),
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
                setCenter();
                globalState.model.doAction(
                  Actions.addNode(
                    {
                      component: 'timetables',
                      config: {},
                      name: '時刻表一覧',
                      type: 'tab',
                    },
                    'center',
                    DockLocation.CENTER,
                    -1,
                  ),
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
                      setCenter();
                      globalState.model.doAction(
                        Actions.addNode(
                          {
                            component: 'trains',
                            config: { timetableId: v.id },
                            name: `時刻表 - ${v.name}`,
                            type: 'tab',
                          },
                          'center',
                          DockLocation.CENTER,
                          -1,
                        ),
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
                          setCenter();
                          globalState.model.doAction(
                            Actions.addNode(
                              {
                                component: 'train',
                                config: { timetableId: v.id, trainId: k.id },
                                name: `列車 - ${k.number}`,
                                type: 'tab',
                              },
                              'center',
                              DockLocation.CENTER,
                              -1,
                            ),
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
                setCenter();
                globalState.model.doAction(
                  Actions.addNode(
                    {
                      component: 'diagram-lines',
                      config: {},
                      name: 'ダイヤ設定一覧',
                      type: 'tab',
                    },
                    'center',
                    DockLocation.CENTER,
                    -1,
                  ),
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
                    setCenter();
                    globalState.model.doAction(
                      Actions.addNode(
                        {
                          component: 'diagram-line',
                          config: { diagramLineId: v.id },
                          name: `ダイヤ設定 - ${v.name}`,
                          type: 'tab',
                        },
                        'center',
                        DockLocation.CENTER,
                        -1,
                      ),
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
                          setCenter();
                          globalState.model.doAction(
                            Actions.addNode(
                              {
                                component: 'diagram',
                                config: {
                                  diagramLineId: lines.id,
                                  timetableId: timetable.id,
                                },
                                name: `ダイヤグラム - ${lines.name} - ${timetable.name}`,
                                type: 'tab',
                              },
                              'center',
                              DockLocation.CENTER,
                              -1,
                            ),
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
