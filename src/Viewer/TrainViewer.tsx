import { useParams } from "react-router-dom";
import useGlobalState from "../globalState/useGlobalState";
import { TimetableService } from "../globalState/TimetableService";
import { TrainService } from "../globalState/TrainService";
import { useRef, useState } from "react";
import { TrainSegment } from "../sharpdia-model/Train";
import { TableViewer } from "../TableViewer/TableViewer";
import { SegmentService } from "../globalState/SegmentService";
import { StationService } from "../globalState/StationService";
import { TrainSegmentService } from "../globalState/TrainSegmentService";
import { parseTime, toTimeString } from "../sharpdia-model/TimeParser";

export function TrainViewer() {
  const globalState = useGlobalState();
  const params = useParams();
  const timetableId = params.timetableId;
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
  const trainId = params.trainId;
  if (!trainId) {
    throw new Error("train id null");
  }
  const trainIndex = TrainService.findIndexById(
    globalState.root,
    timetableIndex,
    trainId
  );
  const train = TrainService.findById(
    globalState.root,
    timetableIndex,
    trainId
  );
  if (!train) {
    throw new Error("train is null");
  }

  const maxX = 2;
  const maxY = train.segments.length + 1;

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
  const [editData, setEditData] = useState(TrainSegment.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      setEditData(TrainSegment.default());
      setEditState("new");
    } else {
      setEditData(train.segments[y]);
      setEditState("edit");
    }
    dialogRef.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((prev) =>
      TrainSegmentService.delete(prev, timetableIndex, trainIndex, y)
    );
  };
  const insertData = (_: number) => {
    setEditData(TrainSegment.default());
    setEditState("insert");
    dialogRef.current?.showModal();
  };

  const onEndStationEnd = () => {
    if (editState === "edit") {
      globalState.setRoot((prev) =>
        TrainSegmentService.update(
          prev,
          timetableIndex,
          trainIndex,
          selectedCellY,
          editData
        )
      );
    } else if (editState === "insert") {
      globalState.setRoot((prev) =>
        TrainSegmentService.insert(
          prev,
          timetableIndex,
          trainIndex,
          selectedCellY,
          editData
        )
      );
    } else if (editState === "new") {
      setSelectedCellY(selectedCellY + 1);
      globalState.setRoot((prev) =>
        TrainSegmentService.append(prev, timetableIndex, trainIndex, editData)
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
        data={train.segments}
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
              return `${
                value?.segments
                  ?.map((v) =>
                    SegmentService.findByIdAll(globalState.root, v?.id)
                  )
                  .map(
                    (v) =>
                      `${
                        StationService.findById(
                          globalState.root,
                          v?.startId || ""
                        )?.name
                      }→${
                        StationService.findById(
                          globalState.root,
                          v?.endId || ""
                        )?.name
                      }`
                  )
                  .join("/") || ""
              }`;
            },
          },
          {
            headerText: "発車時刻",
            widthIc: 5.4,
            cellText(value, _) {
              return toTimeString(value.departureTime);
            },
          },
          {
            headerText: "到着時刻",
            widthIc: 5.4,
            cellText(value, _) {
              return toTimeString(value.arrivalTime);
            },
          },
        ]}
        defaultValue={TrainSegment.default()}
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
              {[
                ...(editData?.segments || []),
                { id: "", isReversed: false },
              ].map((v, i) => (
                <select
                  value={v.id}
                  key={v.id}
                  onChange={(e) => {
                    if (i >= editData?.segments?.length) {
                      editData?.segments?.push({ id: "", isReversed: false });
                    }
                    editData.segments[i].id = e.target.value;
                    setEditData({ ...editData });
                  }}
                >
                  <option value="">選択してください……</option>
                  {globalState?.root?.lines?.map((l) =>
                    l.segments?.map((segment) => (
                      <option value={segment.id}>
                        {l.name}{" "}
                        {
                          StationService.findById(
                            globalState.root,
                            segment.startId
                          )?.name
                        }
                         - 
                        {
                          StationService.findById(
                            globalState.root,
                            segment.endId
                          )?.name
                        }
                      </option>
                    ))
                  )}
                </select>
              ))}
            </label>
            <label>
              発時刻
              <input
                className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
                type="text"
                value={editData?.departureTime}
                ref={(el) => {
                  inputRefs.current[0] = el;
                }}
                onChange={(e) =>
                  setEditData({
                    ...editData,
                    departureTime: parseTime(e.target.value),
                  })
                }
              />
            </label>
            <label>
              着時刻
              <input
                className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
                type="text"
                value={editData?.arrivalTime}
                ref={(el) => {
                  inputRefs.current[0] = el;
                }}
                onChange={(e) =>
                  setEditData({
                    ...editData,
                    arrivalTime: parseTime(e.target.value),
                  })
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
