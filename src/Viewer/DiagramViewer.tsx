import { useState } from "react";
import useGlobalState from "../globalState/useGlobalState";

export function DiagramViewer() {
	const globalState = useGlobalState();
	const [option, setOption] = useState(new DiagramViewerOption(1, 1, 0, 0));

	return <>
		<div>
			<svg></svg>
		</div>
	</>
}

export class DiagramViewerOption {
	constructor(
		public xScale: number,
		public yScale: number,
		public xOffset: number,
		public yOffset: number,
	) {}
}