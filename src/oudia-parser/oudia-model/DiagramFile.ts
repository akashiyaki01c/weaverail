import { LexerRootClass } from "../oudia-model";
import { OuDia_Rosen } from "./Rosen";

export class OuDia_DiagramFile {
	rosen: OuDia_Rosen;

	constructor(value: LexerRootClass) {
		this.rosen = new OuDia_Rosen(value.classes.find(v => v.name === "Rosen")!);
	}
}