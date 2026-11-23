import { useCallback, useEffect, useState } from "react";
import { Stack, Table } from "@mantine/core";
import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import type { Line } from "@weaverail/types";
import { EmptyMessage } from "./EmptyMessage";

export function LineViewer() {
  const { manager } = useExtensionManager();
  const [lines, setLines] = useState<Record<string, Line>>({});

  const reload = useCallback(async () => {
    setLines(await manager.api.data.getLines());
  }, [manager]);

  useEffect(() => {
    void reload();
  }, [reload]);

  const values = Object.values(lines);

  return (
    <Stack>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>路線名</Table.Th>
            <Table.Th>ID</Table.Th>
            <Table.Th>駅間数</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {values.map((line) => (
            <Table.Tr key={line.id}>
              <Table.Td>{line.name}</Table.Td>
              <Table.Td>{line.id}</Table.Td>
              <Table.Td>{line.segments.length}</Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
      {values.length === 0 && <EmptyMessage text="路線がありません。" />}
    </Stack>
  );
}

export class LineViewerExtension implements WeaverailExtension {
  id = "weaverail.core.line-viewer";
  metadata = { name: "路線一覧", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.line-viewer.main-panel",
      label: "路線",
      slot: "main",
      render: () => <LineViewer />,
    });
  }
  deactivate(): void {}
}
