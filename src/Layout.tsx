import { Link, Outlet } from "react-router-dom";
import useGlobalState from "./globalState/useGlobalState";
import { OpenFile, SaveFile } from "./commands/FileIO";
import "./Layout.css";

export function Layout() {
  const globalState = useGlobalState();
  return (
    <>
      <div className="tree-view-outer bg-gray-100 border-r-1 border-r-gray-600 p-2">
        <summary>摂播電鉄各線</summary>
        <ul>
          <li>
            <Link to="/stations">駅一覧</Link>
          </li>
          <li>
            <Link to="/train-types">列車種別一覧</Link>
          </li>
          <li>
            <summary>
              <Link to="/lines">物理路線一覧</Link>
            </summary>
            <ul>
              {globalState.root.lines.map((v) => (
                <li className="h-6" key={v.id}>
                  <Link to={`/lines/${v.id}`}>{v.name}</Link>
                </li>
              ))}
            </ul>
          </li>
          <li>
            <summary>
              <Link to="/timetables">時刻表一覧</Link>
            </summary>
            <ul>
              {globalState.root.timetables.map((v) => (
                <li key={v.id}>
                  <summary>
                    <Link to={`/timetables/${v.id}`}>{v.name}</Link>
                  </summary>
                  <ul>
                    {v.trains.map((k) => (
                      <li key={k.id}><Link to={`/timetables/${v.id}/${k.id}`}>
                        {k.number} 列車種別一覧
                      </Link></li>
                    ))}
                  </ul>
                </li>
              ))}
            </ul>
          </li>
          <li>
            <summary><Link to="/diagram-lines">ダイヤ設定一覧</Link></summary>
            <ul>
              {globalState.root.diagramLines.map(v => (<li>
                <Link to={`/diagram-lines/${v.id}`}>{v.name}</Link>
              </li>))}
            </ul>
          </li>
          <li>
            <summary>ダイヤグラム一覧</summary>
            <ul>
              <li>ダイヤグラム(〜〜線)</li>
              <li>ダイヤグラム(〜〜線)</li>
              <li>ダイヤグラム(〜〜線)</li>
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
        <Outlet />
      </div>
    </>
  );
}
