import { useRef, useState } from 'react';

import { DiagramLineSegmentService } from '../globalState/DiagramLineSegmentService';
import { SegmentService } from '../globalState/SegmentService';
import { StationService } from '../globalState/StationService';
import useGlobalState from '../globalState/useGlobalState';
import { DiagramLineSegment } from '../sharpdia-model/DiagramLine';
import { TableViewer } from '../TableViewer/TableViewer';

export function DiagramLineViewer({
  diagramLineId,
}: {
  diagramLineId: string;
}) {
  const globalState = useGlobalState();

  if (!diagramLineId) {
    throw new Error('diagram line id null');
  }
  const diagramLineIndex = globalState.root.diagramLines.findIndex(
    (v) => v.id === diagramLineId,
  );
  const diagramLine = globalState.root.diagramLines[diagramLineIndex];
  if (!diagramLine) {
    throw new Error('diagram line is null');
  }

  const maxY = diagramLine.segments.length + 1;

  const dialogReference = useRef<HTMLDialogElement>(null);

  // ユーザが選択しているセルのX座標
  const [selectedCellX, setSelectedCellX] = useState(0);
  // ユーザが選択しているセルのY座標
  const [selectedCellY, setSelectedCellY] = useState(0);
  // 現在のウィンドウの状態
  const [editState, setEditState] = useState(
    'viwer' as 'edit' | 'insert' | 'new' | 'viewer',
  );
  const [editData, setEditData] = useState(DiagramLineSegment.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      const value = DiagramLineSegment.default();
      setEditData(value);
      setEditState('new');
    } else {
      setEditData(diagramLine.segments[y]);
      setEditState('edit');
    }
    dialogReference.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((previous) =>
      DiagramLineSegmentService.delete(
        previous,
        diagramLineIndex,
        selectedCellY,
      ),
    );
  };
  const insertData = () => {
    const value = DiagramLineSegment.default();
    setEditData(value);
    setEditState('insert');
    dialogReference.current?.showModal();
  };

  const onEndStationEnd = () => {
    switch (editState) {
      case 'edit': {
        globalState.setRoot((previous) =>
          DiagramLineSegmentService.update(
            previous,
            diagramLineIndex,
            selectedCellY,
            editData,
          ),
        );

        break;
      }
      case 'insert': {
        globalState.setRoot((previous) =>
          DiagramLineSegmentService.insert(
            previous,
            diagramLineIndex,
            selectedCellY,
            editData,
          ),
        );

        break;
      }
      case 'new': {
        setSelectedCellY(selectedCellY + 1);
        globalState.setRoot((previous) =>
          DiagramLineSegmentService.append(
            previous,
            diagramLineIndex,
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
              const segment = SegmentService.findByIdAll(
                globalState.root,
                value.id,
              );
              return `${
                StationService.findById(
                  globalState.root,
                  segment?.startId || '',
                )?.name
              }→${
                StationService.findById(globalState.root, segment?.endId || '')
                  ?.name
              }`;
            },
            headerText: '区間',
            widthIc: 10.4,
          },
        ]}
        data={diagramLine.segments}
        defaultValue={DiagramLineSegment.default()}
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
              区間
              <select
                onChange={(event) => {
                  editData.id = event.target.value;
                  setEditData({ ...editData });
                }}
                value={editData.id}
              >
                <option value="">選択してください……</option>
                {globalState?.root?.lines?.map((l) =>
                  l.segments?.map((segment, index) => (
                    <option value={segment.id}>
                      {index}.{l.name}{' '}
                      {
                        StationService.findById(
                          globalState.root,
                          segment.startId,
                        )?.name
                      }
                      -
                      {
                        StationService.findById(globalState.root, segment.endId)
                          ?.name
                      }
                    </option>
                  )),
                )}
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
