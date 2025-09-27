import useGlobalState from "../globalState/useGlobalState";
import { TimetableService } from "../globalState/TimetableService";
import { TableViewer } from "../TableViewer/TableViewer";
import { useRef, useState } from "react";
import { Train } from "../sharpdia-model/Train";
import { TrainService } from "../globalState/TrainService";
import { StationService } from "../globalState/StationService";
import { toTimeString } from "../sharpdia-model/TimeParser";
import { TrainTypeService } from "../globalState/TrainTypeService";

export function TrainsViewer({ timetableId }: { timetableId: string }) {
  const globalState = useGlobalState();
  if (!timetableId) {
    throw new Error("timetable id null");
  }
  const timetable = TimetableService.findById(globalState.root, timetableId);
  const timetableIndex = TimetableService.findIndexById(
    globalState.root,
    timetableId
  );
  if (!timetable) {
    throw new Error("timetable is null");
  }
  const maxX = 4;
  const maxY = timetable.trains.length + 1;

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
  const [editData, setEditData] = useState(Train.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      setEditData(Train.default());
      setEditState("new");
    } else {
      setEditData(timetable.trains[y]);
      setEditState("edit");
    }
    dialogRef.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((prev) => TrainService.delete(prev, timetableIndex, y));
  };
  const insertData = (_: number) => {
    setEditData(Train.default());
    setEditState("insert");
    dialogRef.current?.showModal();
  };

  const onEndStationEnd = () => {
    if (editState === "edit") {
      globalState.setRoot((prev) =>
        TrainService.update(prev, timetableIndex, selectedCellY, editData)
      );
    } else if (editState === "insert") {
      globalState.setRoot((prev) =>
        TrainService.insert(prev, timetableIndex, selectedCellY, editData)
      );
    } else if (editState === "new") {
      globalState.setRoot((prev) =>
        TrainService.append(prev, timetableIndex, editData)
      );
      setSelectedCellY(selectedCellY + 1);
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
        data={timetable.trains}
        columnSettings={[
          {
            headerText: "#",
            widthIc: 2.4,
            cellText(_, index) {
              return String(index);
            },
          },
          {
            headerText: "列番",
            widthIc: 4.4,
            cellText(value, _) {
              return value.number;
            },
          },
          {
            headerText: "列車種別",
            widthIc: 6.4,
            cellText(value, _) {
              return (
                TrainTypeService.findById(globalState.root, value.trainTypeId)
                  ?.name || ""
              );
            },
          },
          {
            headerText: "始発駅",
            widthIc: 6.4,
            cellText(value, _) {
              return (
                StationService.findById(
                  globalState.root,
                  TrainService.getStartingStation(globalState.root, value)
                    ?.startId || ""
                )?.name || ""
              );
            },
          },
          {
            headerText: "始発時刻",
            widthIc: 4.9,
            cellText(value, _) {
              return toTimeString(value.segments[0]?.departureTime);
            },
          },
          {
            headerText: "終着時刻",
            widthIc: 4.9,
            cellText(value, _) {
              return toTimeString(
                value.segments[value.segments.length - 1]?.arrivalTime
              );
            },
          },
          {
            headerText: "終着駅",
            widthIc: 6.4,
            cellText(value, _) {
              return (
                StationService.findById(
                  globalState.root,
                  TrainService.getDestinationStation(globalState.root, value)
                    ?.endId || ""
                )?.name || ""
              );
            },
          },
        ]}
        defaultValue={Train.default()}
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
              列車番号
              <input
                className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
                type="text"
                value={editData.number}
                ref={(el) => {
                  inputRefs.current[0] = el;
                }}
                onChange={(e) =>
                  setEditData({ ...editData, number: e.target.value })
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
