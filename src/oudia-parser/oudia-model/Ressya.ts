import { Class } from "../oudia-model";

export class OuDia_Ressya {
	houkou: string;
	syubetsu: string;
	ekijikoku: OuDia_Ekijikoku[];
	ressyabangou: string;
	ressyamei: string;
	gousuu: string;
	bikou: string;
	canceled: string;

	constructor(value: Class) {
		this.houkou = value.fields.getValue("Houkou");
		this.syubetsu = value.fields.getValue("Syubetsu");
		this.ekijikoku = value.fields.getValue("EkiJikoku").split(",").map(v => new OuDia_Ekijikoku(v))!;
		this.ressyabangou = value.fields.getValue("Ressyabangou");
		this.ressyamei = value.fields.getValue("Ressyamei");
		this.gousuu = value.fields.getValue("Gousuu");
		this.bikou = value.fields.getValue("Bikou");
		this.canceled = value.fields.getValue("Canceled");
	}
}

export class OuDia_Ekijikoku {
	ekiatsukai: string = "";
	chaku: string = "";
	hatsu: string = "";
	ressyaTrack: string = "";
	constructor(value: string) {
		if (value === "") {
			return;
		}
		
		const regex = /[;\$\/]/;
		const splited = value.split(regex);
		this.ekiatsukai = splited[0];
		this.chaku = splited[1];
		this.hatsu = splited[2];
		this.ressyaTrack = splited[3];
	}
}