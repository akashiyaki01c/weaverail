import { WeaverailApi, WeaverailApiObject } from "@weaverail/api";
import { WeaverailSlot } from "@weaverail/api/ui";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";

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
  private listeners = new Set<() => void>();

  addExtension(extension: WeaverailExtension, api: WeaverailApi) {
    if (this.extensions.has(extension.id)) {
      return;
    }
    extension.init(api);
    this.extensions.set(extension.id, extension);
    this.notify();
  }
  subscribe(callback: () => void) {
    this.listeners.add(callback);

    return () => {
      this.listeners.delete(callback);
    };
  }
  private notify() {
    console.log("notify");
    this.listeners.forEach((callback) => callback());
  }
}

/** WeaverailAPIを使用するためのReact hook */
export const useWeaverail = () => {
  const [api, _] = useState<WeaverailApi>(new WeaverailApiObject());
  return api;
};

const ExtensionContext = createContext<{
  manager: ExtensionManager;
  api: WeaverailApi;
  version: number;
} | null>(null);
export const ExtensionProvider = ({ children }: { children: ReactNode }) => {
  const [manager] = useState(() => {
    const m = new ExtensionManager();
    m.subscribe(() => setVersion((v) => v + 1));
    return m;
  });
  const api = useWeaverail();
  const [version, setVersion] = useState(0);

  return (
    <ExtensionContext.Provider value={{ manager, api, version }}>
      {children}
    </ExtensionContext.Provider>
  );
};

export const useExtensionPanels = (slot: WeaverailSlot) => {
  const ctx = useContext(ExtensionContext);
  const { api, version } = useExtensionManager();
  if (!ctx) return [];

  // ctx.version が変わるたびに、ここが再評価される
  return useMemo(() => {
    return ctx.api.ui.getPanels(slot);
  }, [api, slot, version]);
};

export const useExtensionManager = () => {
  const context = useContext(ExtensionContext);

  if (!context) {
    throw new Error(
      "useExtensionManager must be used within an ExtensionProvider",
    );
  }

  return {
    manager: context.manager,
    api: context.api,
    version: context.version,
  };
};
