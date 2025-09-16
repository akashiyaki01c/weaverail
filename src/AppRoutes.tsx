import { BrowserRouter, Route, Routes } from "react-router-dom";
import Index from "./Index";
import "./AppRoutes.css";
import { StationTableViewer } from "./Viewer/StationTableViewer";
import { LinesViewer } from "./Viewer/LinesViewer";
import { Layout } from "./Layout";
import { LineViewer } from "./Viewer/LineViewer";
import { TrainTypeViewer } from "./Viewer/TrainTypeViewer";
import { TimetableViewer } from "./Viewer/TimetableViewer";
import { TrainsViewer } from "./Viewer/TrainsViewer";
import { TrainViewer } from "./Viewer/TrainViewer";
import { DiagramLinesViewer } from "./Viewer/DiagramLinesViewer";

const AppRoutes = () => {
  return (
    <>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Layout />}>
            <Route path="/" element={<Index />} />
            <Route path="/stations" element={<StationTableViewer />} />
            <Route path="/lines" element={<LinesViewer />} />
            <Route path="/lines/:lineId" element={<LineViewer />} />
            <Route path="/train-types" element={<TrainTypeViewer />} />
            <Route path="/timetables" element={<TimetableViewer />} />
            <Route path="/timetables/:timetableId" element={<TrainsViewer />} />
            <Route path="/timetables/:timetableId/:trainId" element={<TrainViewer />} />
            <Route path="/diagram-lines" element={<DiagramLinesViewer />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </>
  );
};

export default AppRoutes;
