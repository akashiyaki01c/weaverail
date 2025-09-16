export class DiagramLine {
	id: string;

	constructor(
		public name: string,
		public segments: { id: string, isReversed: boolean }[]
	) {
		this.id = crypto.randomUUID();
	}
	static default() {
		return new DiagramLine("", []);
	}
}