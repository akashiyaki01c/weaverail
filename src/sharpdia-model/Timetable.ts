import { Train } from "./Train";

export class Timetable {
	id: string;

	constructor(
		public name: string,
		public trains: Train[],
	) {
		this.id = crypto.randomUUID();
	}

	static default() {
		return new Timetable("", []);
	}
}