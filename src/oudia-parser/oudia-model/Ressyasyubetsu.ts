import { Class } from "../oudia-model";

export class OuDia_Ressyasyubetsu {
	syubetsumei: string;
	ryakusyou: string;
	jikokuhyouMojiColor: string;
	jikokuhyouFontIndex: string;
	jikokuhyouBackColor: string;
	diagramSenColor: string;
	diagramSenStyle: string;
	diagramSenIsBold: string;
	stopMarkDrawType: string;

	constructor(value: Class) {
		this.syubetsumei = value.fields.getValue("Syubetsumei");
		this.ryakusyou = value.fields.getValue("Ryakusyou");
		this.jikokuhyouMojiColor = value.fields.getValue("JikokuhyouMojiColor");
		this.jikokuhyouFontIndex = value.fields.getValue("JikokuhyouFontIndex");
		this.jikokuhyouBackColor = value.fields.getValue("JikokuhyouBackColor");
		this.diagramSenColor = value.fields.getValue("DiagramSenColor")
		this.diagramSenStyle = value.fields.getValue("DiagramSenStyle")
		this.diagramSenIsBold = value.fields.getValue("DiagramSenIsBold")
		this.stopMarkDrawType = value.fields.getValue("StopMarkDrawType")
	}
}