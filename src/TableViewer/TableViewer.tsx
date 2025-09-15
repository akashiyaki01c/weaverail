import { KeyboardEvent, useRef } from "react";

type Props<T> = {
  columnSettings: {
    headerText: string;
    widthIc: number;
    cellText: (value: T, index: number) => string;
  }[];
  data: T[];
  defaultValue: T;

  selectedCellX: number;
  selectedCellY: number;
  setSelectedCellX: (x: number) => void;
  setSelectedCellY: (y: number) => void;
  editState: "viewer" | "new" | "insert" | "edit";
  setEditState: (value: "viewer" | "new" | "insert" | "edit") => void;

  startEdit: (x: number, y: number) => void;
  deleteData: (y: number) => void;
  insertData: (y: number) => void;
};

export function TableViewer<T>(props: Props<T>) {
  const maxX = props.columnSettings.length;
  const maxY = props.data.length + 1;

  // refs
  const cellRefs = useRef<(HTMLTableCellElement | null)[][]>([]);
  if (cellRefs.current.length !== props.data.length + 1) {
    cellRefs.current = Array(props.data.length + 1)
      .fill(null)
      .map(() => Array(maxX).fill(null));
  }
  const tableRef = useRef<HTMLDivElement>(null);

  const selectCell = (x: number, y: number) => {
    props.setSelectedCellX(x);
    props.setSelectedCellY(y);
  };
  const startEdit = (x: number, y: number) => {
    props.startEdit(x, y);
  };

  const scrollcell = () => {
    const cell = cellRefs?.current[props.selectedCellY]?.[props.selectedCellX];
    if (cell) {
      const rect = cell.getBoundingClientRect();
      const padding = 40;
      if (rect.top < padding * 2) {
        // 上方向スクロール
        if (!tableRef.current) return;
        console.log(tableRef.current.scrollTop, rect.top - padding);
        tableRef.current.scrollTop -= padding * 2 - rect.top;
      } else if (rect.bottom > tableRef.current!.clientHeight - padding) {
        // 下方向スクロール
        if (!tableRef.current) return;
        tableRef.current.scrollTop +=
          rect.bottom - tableRef.current!.clientHeight + padding;
      }
    }
  };
  // 選択を右移動
  const moveRight = () => {
    if (props.selectedCellX < maxX - 1) {
      props.setSelectedCellX(props.selectedCellX + 1);
    }
  };
  // 選択を右移動
  const moveLeft = () => {
    if (props.selectedCellX > 0) {
      props.setSelectedCellX(props.selectedCellX - 1);
    }
  };
  // 選択を下移動
  const moveDown = () => {
    if (props.selectedCellY < maxY - 1) {
      props.setSelectedCellY(props.selectedCellY + 1);
      scrollcell();
    }
  };
  // 選択を上移動
  const moveUp = () => {
    if (props.selectedCellY > 0) {
      props.setSelectedCellY(props.selectedCellY - 1);
      scrollcell();
    }
  };
  const onKeyDown = (e: KeyboardEvent) => {
    console.log(e.key);
    e.preventDefault();
    switch (e.key) {
      case "ArrowRight":
        moveRight();
        return;
      case "ArrowLeft":
        moveLeft();
        return;
      case "ArrowDown":
        moveDown();
        return;
      case "ArrowUp":
        moveUp();
        return;
      case "Enter":
        startEdit(props.selectedCellX, props.selectedCellY);
        return;
      case "Delete":
        props.deleteData(props.selectedCellY);
        cellRefs.current.splice(props.selectedCellY, 1);
        console.log(props.data);
        return;
      case "+":
        props.insertData(props.selectedCellY);
        return;
    }
  };

  const cellDefaultClass = (width: number, x: number, y: number) => {
    let borderSting = "";
    if (y !== maxY - 1) {
      borderSting += " border-b-[1px]";
    }
    if (x !== maxX - 1) {
      borderSting += " border-r-[1px]";
    }
    return `${borderSting} pl-[0.2ic] pr-[0.2ic] border-solid border-gray-600 overflow-hidden w-[${width}ic] ${
      x === props.selectedCellX && y === props.selectedCellY
        ? "bg-gray-200"
        : ""
    }`;
  };
  return (
    <>
      <div className="m-2">
        <div
          className="border-[2px] border-solid border-gray-600 w-fit outline-none max-h-[90dvh] overflow-scroll"
          onKeyDown={onKeyDown}
          tabIndex={-1}
          ref={tableRef}
        >
          <div className="sticky top-0 flex bg-gray-50 z-10 h-[1.5ic]">
            {props.columnSettings.map((v, i) => (
              <div
                key={i}
                className={cellDefaultClass(v.widthIc, i, -1)}
                style={{ width: `${v.widthIc}ic` }}
              >
                {v.headerText}
              </div>
            ))}
          </div>
          {props.data.map((d, i) => (
            <>
              <div className="flex z-0 h-[1.5ic]" key={(d as any).key || undefined}>
                {props.columnSettings.map((v, j) => (
                  <div
                    key={j}
                    className={cellDefaultClass(v.widthIc, j, i)}
                    onClick={() => selectCell(j, i)}
                    onDoubleClick={() => startEdit(j, i)}
                    ref={(el: HTMLTableCellElement | null) => {
                      if (!cellRefs.current[i])
                        cellRefs.current[i] = Array(maxX).fill(null);
                      if (!cellRefs.current[maxY - 1])
                        cellRefs.current[maxY - 1] = [null, null, null, null];
                      cellRefs.current[i][j] = el;
                    }}
                    style={{ width: `${v.widthIc}ic` }}
                  >
                    {v.cellText(d, i)}
                  </div>
                ))}
              </div>
            </>
          ))}
          <div className="sticky top-0 flex z-0 h-[1.5ic]">
            {props.columnSettings.map((v, j) => (
              <div
                className={cellDefaultClass(v.widthIc, j, maxY - 1)}
                onClick={() => selectCell(j, maxY - 1)}
                onDoubleClick={() => startEdit(j, maxY - 1)}
                ref={(el: HTMLTableCellElement | null) => {
                  if (!cellRefs.current[maxY - 1])
                    cellRefs.current[maxY - 1] = [null, null, null, null];
                  cellRefs.current[maxY - 1][j] = el;
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
