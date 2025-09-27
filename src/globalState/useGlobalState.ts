import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import { Root } from "../sharpdia-model/Root";
import { Model } from "flexlayout-react";

export interface Store {
	root: Root,
	setRoot: (fn: (prev: Root) => Root) => void;
	model: Model,
	setModel: (fn: (prev: Model) => Model) => void;
}

let useGlobalState = create(immer<Store>((set, _) => ({
	root: new Root(),
	setRoot: (data: (prev: Root) => Root) => set((state) => {
		state.root = data(state.root);
	}),
	model: Model.fromJson({
		global: {},
		borders: [],
		layout: {
			type: "row",
			weight: 100,
			children: [
				{
					id: "left",
					type: "tabset",
					weight: 50,
					children: [
						{
							type: "tab",
							name: "TreeView",
							component: "tree",
						},
					],
				},
				{
					id: "center",
					type: "tabset",
					weight: 200,
					children: [{
						type: "tab",
						name: "新規タブ",
						component: "",
					},],
				},
			],
		},
	}),
	setModel: (data: (prev: Model) => Model) => set((state) => {
		const newModel = data(state.model as Model);
		state.model = newModel;
	})
})));

export default useGlobalState;