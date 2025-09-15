// 列車を表す
export class Train {
	id: string;

	constructor(
		public number: string,
		public trainTypeId: string,
		public segments: TrainSegment[]
	) {
		this.id = crypto.randomUUID();
	}

	static default() {
		return new Train("", "", []);
	}
}

// 列車を表す
export class TrainSegment {
	id: string;

	constructor(
		public segments: { id: string, isReversed: boolean }[],
		public arrivalTime: number,
		public departureTime: number,
	) {
		this.id = crypto.randomUUID();
	}

	static default() {
		return new TrainSegment([], 0, 0);
	}
}