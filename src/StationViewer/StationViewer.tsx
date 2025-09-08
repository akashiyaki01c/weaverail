import { KeyboardEvent, useRef, useState } from "react";
import useGlobalState from "../useGlobalState";
import { Station } from "../sharpdia-model/Station";

export function StationViewer() {
  const globalState = useGlobalState();
  const maxX = 4;
  const maxY = globalState.stations.length + 1;

  const dialogRef = useRef<HTMLDialogElement>(null);
  const cellRefs = useRef<(HTMLTableCellElement | null)[][]>([]);
  const inputRefs = useRef<(HTMLInputElement | null)[]>(Array(maxX).fill(null));
  if (cellRefs.current.length !== globalState.stations.length + 1) {
    cellRefs.current = Array(globalState.stations.length + 1)
      .fill(null)
      .map(() => Array(maxX).fill(null));
  }

  // ユーザが選択しているセルのX座標
  const [selectedCellX, setSelectedCellX] = useState(0);
  // ユーザが選択しているセルのY座標
  const [selectedCellY, setSelectedCellY] = useState(0);
  // 現在のウィンドウの状態
  const [editState, setEditState] = useState(
    "viwer" as "viewer" | "new" | "insert" | "edit"
  );
  const [editStation, setEditStation] = useState(Station.default());

  const scrollcell = () => {
    const cell = cellRefs?.current[selectedCellY]?.[selectedCellX];
    if (cell) {
      const rect = cell.getBoundingClientRect();
      const padding = 40;
      if (rect.top < padding) {
        // 上方向スクロール
        window.scrollBy({ top: rect.top - padding, behavior: "instant" });
      } else if (rect.bottom > window.innerHeight - padding) {
        // 下方向スクロール
        window.scrollBy({
          top: rect.bottom - window.innerHeight + padding,
          behavior: "instant",
        });
      }
    }
  };
  // セルを選択する
  const selectCell = (x: number, y: number) => {
    setSelectedCellX(x);
    setSelectedCellY(y);
  };
  // 選択を右移動
  const moveRight = () => {
    if (selectedCellX < maxX - 1) {
      setSelectedCellX(selectedCellX + 1);
    }
  };
  // 選択を右移動
  const moveLeft = () => {
    if (selectedCellX > 0) {
      setSelectedCellX(selectedCellX - 1);
    }
  };
  // 選択を下移動
  const moveDown = () => {
    if (selectedCellY < maxY - 1) {
      setSelectedCellY(selectedCellY + 1);
      scrollcell();
    }
  };
  // 選択を上移動
  const moveUp = () => {
    if (selectedCellY > 0) {
      setSelectedCellY(selectedCellY - 1);
      scrollcell();
    }
  };

  const onKeyDown = (e: KeyboardEvent) => {
    console.log(e.key);
    e.preventDefault();
    switch (e.key) {
      case "ArrowRight":
        moveRight();
        return;
      case "ArrowLeft":
        moveLeft();
        return;
      case "ArrowDown":
        moveDown();
        return;
      case "ArrowUp":
        moveUp();
        return;
      case "Enter":
        onStationEdit(selectedCellX, selectedCellY);
        return;
      case "Delete":
        globalState.setStations((prev) => {
          const array = prev.splice(selectedCellY, 1);
          return array;
        });
        cellRefs.current.splice(selectedCellY, 1);
        return;
      case "+":
        setEditStation(Station.default());
        setEditState("insert");
        dialogRef.current?.showModal();
        return;
    }
  };
  const onStationEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      setEditStation(Station.default());
      setEditState("new");
    } else {
      setEditStation(globalState.stations[y]);
      setEditState("edit");
    }
    dialogRef.current?.showModal();
  };
  const onEndStationEnd = () => {
    if (editState === "edit") {
      globalState.updateStation(selectedCellY, editStation);
    } else if (editState === "insert") {
      globalState.setStations((prev) => {
        const array = prev.splice(selectedCellY, 0, editStation);
        return array;
      });
    } else if (editState === "new") {
      setSelectedCellY(selectedCellY + 1);
      scrollcell();
      globalState.setStations((prev) => [...prev, editStation]);
    }
    setEditState("viewer");
    dialogRef.current?.close();
  };

  const cellDefaultClass = (width: number, x: number, y: number) => {
    let borderSting = "";
    if (y !== maxY - 1) {
      borderSting += " border-b-[1px]";
    }
    if (x !== maxX - 1) {
      borderSting += " border-r-[1px]";
    }
    return `${borderSting} pl-[0.2ic] pr-[0.2ic] border-solid border-gray-600 overflow-hidden w-[${width}ic] ${
      x === selectedCellX && y === selectedCellY ? "bg-gray-200" : ""
    }`;
  };
  return (
    <>
      <div className="m-2">
        <div
          className="border-[2px] border-solid border-gray-600 w-fit outline-none"
          onKeyDown={onKeyDown}
          tabIndex={-1}
        >
          <div className="sticky top-0 flex bg-gray-50 z-10 h-[1.5ic]">
            <div className={cellDefaultClass(2, 0, -1)}>番号</div>
            <div className={cellDefaultClass(6, 1, -1)}>駅名</div>
            <div className={cellDefaultClass(6, 2, -1)}>駅時刻表示</div>
            <div className={cellDefaultClass(4, 3, -1)}>駅規模</div>
          </div>
          {globalState.stations.map((v, i) => (
            <>
              <div className="sticky top-0 flex z-0 h-[1.5ic]" key={v.id}>
                <div
                  className={cellDefaultClass(2, 0, i)}
                  onClick={() => selectCell(0, i)}
                  onDoubleClick={() => onStationEdit(0, i)}
                  ref={(el: HTMLTableCellElement | null) => {
                    if (!cellRefs.current[i])
                      cellRefs.current[i] = Array(maxX).fill(null);
                    if (!cellRefs.current[maxY - 1])
                      cellRefs.current[maxY - 1] = [null, null, null, null];
                    cellRefs.current[i][0] = el;
                  }}
                >
                  {i}
                </div>
                <div
                  className={cellDefaultClass(6, 1, i)}
                  onClick={() => selectCell(1, i)}
                  onDoubleClick={() => onStationEdit(1, i)}
                  ref={(el: HTMLTableCellElement | null) => {
                    if (!cellRefs.current[i])
                      cellRefs.current[i] = Array(maxX).fill(null);
                    if (!cellRefs.current[maxY - 1])
                      cellRefs.current[maxY - 1] = [null, null, null, null];
                    cellRefs.current[i][1] = el;
                  }}
                >
                  {v.name}
                </div>
                <div
                  className={cellDefaultClass(6, 2, i)}
                  onClick={() => selectCell(2, i)}
                  onDoubleClick={() => onStationEdit(2, i)}
                  ref={(el: HTMLTableCellElement | null) => {
                    if (!cellRefs.current[i])
                      cellRefs.current[i] = Array(maxX).fill(null);
                    if (!cellRefs.current[maxY - 1])
                      cellRefs.current[maxY - 1] = [null, null, null, null];
                    cellRefs.current[i][2] = el;
                  }}
                >
                  {v.displayType}
                </div>
                <div
                  className={cellDefaultClass(4, 3, i)}
                  onClick={() => selectCell(3, i)}
                  onDoubleClick={() => onStationEdit(3, i)}
                  ref={(el: HTMLTableCellElement | null) => {
                    if (!cellRefs.current[i])
                      cellRefs.current[i] = Array(maxX).fill(null);
                    if (!cellRefs.current[maxY - 1])
                      cellRefs.current[maxY - 1] = [null, null, null, null];
                    cellRefs.current[i][3] = el;
                  }}
                >
                  {v.kibo}
                </div>
              </div>
            </>
          ))}
          <div className="sticky top-0 flex z-0 h-[1.5ic]">
            <div
              className={cellDefaultClass(2, 0, maxY - 1)}
              onClick={() => selectCell(0, maxY - 1)}
              onDoubleClick={() => onStationEdit(0, maxY - 1)}
              ref={(el: HTMLTableCellElement | null) => {
                if (!cellRefs.current[maxY - 1])
                  cellRefs.current[maxY - 1] = [null, null, null, null];
                cellRefs.current[maxY - 1][0] = el;
              }}
            ></div>
            <div
              className={cellDefaultClass(6, 1, maxY - 1)}
              onClick={() => selectCell(1, maxY - 1)}
              onDoubleClick={() => onStationEdit(1, maxY - 1)}
              ref={(el: HTMLTableCellElement | null) => {
                if (!cellRefs.current[maxY - 1])
                  cellRefs.current[maxY - 1] = [null, null, null, null];
                cellRefs.current[maxY - 1][1] = el;
              }}
            ></div>
            <div
              className={cellDefaultClass(6, 2, maxY - 1)}
              onClick={() => selectCell(2, maxY - 1)}
              onDoubleClick={() => onStationEdit(2, maxY - 1)}
              ref={(el: HTMLTableCellElement | null) => {
                if (!cellRefs.current[maxY - 1])
                  cellRefs.current[maxY - 1] = [null, null, null, null];
                cellRefs.current[maxY - 1][2] = el;
              }}
            ></div>
            <div
              className={cellDefaultClass(4, 3, maxY - 1)}
              onClick={() => selectCell(3, maxY - 1)}
              onDoubleClick={() => onStationEdit(3, maxY - 1)}
              ref={(el: HTMLTableCellElement | null) => {
                if (!cellRefs.current[maxY - 1])
                  cellRefs.current[maxY - 1] = [null, null, null, null];
                cellRefs.current[maxY - 1][3] = el;
              }}
            ></div>
          </div>
        </div>
        <div className="fixed">
          <dialog
            ref={dialogRef}
            className="m-auto p-[1ic] rounded-sm border-1 border-gray300 shadow-xl"
          >
            <form
              onSubmit={(e) => {
                e.preventDefault();
                onEndStationEnd();
              }}
            >
              <label>
                駅名
                <input
                  className="ml-[1ic] border-1 border-solid border-gray-600 rounded-sm"
                  type="text"
                  value={editStation.name}
                  ref={(el) => {
                    inputRefs.current[0] = el;
                  }}
                  onChange={(e) =>
                    setEditStation({ ...editStation, name: e.target.value })
                  }
                />
              </label>
              <div className="mt-[1ic] flex justify-end gap-2">
                <button
                  className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded-sm"
                  onClick={(_) => {
                    setEditState("viewer");
                    dialogRef.current?.close();
                  }}
                  type="button"
                >
                  キャンセル
                </button>
                <button
                  className="bg-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] text-gray-50 rounded-sm"
                  onClick={(_) => {
                    onEndStationEnd();
                  }}
                >
                  適用
                </button>
              </div>
            </form>
          </dialog>
        </div>
      </div>
    </>
  );
}
