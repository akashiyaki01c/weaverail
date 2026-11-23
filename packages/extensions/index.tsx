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
  /** 有効化関数 */
  activate(api: WeaverailApi): void | Promise<void>;
  /** 無効化関数 */
  deactivate(): void | Promise<void>;
}

/**
 * 拡張機能を管理する機能をまとめたインタフェース
 */
export class ExtensionManager {
  // 拡張機能一覧
  private extensions: Map<string, WeaverailExtension> = new Map();
  private panels: Panel[] = [];
  private extensionPanelDisposers = new Map<string, Set<() => void>>();
  private activatingExtensionId: string | undefined;
  private activationQueue: Promise<void> = Promise.resolve();
  // イベント購読
  private listeners: {
    [K in keyof UiEventMap]?: UiListener<K>[];
  } = {};
  private _api: WeaverailApi;

  constructor() {
    const manager = this;
    this._api = {
      ops: new WeaverailOpsApiObject(),
      data: new WeaverailDataApiObject(),
      ui: {
        registerPanel: function (panel: Panel): () => void {
          if (!manager.activatingExtensionId) {
            throw new Error("Panels can only be registered while an extension is activating");
          }
          if (manager.panels.some(v => v.id === panel.id)) {
            throw new Error(`Panel ID is already registered: ${panel.id}`);
          }
          if (!panel.id) {
            throw new Error("Panel ID is required");
          }
          manager.panels.push(panel);
          let disposed = false;
          const dispose = () => {
            if (disposed) return;
            disposed = true;
            manager.panels = manager.panels.filter(v => v.id !== panel.id);
            manager.dispatch("panel-unregistered", {
              panelId: panel.id,
            });
          };
          const disposers = manager.extensionPanelDisposers.get(manager.activatingExtensionId)
            ?? new Set<() => void>();
          disposers.add(dispose);
          manager.extensionPanelDisposers.set(manager.activatingExtensionId, disposers);
          manager.dispatch("panel-registered", {
            panelId: panel.id,
            slot: panel.slot,
            name: panel.label
          });
          return dispose;
        },
        getPanels: function (slot: WeaverailSlot): Panel[] {
          return manager.panels.filter((v) => v.slot === slot).slice();
        },
        getPanelById(id): Panel | undefined {
          return manager.panels.find(v => v.id === id);
        },
      },
    } satisfies WeaverailApi;
  }

  public get api(): WeaverailApi {
    return this._api;
  }

  async addExtension(extension: WeaverailExtension): Promise<boolean> {
    if (!extension.id) {
      throw new Error("Extension ID is required");
    }
    if (!extension.metadata?.name) {
      throw new Error(`Extension metadata name is required: ${extension.id}`);
    }
    if (this.extensions.has(extension.id)) {
      return false;
    }
    const activation = this.activationQueue.then(async () => {
      if (this.extensions.has(extension.id)) {
        return false;
      }
      this.activatingExtensionId = extension.id;
      try {
        await extension.activate(this.api);
        this.extensions.set(extension.id, extension);
        this.dispatch("extension-added", {
          id: extension.id,
        });
        return true;
      } catch (error) {
        this.disposeExtensionPanels(extension.id);
        throw error;
      } finally {
        this.activatingExtensionId = undefined;
      }
    });
    this.activationQueue = activation.then(
      () => undefined,
      () => undefined,
    );
    return activation;
  }

  async removeExtension(extensionId: string): Promise<boolean> {
    const extension = this.extensions.get(extensionId);
    if (!extension) {
      return false;
    }
    try {
      await extension.deactivate();
    } finally {
      this.disposeExtensionPanels(extensionId);
      this.extensions.delete(extensionId);
      this.dispatch("extension-removed", {
        id: extensionId,
      });
    }
    return true;
  }

  private disposeExtensionPanels(extensionId: string): void {
    const disposers = this.extensionPanelDisposers.get(extensionId);
    if (disposers) {
      for (const dispose of disposers) {
        dispose();
      }
    }
    this.extensionPanelDisposers.delete(extensionId);
  }

  on<K extends keyof UiEventMap>(event: K, listener: UiListener<K>) {
    const listeners = (this.listeners[event] ?? []) as UiListener<K>[];
    listeners.push(listener);
    this.listeners[event] = listeners as (typeof this.listeners)[K];

    // 解除用関数（useEffectのクリーンアップ等で使う）
    return () => {
      const current = (this.listeners[event] ?? []) as UiListener<K>[];
      this.listeners[event] = current.filter((l) => l !== listener) as (typeof this.listeners)[K];
    };
  }

  dispatch<K extends keyof UiEventMap>(event: K, payload: UiEventMap[K]) {
    const targets = this.listeners[event] as UiListener<K>[] | undefined;
    targets?.forEach((listener) => listener(payload));
  }
}
