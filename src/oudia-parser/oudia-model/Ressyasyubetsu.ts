import { Class } from '../oudia-model';

export class OuDia_Ressyasyubetsu {
  diagramSenColor: string;
  diagramSenIsBold: string;
  diagramSenStyle: string;
  jikokuhyouBackColor: string;
  jikokuhyouFontIndex: string;
  jikokuhyouMojiColor: string;
  ryakusyou: string;
  stopMarkDrawType: string;
  syubetsumei: string;

  constructor(value: Class) {
    this.syubetsumei = value.fields.getValue('Syubetsumei');
    this.ryakusyou = value.fields.getValue('Ryakusyou');
    this.jikokuhyouMojiColor = value.fields.getValue('JikokuhyouMojiColor');
    this.jikokuhyouFontIndex = value.fields.getValue('JikokuhyouFontIndex');
    this.jikokuhyouBackColor = value.fields.getValue('JikokuhyouBackColor');
    this.diagramSenColor = value.fields.getValue('DiagramSenColor');
    this.diagramSenStyle = value.fields.getValue('DiagramSenStyle');
    this.diagramSenIsBold = value.fields.getValue('DiagramSenIsBold');
    this.stopMarkDrawType = value.fields.getValue('StopMarkDrawType');
  }
}
