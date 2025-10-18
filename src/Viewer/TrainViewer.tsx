import { useRef, useState } from 'react';

import { SegmentService } from '../globalState/SegmentService';
import { StationService } from '../globalState/StationService';
import { TimetableService } from '../globalState/TimetableService';
import { TrainSegmentService } from '../globalState/TrainSegmentService';
import { TrainService } from '../globalState/TrainService';
import useGlobalState from '../globalState/useGlobalState';
import { parseTime, toTimeString } from '../sharpdia-model/TimeParser';
import { TrainSegment } from '../sharpdia-model/Train';
import { TableViewer } from '../TableViewer/TableViewer';

export function TrainViewer({
  timetableId,
  trainId,
}: {
  timetableId: string;
  trainId: string;
}) {
  const maxX = 2;
  // ユーザが選択しているセルのX座標
  const [selectedCellX, setSelectedCellX] = useState(0);
  // ユーザが選択しているセルのY座標
  const [selectedCellY, setSelectedCellY] = useState(0);
  // 現在のウィンドウの状態
  const [editState, setEditState] = useState(
    'viwer' as 'edit' | 'insert' | 'new' | 'viewer',
  );
  const [editData, setEditData] = useState(TrainSegment.default());

  const dialogReference = useRef<HTMLDialogElement>(null);
  const inputReferences = useRef<(HTMLInputElement | null)[]>(
    Array.from({ length: maxX }, () => Object.create(null)),
  );
  const globalState = useGlobalState();
  if (!timetableId) {
    console.error('timetable id is null');
    return <>timetable id is null</>;
  }
  const timetable = TimetableService.findById(globalState.root, timetableId);
  const timetableIndex = TimetableService.findIndexById(
    globalState.root,
    timetableId,
  );
  if (!timetable) {
    console.error('timetable is null');
    return <>timetable is null</>;
  }
  if (!trainId) {
    console.error('train id is null');
    return <>train id is null</>;
  }
  const trainIndex = TrainService.findIndexById(
    globalState.root,
    timetableIndex,
    trainId,
  );
  const train = TrainService.findById(
    globalState.root,
    timetableIndex,
    trainId,
  );
  if (!train) {
    console.error('train is null');
    return <>train is null</>;
  }
  const maxY = train.segments.length + 1;

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      setEditData(TrainSegment.default());
      setEditState('new');
    } else {
      setEditData(train.segments[y]);
      setEditState('edit');
    }
    dialogReference.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((previous) =>
      TrainSegmentService.delete(previous, timetableIndex, trainIndex, y),
    );
  };
  const insertData = () => {
    setEditData(TrainSegment.default());
    setEditState('insert');
    dialogReference.current?.showModal();
  };

  const onEndStationEnd = () => {
    switch (editState) {
      case 'edit': {
        globalState.setRoot((previous) =>
          TrainSegmentService.update(
            previous,
            timetableIndex,
            trainIndex,
            selectedCellY,
            editData,
          ),
        );

        break;
      }
      case 'insert': {
        globalState.setRoot((previous) =>
          TrainSegmentService.insert(
            previous,
            timetableIndex,
            trainIndex,
            selectedCellY,
            editData,
          ),
        );

        break;
      }
      case 'new': {
        setSelectedCellY(selectedCellY + 1);
        globalState.setRoot((previous) =>
          TrainSegmentService.append(
            previous,
            timetableIndex,
            trainIndex,
            editData,
          ),
        );

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
            headerText: '番号',
            widthIc: 2.4,
          },
          {
            cellText(value) {
              return `${
                value?.segments
                  ?.map((v) =>
                    SegmentService.findByIdAll(globalState.root, v?.id),
                  )
                  .map(
                    (v) =>
                      `${
                        StationService.findById(
                          globalState.root,
                          v?.startId || '',
                        )?.name
                      }→${
                        StationService.findById(
                          globalState.root,
                          v?.endId || '',
                        )?.name
                      }`,
                  )
                  .join('/') || ''
              }`;
            },
            headerText: '区間',
            widthIc: 10.4,
          },
          {
            cellText(value) {
              return toTimeString(value.departureTime);
            },
            headerText: '発車時刻',
            widthIc: 5.4,
          },
          {
            cellText(value) {
              return toTimeString(value.arrivalTime);
            },
            headerText: '到着時刻',
            widthIc: 5.4,
          },
        ]}
        data={train.segments}
        defaultValue={TrainSegment.default()}
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
            className="flex flex-col gap-4"
            onSubmit={(event) => {
              event.preventDefault();
              onEndStationEnd();
            }}
          >
            <label>
              区間
              <div className="flex flex-col">
                {[
                  ...(editData?.segments || []),
                  { id: '', isReversed: false },
                ].map((v, index) => (
                  <select
                    key={v.id}
                    onChange={(event) => {
                      if (index >= editData?.segments?.length) {
                        editData?.segments?.push({ id: '', isReversed: false });
                      }
                      editData.segments[index].id = event.target.value;
                      setEditData({ ...editData });
                    }}
                    value={v.id}
                  >
                    <option value="">選択してください……</option>
                    {globalState?.root?.lines?.map((l) =>
                      l.segments?.map((segment) => (
                        <option value={segment.id}>
                          {l.name}{' '}
                          {
                            StationService.findById(
                              globalState.root,
                              segment.startId,
                            )?.name
                          }
                          -
                          {
                            StationService.findById(
                              globalState.root,
                              segment.endId,
                            )?.name
                          }
                        </option>
                      )),
                    )}
                  </select>
                ))}
              </div>
            </label>
            <label>
              発時刻
              <input
                className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
                onChange={(event) =>
                  setEditData({
                    ...editData,
                    departureTime: parseTime(event.target.value),
                  })
                }
                ref={(element) => {
                  inputReferences.current[0] = element;
                }}
                type="text"
                value={editData?.departureTime}
              />
            </label>
            <label>
              着時刻
              <input
                className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
                onChange={(event) =>
                  setEditData({
                    ...editData,
                    arrivalTime: parseTime(event.target.value),
                  })
                }
                ref={(element) => {
                  inputReferences.current[0] = element;
                }}
                type="text"
                value={editData?.arrivalTime}
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
