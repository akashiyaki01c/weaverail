
/** 駅間を表す */
export class Segment {
	id: string;

	constructor(
		/** 開始駅 */
		public startId: string,
		/** 終了駅 */
		public endId: string,
	) {
		this.id = crypto.randomUUID();
	}

	static default() {
		return new Segment("", "");
	}
}

/** 路線を表す */
export class Line {
	id: string;

	constructor(
		public name: string,
		public segments: Segment[],
	) {
		this.id = crypto.randomUUID();
	}
	static default() {
		return new Line("", []);
	}
}