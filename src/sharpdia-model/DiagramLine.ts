export class DiagramLine {
	id: string;

	constructor(
		public name: string,
		public segments: DiagramLineSegment[]
	) {
		this.id = crypto.randomUUID();
	}
	static default() {
		return new DiagramLine("", []);
	}
}

export class DiagramLineSegment {
	constructor(
		/** ID */
		public id: string,
		/** 逆転しているか */
		public isReversed: boolean,
		/** 表示上の運行時間 */
		public displaySeconds: number,
	) {}

	static default() {
		return new DiagramLineSegment("", false, 0);
	}
}