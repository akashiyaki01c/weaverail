import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';

import { Store } from '../useGlobalState';
import { Station } from '../sharpdia-model/Station';
import { Line } from '../sharpdia-model/Line';
import { TrainType } from '../sharpdia-model/TrainType';


export async function OpenFile(store: Store) {
	const filePath = await openDialog({
		multiple: false,
		directory: false,
		filters: [
			{
				name: 'diagram project file',
				extensions: ['dproj']
			}
		]
	});
	if (!filePath) {
		return;
	}

	const text = await readTextFile(filePath);
	const parsed = JSON.parse(text);

	console.log(parsed);
	store.setStations(() => parsed.stations.map((v: any) => ({...(Station.default()), ...v})) || []);
	store.setLines(() => parsed.lines.map((v: any) => ({...(Line.default()), ...v})) || []);
	store.setTrainTypes(() => parsed.trainTypes.map((v: any) => ({...(TrainType.default()), ...v})) || []);
}

export async function SaveFile(store: Store) {
	const filePath = await saveDialog({
		filters: [
			{
				name: 'diagram project file',
				extensions: ['dproj']
			}
		]
	});
	if (!filePath) {
		return;
	}
	const contents = JSON.stringify({
		stations: store.stations,
		lines: store.lines,
		trainTypes: store.trainTypes,
	});
	await writeTextFile(filePath, contents, {});
}