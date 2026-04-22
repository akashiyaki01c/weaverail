/**
 * Weaverailの拡張機能に要求する機能をまとめたインタフェース
 */
export interface WeaverailExtension {
	/** 拡張機能の一意な識別子 */
	readonly id: string;
	/** 拡張機能の各種情報 */
	readonly metadata: {
		/** 拡張機能名 */
		readonly name: string;
		/** 拡張機能の概要 */
		readonly description?: string;
	};
	/** 初期化関数 */
	init(): void;
}