import { useRef, useState } from 'react';

import { StationService } from '../globalState/StationService';
import { TimetableService } from '../globalState/TimetableService';
import { TrainService } from '../globalState/TrainService';
import { TrainTypeService } from '../globalState/TrainTypeService';
import useGlobalState from '../globalState/useGlobalState';
import { toTimeString } from '../sharpdia-model/TimeParser';
import { Train } from '../sharpdia-model/Train';
import { TableViewer } from '../TableViewer/TableViewer';

export function TrainsViewer({ timetableId }: { timetableId: string }) {
  const globalState = useGlobalState();
  if (!timetableId) {
    throw new Error('timetable id null');
  }
  const timetable = TimetableService.findById(globalState.root, timetableId);
  const timetableIndex = TimetableService.findIndexById(
    globalState.root,
    timetableId,
  );
  if (!timetable) {
    throw new Error('timetable is null');
  }
  const maxX = 4;
  const maxY = timetable.trains.length + 1;

  const dialogReference = useRef<HTMLDialogElement>(null);
  const inputReferences = useRef<(HTMLInputElement | null)[]>(
    Array.from({ length: maxX }, () => Object.create(null)),
  );

  // ユーザが選択しているセルのX座標
  const [selectedCellX, setSelectedCellX] = useState(0);
  // ユーザが選択しているセルのY座標
  const [selectedCellY, setSelectedCellY] = useState(0);
  // 現在のウィンドウの状態
  const [editState, setEditState] = useState(
    'viwer' as 'edit' | 'insert' | 'new' | 'viewer',
  );
  const [editData, setEditData] = useState(Train.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      setEditData(Train.default());
      setEditState('new');
    } else {
      setEditData(timetable.trains[y]);
      setEditState('edit');
    }
    dialogReference.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((previous) =>
      TrainService.delete(previous, timetableIndex, y),
    );
  };
  const insertData = () => {
    setEditData(Train.default());
    setEditState('insert');
    dialogReference.current?.showModal();
  };

  const onEndStationEnd = () => {
    switch (editState) {
      case 'edit': {
        globalState.setRoot((previous) =>
          TrainService.update(
            previous,
            timetableIndex,
            selectedCellY,
            editData,
          ),
        );

        break;
      }
      case 'insert': {
        globalState.setRoot((previous) =>
          TrainService.insert(
            previous,
            timetableIndex,
            selectedCellY,
            editData,
          ),
        );

        break;
      }
      case 'new': {
        globalState.setRoot((previous) =>
          TrainService.append(previous, timetableIndex, editData),
        );
        setSelectedCellY(selectedCellY + 1);

        break;
      }
      // No default
    }
    setEditState('viewer');
    dialogReference.current?.close();
  };

  return (
    <>
      <TableViewer
        columnSettings={[
          {
            cellText(_, index) {
              return String(index);
            },
            headerText: '#',
            widthIc: 2.4,
          },
          {
            cellText(value) {
              return value.number;
            },
            headerText: '列番',
            widthIc: 4.4,
          },
          {
            cellText(value) {
              return (
                TrainTypeService.findById(globalState.root, value.trainTypeId)
                  ?.name || ''
              );
            },
            headerText: '列車種別',
            widthIc: 6.4,
          },
          {
            cellText(value) {
              return (
                StationService.findById(
                  globalState.root,
                  TrainService.getStartingStation(globalState.root, value)
                    ?.startId || '',
                )?.name || ''
              );
            },
            headerText: '始発駅',
            widthIc: 6.4,
          },
          {
            cellText(value) {
              return toTimeString(value.segments[0]?.departureTime);
            },
            headerText: '始発時刻',
            widthIc: 4.9,
          },
          {
            cellText(value) {
              return toTimeString(value.segments.at(-1)?.arrivalTime || 0);
            },
            headerText: '終着時刻',
            widthIc: 4.9,
          },
          {
            cellText(value) {
              return (
                StationService.findById(
                  globalState.root,
                  TrainService.getDestinationStation(globalState.root, value)
                    ?.endId || '',
                )?.name || ''
              );
            },
            headerText: '終着駅',
            widthIc: 6.4,
          },
        ]}
        data={timetable.trains}
        defaultValue={Train.default()}
        deleteData={deleteData}
        editState={editState}
        insertData={insertData}
        selectedCellX={selectedCellX}
        selectedCellY={selectedCellY}
        setEditState={setEditState}
        setSelectedCellX={setSelectedCellX}
        setSelectedCellY={setSelectedCellY}
        startEdit={startEdit}
      />
      <div className="fixed">
        <dialog
          className="m-auto p-[1ic] rounded  shadow-xl"
          ref={dialogReference}
        >
          <form
            onSubmit={(event) => {
              event.preventDefault();
              onEndStationEnd();
            }}
          >
            <label>
              列車番号
              <input
                className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
                onChange={(event) =>
                  setEditData({ ...editData, number: event.target.value })
                }
                ref={(element) => {
                  inputReferences.current[0] = element;
                }}
                type="text"
                value={editData.number}
              />
            </label>
            <div className="mt-[1ic] flex justify-end gap-2">
              <button
                className="border-1 text-blue-400 border-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] rounded"
                onClick={() => {
                  setEditState('viewer');
                  dialogReference.current?.close();
                }}
                type="button"
              >
                キャンセル
              </button>
              <button
                className="bg-blue-400 p-[0.25ic] pl-[1ic] pr-[1ic] text-gray-50 rounded"
                onClick={() => {
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
