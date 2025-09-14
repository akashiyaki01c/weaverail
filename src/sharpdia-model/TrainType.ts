export class TrainType {
	id: string;

	constructor(
		public name: string,
	) {
		this.id = crypto.randomUUID();
	}

	static default() {
		return new TrainType("");
	}
}