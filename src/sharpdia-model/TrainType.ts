export class TrainType {
	constructor(
		public name: string,
	) { }

	static default() {
		return new TrainType("");
	}
}