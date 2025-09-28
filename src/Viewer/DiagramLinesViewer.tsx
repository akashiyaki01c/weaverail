import { useRef, useState } from 'react';

import { DiagramLineService } from '../globalState/DiagramLineService';
import useGlobalState from '../globalState/useGlobalState';
import { DiagramLine } from '../sharpdia-model/DiagramLine';
import { TableViewer } from '../TableViewer/TableViewer';

export function DiagramLinesViewer() {
  const globalState = useGlobalState();

  const maxX = 2;
  const maxY = globalState.root.diagramLines.length + 1;

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
  const [editData, setEditData] = useState(DiagramLine.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      setEditData(DiagramLine.default());
      setEditState('new');
    } else {
      setEditData(globalState.root.diagramLines[y]);
      setEditState('edit');
    }
    dialogReference.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((previous) => DiagramLineService.delete(previous, y));
  };
  const insertData = () => {
    setEditData(DiagramLine.default());
    setEditState('insert');
    dialogReference.current?.showModal();
  };

  const onEndStationEnd = () => {
    switch (editState) {
      case 'edit': {
        globalState.setRoot((previous) =>
          DiagramLineService.update(previous, selectedCellY, editData),
        );

        break;
      }
      case 'insert': {
        globalState.setRoot((previous) =>
          DiagramLineService.insert(previous, selectedCellY, editData),
        );

        break;
      }
      case 'new': {
        setSelectedCellY(selectedCellY + 1);
        globalState.setRoot((previous) =>
          DiagramLineService.append(previous, editData),
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
              return value.name;
            },
            headerText: 'ダイヤ設定名',
            widthIc: 6.4,
          },
        ]}
        data={globalState.root.diagramLines}
        defaultValue={DiagramLine.default()}
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
              ダイヤ設定名
              <input
                className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
                onChange={(event) =>
                  setEditData({ ...editData, name: event.target.value })
                }
                ref={(element) => {
                  inputReferences.current[0] = element;
                }}
                type="text"
                value={editData.name}
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
