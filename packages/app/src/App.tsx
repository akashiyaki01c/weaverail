import { useEffect, useState } from "react";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Station, StationId } from "@weaverail/types";
import { WeaverailApi } from "../../api";

function App() {
  const api = new WeaverailApi();
  const [newStationName, setNewStationName] = useState("");
  const [root, setRoot] = useState({});

  const [undoable, setUndoable] = useState(false);
  const [redoable, setRedoable] = useState(false);

  useEffect(() => {
    listen<any>("station::added", async (event) => {
      setRoot(await api.getRoot());
    });
    listen<any>("station::deleted", async (event) => {
      setRoot(await api.getRoot());
    });
  }, []);

  return (
    <main className="container">
      <div>
        <button
          disabled={!undoable}
          onClick={async () => {
            await api.undo();
            setUndoable(await api.undoable());
            setRedoable(await api.redoable());
          }}
        >
          元に戻す
        </button>
        <button
          disabled={!redoable}
          onClick={async () => {
            await api.redo();
            setUndoable(await api.undoable());
            setRedoable(await api.redoable());
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
            const newId = await api.new_station_id();
            const station = {
              id: newId,
              name: newStationName,
              properties: {},
              tracks: {},
            } satisfies Station;
            await api.add_station(station);

            const undoable = await api.undoable();
            setUndoable(undoable as boolean);
            const redoable = await api.redoable();
            setRedoable(redoable as boolean);
            console.log(undoable, redoable);
          }}
        >
          追加
        </button>
      </div>
    </main>
  );
}

export default App;
