import { useEffect, useState } from "react";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { Station } from "@weaverail/types";
import { WeaverailApi, WeaverailApiObject } from "../../api";

function App() {
  const api: WeaverailApi = new WeaverailApiObject();
  const [newStationName, setNewStationName] = useState("");
  const [root, setRoot] = useState({});

  const [undoable, setUndoable] = useState(false);
  const [redoable, setRedoable] = useState(false);

  useEffect(() => {
    (async () => {
      setRoot(await api.data.getRoot());
    })();
    listen<any>("station::added", async (event) => {
      setRoot(await api.data.getRoot());
    });
    listen<any>("station::deleted", async (event) => {
      setRoot(await api.data.getRoot());
    });
  }, []);

  return (
    <main className="container">
      <div>
        <button
          disabled={!undoable}
          onClick={async () => {
            await api.ops.undo();
            setUndoable(await api.ops.undoable());
            setRedoable(await api.ops.redoable());
          }}
        >
          元に戻す
        </button>
        <button
          disabled={!redoable}
          onClick={async () => {
            await api.ops.redo();
            setUndoable(await api.ops.undoable());
            setRedoable(await api.ops.redoable());
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
            const newId = await api.ops.station.new_station_id();
            const station = {
              id: newId,
              name: newStationName,
              properties: {},
              tracks: {},
            } satisfies Station;
            await api.ops.station.add_station(station);

            const undoable = await api.ops.undoable();
            setUndoable(undoable as boolean);
            const redoable = await api.ops.redoable();
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
