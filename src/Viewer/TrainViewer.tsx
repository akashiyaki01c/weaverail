import { useParams } from "react-router-dom";
import useGlobalState from "../globalState/useGlobalState";
import { TimetableService } from "../globalState/TimetableService";
import { TrainService } from "../globalState/TrainService";

export function TrainViewer() {
  const globalState = useGlobalState();
  const params = useParams();
  const timetableId = params.timetableId;
  if (!timetableId) {
    throw new Error("timetable id null");
  }
  const timetable = TimetableService.findById(globalState.root, timetableId);
  const timetableIndex = TimetableService.findIndexById(
    globalState.root,
    timetableId
  );
  if (!timetable) {
    throw new Error("timetable is null");
  }
	const trainId = params.trainId;
	if (!trainId) {
    throw new Error("train id null");
  }
	const train = TrainService.findById(globalState.root, timetableIndex, trainId);
	if (!trainId) {
		throw new Error("train is null");
	}
	
  return <></>;
}
