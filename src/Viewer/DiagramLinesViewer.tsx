import { useRef, useState } from "react";
import { TableViewer } from "../TableViewer/TableViewer";
import useGlobalState from "../globalState/useGlobalState";
import { DiagramLine } from "../sharpdia-model/DiagramLine";
import { DiagramLineService } from "../globalState/DiagramLineService";

export function DiagramLinesViewer() {
  const globalState = useGlobalState();

  const maxX = 2;
  const maxY = globalState.root.diagramLines.length + 1;

  const dialogRef = useRef<HTMLDialogElement>(null);
  const inputRefs = useRef<(HTMLInputElement | null)[]>(Array(maxX).fill(null));

  // ユーザが選択しているセルのX座標
  const [selectedCellX, setSelectedCellX] = useState(0);
  // ユーザが選択しているセルのY座標
  const [selectedCellY, setSelectedCellY] = useState(0);
  // 現在のウィンドウの状態
  const [editState, setEditState] = useState(
    "viwer" as "viewer" | "new" | "insert" | "edit"
  );
  const [editData, setEditData] = useState(DiagramLine.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      setEditData(DiagramLine.default());
      setEditState("new");
    } else {
      setEditData(globalState.root.diagramLines[y]);
      setEditState("edit");
    }
    dialogRef.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot(prev => DiagramLineService.delete(prev, y));
  };
  const insertData = (_: number) => {
    setEditData(DiagramLine.default());
    setEditState("insert");
    dialogRef.current?.showModal();
  };

  const onEndStationEnd = () => {
    if (editState === "edit") {
      globalState.setRoot(prev => DiagramLineService.update(prev, selectedCellY, editData));
    } else if (editState === "insert") {
      globalState.setRoot(prev => DiagramLineService.insert(prev, selectedCellY, editData));
    } else if (editState === "new") {
      setSelectedCellY(selectedCellY + 1);
      globalState.setRoot(prev => DiagramLineService.append(prev, editData));
    }
    setEditState("viewer");
    dialogRef.current?.close();
  };

  return (
    <>
      <TableViewer
        selectedCellX={selectedCellX}
        selectedCellY={selectedCellY}
        setSelectedCellX={setSelectedCellX}
        setSelectedCellY={setSelectedCellY}
        editState={editState}
        setEditState={setEditState}
        startEdit={startEdit}
        deleteData={deleteData}
        insertData={insertData}
        data={globalState.root.diagramLines}
        columnSettings={[
          {
            headerText: "番号",
            widthIc: 2.4,
            cellText(_, index) {
              return String(index);
            },
          },
          {
            headerText: "ダイヤ設定名",
            widthIc: 6.4,
            cellText(value, _) {
              return value.name;
            },
          },
        ]}
        defaultValue={DiagramLine.default()}
      />
      <div className="fixed">
        <dialog ref={dialogRef} className="m-auto p-[1ic] rounded  shadow-xl">
          <form
            onSubmit={(e) => {
              e.preventDefault();
              onEndStationEnd();
            }}
          >
            <label>
              ダイヤ設定名
              <input
                className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
                type="text"
                value={editData.name}
                ref={(el) => {
                  inputRefs.current[0] = el;
                }}
                onChange={(e) =>
                  setEditData({ ...editData, name: e.target.value })
                }
              />
            </label>
            <div className="mt-[1ic] flex justify-end gap-2">
              <button
                className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded"
                onClick={(_) => {
                  setEditState("viewer");
                  dialogRef.current?.close();
                }}
                type="button"
              >
                キャンセル
              </button>
              <button
                className="bg-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] text-gray-50 rounded"
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
    </>
  );
}
