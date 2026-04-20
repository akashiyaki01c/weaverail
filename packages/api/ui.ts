export interface WeaverailUiApi {
	registerPanel(options: PanelRegistrationOptions): void;
	setContext(contextId: WeaverailContext): void;
	selection: {
		getActive(): string | null;
		setActive(id: string | null): void;
	}

	readonly events: {
		on(event: string, callback: (payload: any) => void): () => void;
	}
}

export class WeaverailUiApiObject {
	registerPanel(options: PanelRegistrationOptions): void { }
	setContext(contextId: WeaverailContext): void { }
	selection: {
		getActive(): string | null;
		setActive(id: string | null): void;
	}
	readonly events: {
		on(event: string, callback: (payload: any) => void): () => void;
	}

	constructor() {
		this.selection = {
			getActive() {
				return null;
			},
			setActive(id: string) {
			}
		};
		this.events = {
			on(event, callback) {
				return () => {}
			},
		}
	}
}

export type WeaverailContext = 'INFRASTRUCTURE' | 'SERVICE_DESIGN' | 'DIAGRAM' | 'GLOBAL';
export type WeaverailSlot = 'sidebar' | 'main' | 'inspector' | 'toolbar' | 'bottom';

export interface PanelRegistrationOptions {
	/** パネルの識別名 */
	id: string;
	/** 表示ラベル */
	label: string;
	/** 配置場所 */
	slot: WeaverailSlot;
	/** 表示するコンテキスト（指定しない場合はGLOBAL） */
	context?: WeaverailContext;
	/** 描画関数（Reactコンポーネントを返す） */
	render: () => React.ReactNode;
}