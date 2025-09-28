import { KeyboardEvent, useRef } from 'react';

interface Keyable {
  key: string;
}

type Properties<T> = {
  columnSettings: {
    cellText: (value: T, index: number) => string;
    headerText: string;
    widthIc: number;
  }[];
  data: T[];
  defaultValue: T;

  deleteData: (y: number) => void;
  editState: 'edit' | 'insert' | 'new' | 'viewer';
  insertData: (y: number) => void;
  selectedCellX: number;
  selectedCellY: number;
  setEditState: (value: 'edit' | 'insert' | 'new' | 'viewer') => void;

  setSelectedCellX: (x: number) => void;
  setSelectedCellY: (y: number) => void;
  startEdit: (x: number, y: number) => void;
};

export function TableViewer<T>(properties: Properties<T>) {
  const maxX = properties.columnSettings.length;
  const maxY = properties.data.length + 1;

  // refs
  const cellReferences = useRef<(HTMLTableCellElement | null)[][]>([]);
  if (cellReferences.current.length !== properties.data.length + 1) {
    cellReferences.current = Array.from({ length: properties.data.length + 1 })
      .fill(Object.create(null))
      .map(() => Array.from({ length: maxX }, () => Object.create(null)));
  }
  const tableReference = useRef<HTMLDivElement>(null);

  const selectCell = (x: number, y: number) => {
    properties.setSelectedCellX(x);
    properties.setSelectedCellY(y);
  };
  const startEdit = (x: number, y: number) => {
    properties.startEdit(x, y);
  };

  const scrollcell = () => {
    const cell =
      cellReferences?.current[properties.selectedCellY]?.[
        properties.selectedCellX
      ];
    if (cell) {
      const rect = cell.getBoundingClientRect();
      const padding = 40;
      if (rect.top < padding * 2) {
        // 上方向スクロール
        if (!tableReference.current) return;
        console.log(tableReference.current.scrollTop, rect.top - padding);
        tableReference.current.scrollTop -= padding * 2 - rect.top;
      } else if (rect.bottom > tableReference.current!.clientHeight - padding) {
        // 下方向スクロール
        if (!tableReference.current) return;
        tableReference.current.scrollTop +=
          rect.bottom - tableReference.current!.clientHeight + padding;
      }
    }
  };
  // 選択を右移動
  const moveRight = () => {
    if (properties.selectedCellX < maxX - 1) {
      properties.setSelectedCellX(properties.selectedCellX + 1);
    }
  };
  // 選択を右移動
  const moveLeft = () => {
    if (properties.selectedCellX > 0) {
      properties.setSelectedCellX(properties.selectedCellX - 1);
    }
  };
  // 選択を下移動
  const moveDown = () => {
    if (properties.selectedCellY < maxY - 1) {
      properties.setSelectedCellY(properties.selectedCellY + 1);
      scrollcell();
    }
  };
  // 選択を上移動
  const moveUp = () => {
    if (properties.selectedCellY > 0) {
      properties.setSelectedCellY(properties.selectedCellY - 1);
      scrollcell();
    }
  };
  const onKeyDown = (event: KeyboardEvent) => {
    console.log(event.key);
    event.preventDefault();
    switch (event.key) {
      case '+': {
        properties.insertData(properties.selectedCellY);
        return;
      }
      case 'ArrowDown': {
        moveDown();
        return;
      }
      case 'ArrowLeft': {
        moveLeft();
        return;
      }
      case 'ArrowRight': {
        moveRight();
        return;
      }
      case 'ArrowUp': {
        moveUp();
        return;
      }
      case 'Delete': {
        properties.deleteData(properties.selectedCellY);
        cellReferences.current.splice(properties.selectedCellY, 1);
        console.log(properties.data);
        return;
      }
      case 'Enter': {
        startEdit(properties.selectedCellX, properties.selectedCellY);
        return;
      }
    }
  };

  const cellDefaultClass = (width: number, x: number, y: number) => {
    let borderSting = '';
    if (y !== maxY - 1) {
      borderSting += ' border-b-[1px]';
    }
    if (x !== maxX - 1) {
      borderSting += ' border-r-[1px]';
    }
    return `${borderSting} pl-[0.2ic] pr-[0.2ic] border-solid border-gray-600 overflow-hidden w-[${width}ic] ${
      x === properties.selectedCellX && y === properties.selectedCellY
        ? 'bg-gray-200'
        : ''
    }`;
  };
  return (
    <>
      <div className="m-2">
        <div
          className="border-[2px] border-solid border-gray-600 w-fit outline-none max-h-[90dvh] overflow-scroll"
          onKeyDown={onKeyDown}
          ref={tableReference}
          tabIndex={-1}
        >
          <div className="sticky top-0 flex bg-gray-50 z-10 h-[1.5ic]">
            {properties.columnSettings.map((v, index) => (
              <div
                className={cellDefaultClass(v.widthIc, index, -1)}
                key={index}
                style={{ width: `${v.widthIc}ic` }}
              >
                {v.headerText}
              </div>
            ))}
          </div>
          {properties.data.map((d, index) => (
            <>
              <div
                className="flex z-0 h-[1.5ic]"
                key={(d as Keyable).key || undefined}
              >
                {properties.columnSettings.map((v, index_) => (
                  <div
                    className={cellDefaultClass(v.widthIc, index_, index)}
                    key={index_}
                    onClick={() => selectCell(index_, index)}
                    onDoubleClick={() => startEdit(index_, index)}
                    ref={(element: HTMLTableCellElement | null) => {
                      if (!cellReferences.current[index])
                        cellReferences.current[index] = Array.from(
                          { length: maxX },
                          () => Object.create(null),
                        );
                      if (!cellReferences.current[maxY - 1])
                        cellReferences.current[maxY - 1] = Array.from(
                          { length: maxX },
                          () => Object.create(null),
                        );
                      cellReferences.current[index][index_] = element;
                    }}
                    style={{ width: `${v.widthIc}ic` }}
                  >
                    {v.cellText(d, index)}
                  </div>
                ))}
              </div>
            </>
          ))}
          <div className="sticky top-0 flex z-0 h-[1.5ic]">
            {properties.columnSettings.map((v, index) => (
              <div
                className={cellDefaultClass(v.widthIc, index, maxY - 1)}
                key={index}
                onClick={() => selectCell(index, maxY - 1)}
                onDoubleClick={() => startEdit(index, maxY - 1)}
                ref={(element: HTMLTableCellElement | null) => {
                  if (!cellReferences.current[maxY - 1])
                    cellReferences.current[maxY - 1] = Array.from(
                      { length: maxX },
                      () => Object.create(null),
                    );
                  cellReferences.current[maxY - 1][index] = element;
                }}
                style={{ width: `${v.widthIc}ic` }}
              ></div>
            ))}
          </div>
        </div>
      </div>
    </>
  );
}
