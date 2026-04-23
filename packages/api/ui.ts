export interface UiEventMap {
	"extension-added": { id: string };
	"panel-registered": { panelId: string, slot: WeaverailSlot, name: string };
	"open-tab": { id: string; name: string; component: string };
	"close-tab": { id: string };
}

export type UiListener<K extends keyof UiEventMap> = (payload: UiEventMap[K]) => void;

export interface WeaverailUiApi {
	registerPanel(panel: Panel): void;

	getPanels(slot: WeaverailSlot): Panel[];

	getPanelById(id: string): Panel;
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
	render:  () => React.ReactNode;
}