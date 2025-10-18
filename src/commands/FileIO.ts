import {
  open as openDialog,
  save as saveDialog,
} from '@tauri-apps/plugin-dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';

import { Store } from '../globalState/useGlobalState';
import { Root } from '../sharpdia-model/Root';

export async function OpenFile(store: Store) {
  const filePath = await openDialog({
    directory: false,
    filters: [
      {
        extensions: ['dproj'],
        name: 'diagram project file',
      },
    ],
    multiple: false,
  });
  if (!filePath) {
    return;
  }

  const text = await readTextFile(filePath);
  const parsed = JSON.parse(text);

  store.setRoot(() => ({ ...new Root(), ...parsed }));
}

export async function SaveFile(store: Store) {
  const filePath = await saveDialog({
    filters: [
      {
        extensions: ['dproj'],
        name: 'diagram project file',
      },
    ],
  });
  if (!filePath) {
    return;
  }
  const contents = JSON.stringify(store.root, undefined, '\t');
  await writeTextFile(filePath, contents, {});
}
