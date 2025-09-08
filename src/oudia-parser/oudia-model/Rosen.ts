import { Class } from "../oudia-model";
import { OuDia_Dia } from "./Dia";
import { OuDia_Eki } from "./Eki";
import { OuDia_Ressyasyubetsu } from "./Ressyasyubetsu";

export class OuDia_Rosen {
	rosenmei: string;
	kudariDiaAlias: string;
	noboriDiaAlias: string;
	kitenJikoku: string;
	diagramDgrYZahyouKyoriDefault: string;
	enableOperation: string;
	operationCrossKitenJikoku: string;
	kijunDiaIndex: string;
	comment: string;

	eki: OuDia_Eki[];
	ressyasyubetsu: OuDia_Ressyasyubetsu[];
	dia: OuDia_Dia[];

	constructor(value: Class) {
		this.rosenmei = value.fields.getValue("Rosenmei");
		this.kudariDiaAlias = value.fields.getValue("KudariDiaAlias");
		this.noboriDiaAlias = value.fields.getValue("NoboriDiaAlias");
		this.kitenJikoku = value.fields.getValue("KitenJikoku")
		this.diagramDgrYZahyouKyoriDefault = value.fields.getValue("DiagramDgrYZahyouKyoriDefault")
		this.enableOperation = value.fields.getValue("enableOperation")
		this.operationCrossKitenJikoku = value.fields.getValue("OperationCrossKitenJikoku")
		this.kijunDiaIndex = value.fields.getValue("KijunDiaIndex")
		this.comment = value.fields.getValue("Comment")
		this.eki = value.classes.filter(v => v.name === "Eki").map(v => new OuDia_Eki(v));
		this.ressyasyubetsu = value.classes.filter(v => v.name === "Ressyasyubetsu").map(v => new OuDia_Ressyasyubetsu(v));
		this.dia = value.classes.filter(v => v.name === "Dia").map(v => new OuDia_Dia(v));
	}
}