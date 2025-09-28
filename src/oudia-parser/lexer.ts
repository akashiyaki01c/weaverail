import { Class, Field, Fields, LexerRootClass } from './oudia-model';

class TextReader {
  #value: string[] = [];

  constructor(text: string) {
    this.#value = text.split('\r\n').filter((v) => v !== '');
  }

  /** 現在の列を返す */
  public read() {
    return this.#value.shift();
  }
  /** 次の列を返す */
  public seek() {
    return this.#value[0];
  }
  /** シーク可能か */
  public seekable() {
    return this.#value.length > 0;
  }
}

export function ParseOud2(text: string) {
  const reader = new TextReader(text);

  const fields: Field[] = [];
  const classes: Class[] = [];
  while (true) {
    if (!reader.seekable()) {
      break;
    }
    const seekString = reader.seek()!;
    if (isClassHeader(seekString)) {
      classes.push(ParseObject(reader));
    }
    if (isField(seekString)) {
      reader.read();
      fields.push(getField(seekString));
    }
    if (isClassEnd(seekString)) {
      reader.read();
      throw new SyntaxError('終端に達しました');
    }
  }

  return new LexerRootClass(fields, classes);
}

/** クラスヘッダの名前を取得 */
function getClassName(text: string) {
  return text.match(/^(?<text>.+)[.]$/)?.groups?.text || '';
}

function getField(text: string) {
  const result = text.match(/^(?<name>.*)=(?<value>.*)$/);
  const name = result?.groups?.name || '';
  const value = result?.groups?.value || '';
  return new Field(name, value);
}
/** 文字列がクラスの終端であるか */
function isClassEnd(text: string) {
  return text === '.';
}
/** 文字列がクラスヘッダであるか */
function isClassHeader(text: string) {
  return /^(?<text>.+)[.]$/.test(text);
}
/** 文字列がフィールドであるか */
function isField(text: string) {
  return /^(?<name>.*)=(?<value>.*)$/.test(text);
}
function ParseObject(reader: TextReader) {
  if (!reader.seekable()) {
    throw new SyntaxError('終端に達しました');
  }
  if (!isClassHeader(reader.seek())) {
    throw new SyntaxError('終端に達しました');
  }
  const classHeader = getClassName(reader.read()!);
  const fields: Field[] = [];
  const classes: Class[] = [];
  while (true) {
    if (!reader.seekable()) {
      break;
    }
    const seekString = reader.seek()!;
    if (isClassHeader(seekString)) {
      classes.push(ParseObject(reader));
    }
    if (isField(seekString)) {
      fields.push(getField(seekString));
      reader.read()!;
    }
    if (isClassEnd(seekString)) {
      reader.read()!;
      break;
    }
  }

  return new Class(classHeader, new Fields(fields), classes);
}
