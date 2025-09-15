import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';

import { Store } from '../globalState/useGlobalState';
import { Root } from '../sharpdia-model/Root';

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

	store.setRoot(_ => ({ ...new Root(), ...parsed }));
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
	const contents = JSON.stringify(store.root);
	await writeTextFile(filePath, contents, {});
}