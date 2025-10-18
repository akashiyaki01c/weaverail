import { useRef, useState } from 'react';

import { SegmentService } from '../globalState/SegmentService';
import { StationService } from '../globalState/StationService';
import { TemplateTrainSegmentService } from '../globalState/TemplateTrainSegmentService';
import { TemplateTrainService } from '../globalState/TrainTemplateService';
import useGlobalState from '../globalState/useGlobalState';
import { TemplateTrainSegment } from '../sharpdia-model/TemplateTrain';
import { TableViewer } from '../TableViewer/TableViewer';

export function TemplateViewer({ templateId }: { templateId: string }) {
  const globalState = useGlobalState();
  const templateIndex = TemplateTrainService.findIndexById(
    globalState.root,
    templateId,
  );
  const template = TemplateTrainService.findById(globalState.root, templateId);
  if (!template) {
    throw new Error('template is null');
  }
  const maxY = template.segments.length + 1;

  const dialogReference = useRef<HTMLDialogElement>(null);

  // ユーザが選択しているセルのX座標
  const [selectedCellX, setSelectedCellX] = useState(0);
  // ユーザが選択しているセルのY座標
  const [selectedCellY, setSelectedCellY] = useState(0);
  // 現在のウィンドウの状態
  const [editState, setEditState] = useState(
    'viwer' as 'edit' | 'insert' | 'new' | 'viewer',
  );
  const [editData, setEditData] = useState(TemplateTrainSegment.default());

  const startEdit = (_: number, y: number) => {
    if (y === maxY - 1) {
      setEditData(TemplateTrainSegment.default());
      setEditState('new');
    } else {
      setEditData(template.segments[y]);
      setEditState('edit');
    }
    dialogReference.current?.showModal();
  };
  const deleteData = (y: number) => {
    if (y === maxY - 1) {
      return;
    }
    globalState.setRoot((previous) =>
      TemplateTrainSegmentService.delete(previous, templateIndex, y),
    );
  };
  const insertData = () => {
    setEditData(TemplateTrainSegment.default());
    setEditState('insert');
    dialogReference.current?.showModal();
  };

  const onEndStationEnd = () => {
    switch (editState) {
      case 'edit': {
        globalState.setRoot((previous) =>
          TemplateTrainSegmentService.update(
            previous,
            templateIndex,
            selectedCellY,
            editData,
          ),
        );

        break;
      }
      case 'insert': {
        globalState.setRoot((previous) =>
          TemplateTrainSegmentService.insert(
            previous,
            templateIndex,
            selectedCellY,
            editData,
          ),
        );

        break;
      }
      case 'new': {
        setSelectedCellY(selectedCellY + 1);
        globalState.setRoot((previous) =>
          TemplateTrainSegmentService.append(previous, templateIndex, editData),
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
            cellText(v) {
              return String(v.time);
            },
            headerText: '運転時秒',
            widthIc: 4.4,
          },
        ]}
        data={template.segments}
        defaultValue={TemplateTrainSegment.default()}
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
            <input
              className="ml-[1ic] border-1 border-solid border-gray-600 rounded focus:outline-1 outline-offset-1 outline-blue-200 pl-1"
              onChange={(event) =>
                setEditData({
                  ...editData,
                  time: Number.parseInt(event.target.value) || 0,
                })
              }
              type="number"
              value={editData.time}
            />
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
