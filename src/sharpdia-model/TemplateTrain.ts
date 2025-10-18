/** テンプレート用列車を表す */
export class TemplateTrain {
  id: string;

  constructor(
    public name: string,
    public trainTypeId: string,
    public segments: TemplateTrainSegment[],
  ) {
    this.id = crypto.randomUUID();
  }

  static default() {
    return new TemplateTrain('', '', []);
  }
}

/** テンプレート用列車の時刻設定を表す */
export class TemplateTrainSegment {
  id: string;

  constructor(
    public segments: { id: string; isReversed: boolean }[],
    public time: number,
  ) {
    this.id = crypto.randomUUID();
  }

  static default() {
    return new TemplateTrainSegment([], 0);
  }
}
