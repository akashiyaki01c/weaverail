import { BrowserRouter, Route, Routes } from "react-router-dom";
import Index from "./Index";
import "./AppRoutes.css";
import { StationViewer } from "./StationViewer/StationViewer";

const AppRoutes = () => {
  return (
    <>
      <div className="tree-view-outer bg-gray-100 border-r-1 border-r-gray-600 p-2">
        <ul>
          <li>
            <a href="/stations">駅一覧</a>
          </li>
          <li>
            <a href="/traintypes">列車種別一覧</a>
          </li>
          <li>
            <summary>路線一覧</summary>
            <ul>
              <li>〜〜〜線</li>
              <li>〜〜〜線</li>
            </ul>
          </li>
        </ul>
      </div>
      <div className="page-outer">
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<Index />} />
            <Route path="/stations" element={<StationViewer />} />
          </Routes>
        </BrowserRouter>
      </div>
    </>
  );
};

export default AppRoutes;
