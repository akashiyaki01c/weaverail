export class TrainType {
	id: string;

	constructor(
		public name: string,
		public color: string,
	) {
		this.id = crypto.randomUUID();
	}

	static default() {
		return new TrainType("", "#000000");
	}
}