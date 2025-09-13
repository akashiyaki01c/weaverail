import { useParams } from "react-router-dom";
import useGlobalState from "../useGlobalState";
import { useRef, useState } from "react";
import { TableViewer } from "../TableViwer/TableViewer";
import { Segment } from "../sharpdia-model/Line";

export function LineViewer() {
  const globalState = useGlobalState();
  const params = useParams();
  const lineId = params.lineId;
  if (!lineId) {
    throw new Error("line id null");
  }
  const lineIndex = globalState.lines.findIndex((v) => v.id === lineId);
  const line = globalState.lines[lineIndex];
  if (!line) {
    throw new Error("line is null");
  }

  const maxX = 2;
  const maxY = line.segments.length + 1;

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
  const [editData, setEditData] = useState(Segment.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      const value = Segment.default();
      const beforeSegment = line.segments[selectedCellY - 1];
      console.log(beforeSegment);
      if (beforeSegment) {
        value.startId = beforeSegment.endId;
      }
      setEditData(value);
      setEditState("new");
    } else {
      setEditData(line.segments[y]);
      setEditState("edit");
    }
    dialogRef.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setSegments(lineIndex, (prev) => {
      prev.splice(y, 1);
      return prev;
    });
  };
  const insertData = (_: number) => {
    const value = Segment.default();
    const beforeSegment = line.segments[selectedCellY - 1];
    console.log(beforeSegment);
    if (beforeSegment) {
      value.startId = beforeSegment.endId;
    }
    setEditData(value);
    setEditState("insert");
    dialogRef.current?.showModal();
  };

  const onEndStationEnd = () => {
    if (editState === "edit") {
      globalState.setSegments(lineIndex, (prev) => {
        prev[selectedCellY] = editData;
        return prev;
      });
    } else if (editState === "insert") {
      globalState.setSegments(lineIndex, (prev) => {
        prev.splice(selectedCellY, 0, editData);
        return prev;
      });
    } else if (editState === "new") {
      setSelectedCellY(selectedCellY + 1);
      globalState.setSegments(lineIndex, (prev) => {
        prev.push(editData);
        return prev;
      });
    }
    setEditState("viewer");
    dialogRef.current?.close();
  };

  return (
    <>
      <TableViewer
        columnSettings={[
          {
            headerText: "番号",
            widthIc: 2,
            cellText: function (_: Segment, index: number): string {
              return String(index);
            },
          },
          {
            headerText: "開始駅",
            widthIc: 6,
            cellText: function (segment: Segment, _: number): string {
              const station = globalState.stations.find(
                (v) => v.id === segment.startId
              )!;
              return station?.name;
            },
          },
          {
            headerText: "終了駅",
            widthIc: 6,
            cellText: function (segment: Segment, _: number): string {
              const station = globalState.stations.find(
                (v) => v.id === segment.endId
              )!;
              return station?.name;
            },
          },
        ]}
        data={line.segments}
        defaultValue={Segment.default()}
        selectedCellX={selectedCellX}
        selectedCellY={selectedCellY}
        setSelectedCellX={setSelectedCellX}
        setSelectedCellY={setSelectedCellY}
        editState={editState}
        setEditState={setEditState}
        startEdit={startEdit}
        deleteData={deleteData}
        insertData={insertData}
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
              開始駅
              <select
                value={editData.startId}
                onChange={(e) =>
                  setEditData({ ...editData, startId: e.target.value })
                }
                disabled={selectedCellY !== 0}
              >
                <option value="">選択してください</option>
                {globalState.stations.map((v, i) => (
                  <option value={v.id}>
                    {i}. {v.name}
                  </option>
                ))}
              </select>
            </label>
            <label>
              終了駅
              <select
                value={editData.endId}
                onChange={(e) =>
                  setEditData({ ...editData, endId: e.target.value })
                }
              >
                <option value="">選択してください</option>
                {globalState.stations.map((v, i) => (
                  <option value={v.id}>
                    {i}. {v.name}
                  </option>
                ))}
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
