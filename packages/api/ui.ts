export interface UiEventMap {
	"extension-added": { id: string };
	"extension-removed": { id: string };
	"panel-registered": { panelId: string, slot: WeaverailSlot, name: string };
	"panel-unregistered": { panelId: string };
	"station-selected": { stationId: string | null; stationName: string | null };
	"line-selected": { lineId: string | null; lineName: string | null };
	"open-tab": { id: string; name: string; component: string };
	"close-tab": { id: string };
}

export type UiListener<K extends keyof UiEventMap> = (payload: UiEventMap[K]) => void;

export interface WeaverailUiApi {
	registerPanel(panel: Panel): () => void;

	getPanels(slot: WeaverailSlot): Panel[];

	getPanelById(id: string): Panel | undefined;
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