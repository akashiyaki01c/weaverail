import { Layout, TabNode } from 'flexlayout-react';
import 'flexlayout-react/style/rounded.css';

import { OpenFile, SaveFile } from './commands/FileIO';
import useGlobalState from './globalState/useGlobalState';
import { DiagramLinesViewer } from './Viewer/DiagramLinesViewer';
import { DiagramLineViewer } from './Viewer/DiagramLineViewer';
import { DiagramViewer } from './Viewer/DiagramViewer';
import { LinesViewer } from './Viewer/LinesViewer';
import { LineViewer } from './Viewer/LineViewer';
import { StationTableViewer } from './Viewer/StationTableViewer';
import { TemplatesViewer } from './Viewer/TemplatesViewer';
import { TemplateViewer } from './Viewer/TemplateViewer';
import { TimetableViewer } from './Viewer/TimetableViewer';
import { TrainsViewer } from './Viewer/TrainsViewer';
import { TrainTypeViewer } from './Viewer/TrainTypeViewer';
import { TrainViewer } from './Viewer/TrainViewer';
import { TreeViewer } from './Viewer/TreeViewer';
import './App.css';

export function App() {
  const globalState = useGlobalState();
  const factory = (node: TabNode) => {
    const component = node.getComponent();

    if (component === 'stations') {
      return <StationTableViewer />;
    }
    if (component === 'lines') {
      return <LinesViewer />;
    }
    if (component === 'line') {
      const lineId = node.getConfig()?.lineId;
      return <LineViewer lineId={lineId} />;
    }
    if (component === 'train-types') {
      return <TrainTypeViewer />;
    }
    if (component === 'templates') {
      return <TemplatesViewer />;
    }
    if (component === 'template') {
      const templateId = node.getConfig()?.templateId;
      return <TemplateViewer templateId={templateId} />;
    }
    if (component === 'timetables') {
      return <TimetableViewer />;
    }
    if (component === 'trains') {
      const timetableId = node.getConfig()?.timetableId;
      return <TrainsViewer timetableId={timetableId} />;
    }
    if (component === 'train') {
      const timetableId = node.getConfig()?.timetableId;
      const trainId = node.getConfig()?.trainId;
      return <TrainViewer timetableId={timetableId} trainId={trainId} />;
    }
    if (component === 'diagram-lines') {
      return <DiagramLinesViewer />;
    }
    if (component === 'diagram-line') {
      const diagramLineId = node.getConfig()?.diagramLineId;
      return <DiagramLineViewer diagramLineId={diagramLineId} />;
    }
    if (component === 'diagram') {
      const timetableId = node.getConfig()?.timetableId;
      const diagramLineId = node.getConfig()?.diagramLineId;
      return (
        <DiagramViewer
          diagramLineId={diagramLineId}
          timetableId={timetableId}
        />
      );
    }
    if (component === 'tree') {
      return <TreeViewer />;
    }

    return <div>未定義ビュー</div>;
  };
  return (
    <>
      <div className="flex flex-col h-[100dvh]">
        <div className="page-outer">
          <div className="tool-bar w-[100%] h-[2ic] bg-gray-100 flex items-center">
            <button
              className="bg-gray-200 p-[0.15ic] border-1 border-gray-600 rounded"
              onClick={() => OpenFile(globalState)}
            >
              開く…
            </button>
            <button
              className="bg-gray-200 p-[0.15ic] border-1 border-gray-600 rounded"
              onClick={() => SaveFile(globalState)}
            >
              保存…
            </button>
          </div>
        </div>
        <div className="flex-1 relative">
          <Layout
            factory={factory}
            model={globalState.model}
            realtimeResize={true}
          />
        </div>
      </div>
    </>
  );
}
