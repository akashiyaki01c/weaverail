import { OuDia_Eki, OuDia_Ekijikokukeisiki, OuDia_Ekikibo, OuDia_EkiTrack2 } from "../oudia-parser/oudia-model/Eki";

/** 駅を表す */
export class Station {
	id: string;
	constructor(
		public name: string,
		public displayType: OuDia_Ekijikokukeisiki,
		public kibo: OuDia_Ekikibo,
		public tracks: Track[],
	) {
		this.id = crypto.randomUUID();
	}

	static fromOuDia(value: OuDia_Eki) {
		const obj = new Station(value.ekimei, value.ekijikokukeisiki, value.ekikibo, value.track.value.map(v => Track.fromOuDia(v)));
		return obj;
	}
	static default() {
		return new Station("", "Hatsu", "Ippan", []);
	}
}

/** 番線を表す */
export class Track {
	constructor(
		public name: string,
	) { }

	static fromOuDia(value: OuDia_EkiTrack2) {
		const obj = new Track(value.trackName);
		return obj;
	}
}