import { createContext, useContext, useMemo, useState } from "react";

export type WeaverailSelection =
  | { kind: "station"; id: string; name: string }
  | { kind: "line"; id: string; name: string }
  | { kind: "template-train"; id: string; name: string }
  | null;

interface SelectionContextValue {
  selection: WeaverailSelection;
  selectStation: (id: string, name: string) => void;
  selectLine: (id: string, name: string) => void;
  selectTemplateTrain: (id: string, name: string) => void;
  clearSelection: () => void;
}

const SelectionContext = createContext<SelectionContextValue | null>(null);

export function SelectionProvider({ children }: { children: React.ReactNode }) {
  const [selection, setSelection] = useState<WeaverailSelection>(null);

  const value = useMemo<SelectionContextValue>(
    () => ({
      selection,
      selectStation: (id, name) => setSelection({ kind: "station", id, name }),
      selectLine: (id, name) => setSelection({ kind: "line", id, name }),
      selectTemplateTrain: (id, name) =>
        setSelection({ kind: "template-train", id, name }),
      clearSelection: () => setSelection(null),
    }),
    [selection],
  );

  return (
    <SelectionContext.Provider value={value}>
      {children}
    </SelectionContext.Provider>
  );
}

export function useSelection(): SelectionContextValue {
  const context = useContext(SelectionContext);
  if (!context) {
    throw new Error("useSelection must be used within SelectionProvider");
  }
  return context;
}
