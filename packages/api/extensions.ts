import { WeaverailApi } from ".";

/**
 * Weaverailの拡張機能が実装するインタフェース
 */
export interface WeaverailExtension {
	/** 拡張機能の識別子 */
	readonly id: string;

	/** ユーザ向けメタデータ */
	readonly metadata: {
		/** 拡張機能名 */
		name: string,
		/** 拡張機能の説明 */
		description?: string;
	},

	/** 初期化メソッド */
	setup(api: WeaverailApi): Promise<void>;

	/** 有効化処理 */
	activate?(): void;
	/** 無効化処理 */
	deactivate?(): void;
}