import { IJsonModel, Model } from "flexlayout-react";

export const InitLayout = Model.fromJson({
	global: {},
	borders: [
		{
			location: "bottom",
			children: [],
			type: "border"
		},
		{
			location: "left",
			children: [],
			type: "border"
		},
		{
			
			location: "right",
			children: [],
			type: "border"
		}
	],
	layout: {
		children: [
			{
				id: "main",
				type: "tabset",
				children: [
				]
			}
		],
		type: "row"
	}
} satisfies IJsonModel);
