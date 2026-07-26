import { useEffect, useState } from "react";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { TimetableId } from "@weaverail/types";

export function TestViwer() {
  const [time, setTime] = useState(0);
  const { manager } = useExtensionManager();
  const [timetableId, setTimetableId] = useState<TimetableId>("");

  useEffect(() => {
    (async () => {
      const root = await manager.api.data.getRoot();
      console.log(root);
      const timetableId = Object.keys(root.timetables)[0];
      setTimetableId(timetableId);
    })();
  }, []);

  return (
    <>
      <input
        type="number"
        value={time}
        onChange={(e) => setTime(Number.parseInt(e.target.value))}
      />
      <button
        onClick={() => {
          manager.api.data.debugInsertTrain(timetableId, time);
        }}
      >
        さぶみっと
      </button>
    </>
  );
}
