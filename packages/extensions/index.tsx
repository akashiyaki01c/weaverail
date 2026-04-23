import {
  WeaverailApi,
  WeaverailDataApiObject,
  WeaverailOpsApiObject,
} from "@weaverail/api";
import {
  Panel,
  UiEventMap,
  UiListener,
  WeaverailSlot,
} from "@weaverail/api/ui";

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
  init(api: WeaverailApi): void;
  /** 始末関数 */
  destroy(): void;
}

/**
 * 拡張機能を管理する機能をまとめたインタフェース
 */
export class ExtensionManager {
  // 拡張機能一覧
  private extensions: Map<string, WeaverailExtension> = new Map();
  private panels: Panel[] = [];
  // イベント購読
  private listeners = new Map<string, Function[]>();
  private _api: WeaverailApi;

  constructor() {
    const manager = this;
    this._api = {
      ops: new WeaverailOpsApiObject(),
      data: new WeaverailDataApiObject(),
      ui: {
        registerPanel: function (panel: Panel): void {
          if (manager.panels.some(v => v.id === panel.id))
            return;
          manager.panels.push(panel);
          manager.dispatch("panel-registered", {
            panelId: panel.id,
            slot: panel.slot,
            name: panel.label
          });
        },
        getPanels: function (slot: WeaverailSlot): Panel[] {
          return manager.panels.filter((v) => v.slot === slot);
        },
        getPanelById(id): Panel {
          return manager.panels.find(v => v.id === id)!
        },
      },
    } satisfies WeaverailApi;
  }

  public get api(): WeaverailApi {
    return this._api;
  }

  addExtension(extension: WeaverailExtension) {
    if (this.extensions.has(extension.id)) {
      return;
    }
    extension.init(this.api);
    this.extensions.set(extension.id, extension);
    this.dispatch("extension-added", {
      id: extension.id,
    });
  }

  on<K extends keyof UiEventMap>(event: K, listener: UiListener<K>) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)?.push(listener);

    // 解除用関数（useEffectのクリーンアップ等で使う）
    return () => {
      this.listeners.set(
        event,
        this.listeners.get(event)!.filter((l) => l !== listener),
      );
    };
  }

  dispatch<K extends keyof UiEventMap>(event: K, payload: UiEventMap[K]) {
    const targets = this.listeners.get(event);
    console.log(event, targets)
    if (targets) {
      targets.forEach((listener) => listener(payload));
    }
  }
}
