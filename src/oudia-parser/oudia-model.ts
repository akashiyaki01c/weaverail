export class Field {
	constructor(
		public name: string,
		public value: string,
	) { }
}
export class Fields {
	constructor(
		public data: Field[]
	) {}

	get(key: string) {
		return this.data.find(v => v.name === key)!
	}
	getValue(key: string) {
		return this.data.find(v => v.name === key)?.value || ""
	}
	getValueDefault(key: string, defaultValue: string) {
		return this.data.find(v => v.name === key)?.value || defaultValue
	}
}
export class Class {
	constructor(
		public name: string,
		public fields: Fields,
		public classes: Class[],
	) { }
}
export class LexerRootClass {
	constructor(
		public fields: Field[],
		public classes: Class[],
	) {}
}

export const EMPTY_CLASS = new Class("", new Fields([]), []);