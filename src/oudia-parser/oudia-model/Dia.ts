import { Class } from "../oudia-model";
import { OuDia_Ressya } from "./Ressya";

/** 一つの時刻表を表す */
export class OuDia_Dia {
	diaName: string;
	mainBackColorIndex: string;
	subBackColorIndex: string;
	backPatternIndex: string;
	kudari: OuDia_Kudari;
	nobori: OuDia_Kudari;

	constructor(value: Class) {
		this.diaName = value.fields.getValue("DiaName");
		this.mainBackColorIndex = value.fields.getValue("MainBackColorIndex");
		this.subBackColorIndex = value.fields.getValue("SubBackColorIndex");
		this.backPatternIndex = value.fields.getValue("BackPatternIndex");
		this.kudari = new OuDia_Kudari(value.classes.find(v => v.name === "Kudari")!);
		this.nobori = new OuDia_Kudari(value.classes.find(v => v.name === "Nobori")!);
	}
}

export class OuDia_Kudari {
	ressya: OuDia_Ressya[]

	constructor(value: Class) {
		this.ressya = value.classes.filter(v => v.name === "Ressya").map(v => new OuDia_Ressya(v));
	}
}