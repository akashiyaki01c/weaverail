import { useParams } from "react-router-dom";
import useGlobalState from "../globalState/useGlobalState";
import { useRef, useState } from "react";
import { DiagramLineSegment } from "../sharpdia-model/DiagramLine";
import { DiagramLineSegmentService } from "../globalState/DiagramLineSegmentService";
import { TableViewer } from "../TableViewer/TableViewer";
import { StationService } from "../globalState/StationService";
import { SegmentService } from "../globalState/SegmentService";

export function DiagramLineViewer() {
  const globalState = useGlobalState();
  const params = useParams();
  const diagramLineId = params.diagramLineId;
  if (!diagramLineId) {
    throw new Error("diagram line id null");
  }
  const diagramLineIndex = globalState.root.diagramLines.findIndex(
    (v) => v.id === diagramLineId
  );
  const diagramLine = globalState.root.diagramLines[diagramLineIndex];
  if (!diagramLine) {
    throw new Error("diagram line is null");
  }

  const maxY = diagramLine.segments.length + 1;

  const dialogRef = useRef<HTMLDialogElement>(null);

  // ユーザが選択しているセルのX座標
  const [selectedCellX, setSelectedCellX] = useState(0);
  // ユーザが選択しているセルのY座標
  const [selectedCellY, setSelectedCellY] = useState(0);
  // 現在のウィンドウの状態
  const [editState, setEditState] = useState(
    "viwer" as "viewer" | "new" | "insert" | "edit"
  );
  const [editData, setEditData] = useState(DiagramLineSegment.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      const value = DiagramLineSegment.default();
      setEditData(value);
      setEditState("new");
    } else {
      setEditData(diagramLine.segments[y]);
      setEditState("edit");
    }
    dialogRef.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((prev) =>
      DiagramLineSegmentService.delete(prev, diagramLineIndex, selectedCellY)
    );
  };
  const insertData = (_: number) => {
    const value = DiagramLineSegment.default();
    setEditData(value);
    setEditState("insert");
    dialogRef.current?.showModal();
  };

  const onEndStationEnd = () => {
    if (editState === "edit") {
      globalState.setRoot((prev) =>
        DiagramLineSegmentService.update(
          prev,
          diagramLineIndex,
          selectedCellY,
          editData
        )
      );
    } else if (editState === "insert") {
      globalState.setRoot((prev) =>
        DiagramLineSegmentService.insert(
          prev,
          diagramLineIndex,
          selectedCellY,
          editData
        )
      );
    } else if (editState === "new") {
      setSelectedCellY(selectedCellY + 1);
      globalState.setRoot((prev) =>
        DiagramLineSegmentService.append(prev, diagramLineIndex, editData)
      );
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
        data={diagramLine.segments}
        columnSettings={[
          {
            headerText: "番号",
            widthIc: 2.4,
            cellText(_, index) {
              return String(index);
            },
          },
          {
            headerText: "区間",
            widthIc: 10.4,
            cellText(value, _) {
              const segment = SegmentService.findByIdAll(
                globalState.root,
                value.id
              );
              return `${
                StationService.findById(
                  globalState.root,
                  segment?.startId || ""
                )?.name
              }→${
                StationService.findById(globalState.root, segment?.endId || "")
                  ?.name
              }`;
            },
          },
        ]}
        defaultValue={DiagramLineSegment.default()}
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
              区間
              <select
                value={editData.id}
                onChange={(e) => {
                  editData.id = e.target.value;
                  setEditData({ ...editData });
                }}
              >
                <option value="">選択してください……</option>
                {globalState?.root?.lines?.map((l) =>
                  l.segments?.map((segment, i) => (
                    <option value={segment.id}>
                      {i}.{l.name}{" "}
                      {
                        StationService.findById(
                          globalState.root,
                          segment.startId
                        )?.name
                      }
                      -
                      {
                        StationService.findById(globalState.root, segment.endId)
                          ?.name
                      }
                    </option>
                  ))
                )}
              </select>
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
