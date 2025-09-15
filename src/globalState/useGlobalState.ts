import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import { Root } from "../sharpdia-model/Root";

export interface Store {
	root: Root,
	setRoot: (fn: (prev: Root) => Root) => void;
}

let useGlobalState = create(immer<Store>((set, _) => ({
	root: new Root(),
	setRoot: (data: (prev: Root) => Root) => set((state) => {
		state.root = data(state.root);
	}),
})));

export default useGlobalState;