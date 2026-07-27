import { useEffect, useState } from "react";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { TimetableId } from "@weaverail/types";

export function TestViwer() {
  const [hour, setHour] = useState(0);
  const [minute, setMinute] = useState(0);
  const [second, setSecond] = useState(0);
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
      <div style={{display: "flex"}}>
        <input
		  style={{width: "4em"}}
          type="number"
          value={hour}
          onChange={(e) => setHour(Number.parseInt(e.target.value))}
        />
        :
        <input
		  style={{width: "4em"}}
          type="number"
          value={minute}
          onChange={(e) => setMinute(Number.parseInt(e.target.value))}
        />
        :
        <input
		  style={{width: "4em"}}
          type="number"
          value={second}
          onChange={(e) => setSecond(Number.parseInt(e.target.value))}
        />
      </div>
      <button
        onClick={async () => {
          console.time();
          await manager.api.data.debugInsertTrain(timetableId, hour * 3600 + minute * 60 + second);
          console.timeEnd();
          setHour(0);
          setMinute(0);
          setSecond(0);
        }}
      >
        さぶみっと
      </button>
    </>
  );
}
