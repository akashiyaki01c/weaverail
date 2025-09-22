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
		public id: string,
		public isReversed: boolean,
	) {}

	static default() {
		return new DiagramLineSegment("", false);
	}
}