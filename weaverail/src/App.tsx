import { useState } from "react";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [newStationName, setNewStationName] = useState("");
  const [root, setRoot] = useState({});

  const [undoable, setUndoable] = useState(false);
  const [redoable, setRedoable] = useState(false);

  listen<any>("station_changed", (event) => {
    setRoot(event.payload);
  });

  return (
    <main className="container">
      <div>
        <button
          disabled={!undoable}
          onClick={async () => {
            await invoke("undo");
            const undoable = await invoke("undoable");
            setUndoable(undoable as boolean);
            const redoable = await invoke("redoable");
            setRedoable(redoable as boolean);
            console.log(undoable, redoable)
          }}
        >
          元に戻す
        </button>
        <button
          disabled={!redoable}
          onClick={async () => {
            await invoke("redo");
            setUndoable(await invoke("undoable"));
            setRedoable(await invoke("redoable"));
          }}
        >
          やり直す
        </button>
      </div>
      <div>{JSON.stringify(root, null, "\t")}</div>
      <div>
        <label>
          駅名
          <input
            type="text"
            value={newStationName}
            onChange={(v) => setNewStationName(v.target.value)}
          />
        </label>
        <button
          onClick={async () => {
            const station = {
              id: crypto.randomUUID(),
              name: newStationName,
              properties: {},
            };
            await invoke("add_station", { station });

            const undoable = await invoke("undoable");
            setUndoable(undoable as boolean);
            const redoable = await invoke("redoable");
            setRedoable(redoable as boolean);
            console.log(undoable, redoable)
          }}
        >
          追加
        </button>
      </div>
    </main>
  );
}

export default App;
