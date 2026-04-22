export interface WeaverailUiApi {
	registerPanel(panel: Panel): void;

	getPanels(slot: WeaverailSlot): Panel[];
}

export class WeaverailUiApiObject implements WeaverailUiApi {
	panels: Panel[] = [];

	registerPanel(panel: Panel): void {
		this.panels.push(panel);
	}

	getPanels(slot: WeaverailSlot): Panel[] {
		return [...this.panels.filter(v => v.slot === slot)];
	}
}

export type WeaverailSlot = 'sidebar' | 'main' | 'inspector' | 'toolbar' | 'bottom';

export interface Panel {
	/** パネルの識別名 */
	id: string;
	/** 表示ラベル */
	label: string;
	/** 配置場所 */
	slot: WeaverailSlot;
	/** 描画関数（Reactコンポーネントを返す） */
	render: () => React.ReactNode;
}