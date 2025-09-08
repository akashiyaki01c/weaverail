import { Class } from "../oudia-model";
import { EMPTY_CLASS } from "../oudia-model";

/** `CentDedEkiTrack2`
 * 駅の番線
 */
export class OuDia_EkiTrack2 {
	/** 番線名 */
	trackName: string;
	/** 番線略称 */
	trackRyakusyou: string;
	/** 上り番線略称 */
	trackNoboriRyakusyou: string;
	constructor(value: Class) {
		this.trackName = value.fields.getValue("TrackName");
		this.trackRyakusyou = value.fields.getValue("TrackRyakusyou");
		this.trackNoboriRyakusyou = value.fields.getValue("TrackNoboriRyakusyou");
	}
}

/** `CentDedEkiTrack2Cont `
 * 駅の番線コンテナ
 */
export class OuDia_EkiTrack2Cont {
	value: OuDia_EkiTrack2[];

	constructor(value: Class) {
		this.value = value.classes
			.filter(v => v.name === "EkiTrack2")
			.map(v => new OuDia_EkiTrack2(v));
	}
}

/** `EEkijikokukeisiki`
 * 駅時刻形式
 */
export type OuDia_Ekijikokukeisiki = "Hatsu" | "Hatsuchaku" | "KudariChaku" | "NoboriChaku" | "KudariHatsuchaku" | "NoboriHatsuchaku";
function parseEkijikokukeisiki(value: string): OuDia_Ekijikokukeisiki {
	switch (value) {
		case "Jikokukeisiki_Hatsu":
			return "Hatsu";
		case "Jikokukeisiki_Hatsuchaku":
			return "Hatsuchaku";
		case "Jikokukeisiki_KudariChaku":
			return "KudariChaku";
		case "Jikokukeisiki_NoboriChaku":
			return "NoboriChaku";
		case "Jikokukeisiki_KudariHatsuchaku":
			return "KudariHatsuchaku";
		case "Jikokukeisiki_NoboriHatsuchaku":
			return "NoboriHatsuchaku";
		default:
			throw new SyntaxError();
	}
}

/**
 * `EEkikibo`
 * 駅規模
 */
export type OuDia_Ekikibo = "Ippan" | "Syuyou";
function parseEkikibo(value: string): OuDia_Ekikibo {
	switch (value) {
		case "Ekikibo_Ippan":
			return "Ippan";
		case "Ekikibo_Syuyou":
			return "Syuyou";
		default:
			throw new SyntaxError();
	}
}

/**
 * `CentDedEki`
 * 駅
 */
export class OuDia_Eki {
	ekimei: string;
	ekimeiJikokuRyaku: string;
	ekimeiDiaRyaku: string;
	ekijikokukeisiki: OuDia_Ekijikokukeisiki;
	ekikibo: OuDia_Ekikibo;
	downMain: string;
	upMain: string;
	jikokuhyouTrackOmit: string;
	jikokuhyouJikokuDisplayKudari: string;
	jikokuhyouJikokuDisplayNobori: string;
	jikokuhyouSyubetsuChangeDisplayKudari: string;
	jikokuhyouSyubetsuChangeDisplayNobori: string;
	diagramColorNextEki: string;
	jikokuhyouOuterDisplayKudari: string;
	jikokuhyouOuterDisplayNobori: string;

	track: OuDia_EkiTrack2Cont

	constructor(value: Class) {
		this.ekimei = value.fields.getValue("Ekimei");
		this.ekimeiJikokuRyaku = value.fields.getValue("EkimeiJikokuRyaku");
		this.ekimeiDiaRyaku = value.fields.getValue("EkimeiDiaRyaku");
		this.ekijikokukeisiki = parseEkijikokukeisiki(value.fields.getValue("Ekijikokukeisiki"));
		this.ekikibo = parseEkikibo(value.fields.getValue("Ekikibo"));
		this.downMain = value.fields.getValue("DownMain");
		this.upMain = value.fields.getValue("UpMain");
		this.jikokuhyouTrackOmit = value.fields.getValue("JikokuhyouTrackOmit");
		this.jikokuhyouJikokuDisplayKudari = value.fields.getValue("JikokuhyouJikokuDisplayKudari");
		this.jikokuhyouJikokuDisplayNobori = value.fields.getValue("JikokuhyouJikokuDisplayNobori");
		this.jikokuhyouSyubetsuChangeDisplayKudari = value.fields.getValue("JikokuhyouSyubetsuChangeDisplayKudari");
		this.jikokuhyouSyubetsuChangeDisplayNobori = value.fields.getValue("JikokuhyouSyubetsuChangeDisplayNobori");
		this.diagramColorNextEki = value.fields.getValue("DiagramColorNextEki");
		this.jikokuhyouOuterDisplayKudari = value.fields.getValue("JikokuhyouOuterDisplayKudari");
		this.jikokuhyouOuterDisplayNobori = value.fields.getValue("JikokuhyouOuterDisplayNobori");
		this.track = new OuDia_EkiTrack2Cont(value.classes.find(v => v.name === "EkiTrack2Cont") || EMPTY_CLASS);
	}
}