import { BrowserRouter, Route, Routes } from "react-router-dom";
import Index from "./Index";
import "./AppRoutes.css";
import { StationTableViewer } from "./StationViewer/StationTableViewer";
import { LinesViewer } from "./LinesViewer/LinesViewer";
import { Layout } from "./Layout";
import { LineViewer } from "./LineViewer/LineViewer";

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
          </Route>
        </Routes>
      </BrowserRouter>
    </>
  );
};

export default AppRoutes;
