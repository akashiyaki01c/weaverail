import { Model } from 'flexlayout-react';
import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

import { Root } from '../sharpdia-model/Root';

export interface Store {
  model: Model;
  root: Root;
  setModel: (function_: (previous: Model) => Model) => void;
  setRoot: (function_: (previous: Root) => Root) => void;
}

const useGlobalState = create(
  immer<Store>((set) => ({
    model: Model.fromJson({
      borders: [],
      global: {},
      layout: {
        children: [
          {
            children: [
              {
                component: 'tree',
                name: 'TreeView',
                type: 'tab',
              },
            ],
            id: 'left',
            type: 'tabset',
            weight: 50,
          },
          {
            children: [
              {
                component: '',
                name: '新規タブ',
                type: 'tab',
              },
            ],
            id: 'center',
            type: 'tabset',
            weight: 200,
          },
        ],
        type: 'row',
        weight: 100,
      },
    }),
    root: new Root(),
    setModel: (data: (previous: Model) => Model) =>
      set((state) => {
        const newModel = data(state.model as Model);
        state.model = newModel;
      }),
    setRoot: (data: (previous: Root) => Root) =>
      set((state) => {
        state.root = data(state.root);
      }),
  })),
);

export default useGlobalState;
