import { useRef, useState } from 'react';

import { SegmentService } from '../globalState/SegmentService';
import useGlobalState from '../globalState/useGlobalState';
import { Segment } from '../sharpdia-model/Line';
import { TableViewer } from '../TableViewer/TableViewer';

export function LineViewer({ lineId }: { lineId: string }) {
  const globalState = useGlobalState();
  if (!lineId) {
    throw new Error('line id null');
  }
  const lineIndex = globalState.root.lines.findIndex((v) => v.id === lineId);
  const line = globalState.root.lines[lineIndex];
  if (!line) {
    throw new Error('line is null');
  }

  const maxY = line.segments.length + 1;

  const dialogReference = useRef<HTMLDialogElement>(null);

  // ユーザが選択しているセルのX座標
  const [selectedCellX, setSelectedCellX] = useState(0);
  // ユーザが選択しているセルのY座標
  const [selectedCellY, setSelectedCellY] = useState(0);
  // 現在のウィンドウの状態
  const [editState, setEditState] = useState(
    'viwer' as 'edit' | 'insert' | 'new' | 'viewer',
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
      setEditState('new');
    } else {
      setEditData(line.segments[y]);
      setEditState('edit');
    }
    dialogReference.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((previous) =>
      SegmentService.delete(previous, lineIndex, selectedCellY),
    );
  };
  const insertData = () => {
    const value = Segment.default();
    const beforeSegment = line.segments[selectedCellY - 1];
    console.log(beforeSegment);
    if (beforeSegment) {
      value.startId = beforeSegment.endId;
    }
    setEditData(value);
    setEditState('insert');
    dialogReference.current?.showModal();
  };

  const onEndStationEnd = () => {
    switch (editState) {
      case 'edit': {
        globalState.setRoot((previous) =>
          SegmentService.update(previous, lineIndex, selectedCellY, editData),
        );

        break;
      }
      case 'insert': {
        globalState.setRoot((previous) =>
          SegmentService.insert(previous, lineIndex, selectedCellY, editData),
        );

        break;
      }
      case 'new': {
        setSelectedCellY(selectedCellY + 1);
        globalState.setRoot((previous) =>
          SegmentService.append(previous, lineIndex, editData),
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
            cellText: function (_: Segment, index: number): string {
              return String(index);
            },
            headerText: '番号',
            widthIc: 2,
          },
          {
            cellText: function (segment: Segment): string {
              const station = globalState.root.stations.find(
                (v) => v.id === segment.startId,
              )!;
              return station?.name;
            },
            headerText: '開始駅',
            widthIc: 6,
          },
          {
            cellText: function (segment: Segment): string {
              const station = globalState.root.stations.find(
                (v) => v.id === segment.endId,
              )!;
              return station?.name;
            },
            headerText: '終了駅',
            widthIc: 6,
          },
        ]}
        data={line.segments}
        defaultValue={Segment.default()}
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
              開始駅
              <select
                disabled={selectedCellY !== 0}
                onChange={(event) =>
                  setEditData({ ...editData, startId: event.target.value })
                }
                value={editData.startId}
              >
                <option value="">選択してください</option>
                {globalState.root.stations.map((v, index) => (
                  <option value={v.id}>
                    {index}. {v.name}
                  </option>
                ))}
              </select>
            </label>
            <label>
              終了駅
              <select
                onChange={(event) =>
                  setEditData({ ...editData, endId: event.target.value })
                }
                value={editData.endId}
              >
                <option value="">選択してください</option>
                {globalState.root.stations.map((v, index) => (
                  <option value={v.id}>
                    {index}. {v.name}
                  </option>
                ))}
              </select>
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
