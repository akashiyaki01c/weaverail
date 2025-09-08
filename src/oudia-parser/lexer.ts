import { Class, Field, Fields, LexerRootClass } from "./oudia-model";

class TextReader {
	#value: string[] = [];

	constructor(text: string) {
		this.#value = text.split("\r\n").filter(v => v !== "");
	}

	/** シーク可能か */
	public seekable() {
		return this.#value.length > 0;
	}
	/** 次の列を返す */
	public seek() {
		return this.#value[0]
	}
	/** 現在の列を返す */
	public read() {
		return this.#value.shift()
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
		const seekStr = reader.seek()!;
		if (isClassHeader(seekStr)) {
			classes.push(ParseObject(reader));
		}
		if (isField(seekStr)) {
			reader.read();
			fields.push(getField(seekStr))
		}
		if (isClassEnd(seekStr)) {
			reader.read();
			throw new SyntaxError();
		}
	}

	return new LexerRootClass(fields, classes);
}

function ParseObject(reader: TextReader) {
	if (!reader.seekable()) {
		throw new SyntaxError();
	}
	if (!isClassHeader(reader.seek())) {
		throw new SyntaxError();
	}
	const classHeader = getClassName(reader.read()!);
	const fields: Field[] = [];
	const classes: Class[] = [];
	while (true) {
		if (!reader.seekable()) {
			break;
		}
		const seekStr = reader.seek()!;
		if (isClassHeader(seekStr)) {
			classes.push(ParseObject(reader));
		}
		if (isField(seekStr)) {
			fields.push(getField(seekStr))
			reader.read()!;
		}
		if (isClassEnd(seekStr)) {
			reader.read()!;
			break;
		}
	}

	return new Class(classHeader, new Fields(fields), classes);
}

/** 文字列がクラスヘッダであるか */
function isClassHeader(text: string) {
	return /^(?<text>.+)[.]$/.test(text);
}
/** クラスヘッダの名前を取得 */
function getClassName(text: string) {
	return text.match(/^(?<text>.+)[.]$/)?.groups?.text!;
}
/** 文字列がクラスの終端であるか */
function isClassEnd(text: string) {
	return text === "."
}
/** 文字列がフィールドであるか */
function isField(text: string) {
	return /^(?<name>.*)=(?<value>.*)$/.test(text);
}
function getField(text: string) {
	const result = text.match(/^(?<name>.*)=(?<value>.*)$/);
	const name = result?.groups?.name!;
	const value = result?.groups?.value!;
	return new Field(name, value);
}