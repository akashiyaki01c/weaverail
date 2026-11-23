import { useCallback, useEffect, useState } from "react";
import { Stack, Table } from "@mantine/core";
import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import type { TrainType } from "@weaverail/types";
import { EmptyMessage } from "./EmptyMessage";

export function TrainTypeViewer() {
  const { manager } = useExtensionManager();
  const [trainTypes, setTrainTypes] = useState<Record<string, TrainType>>({});

  const reload = useCallback(async () => {
    setTrainTypes(await manager.api.data.getTrainTypes());
  }, [manager]);

  useEffect(() => {
    void reload();
  }, [reload]);

  const values = Object.values(trainTypes);

  return (
    <Stack>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>列車種別名</Table.Th>
            <Table.Th>ID</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {values.map((trainType) => (
            <Table.Tr key={trainType.id}>
              <Table.Td>{trainType.name}</Table.Td>
              <Table.Td>{trainType.id}</Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
      {values.length === 0 && <EmptyMessage text="列車種別がありません。" />}
    </Stack>
  );
}

export class TrainTypeViewerExtension implements WeaverailExtension {
  id = "weaverail.core.train-type-viewer";
  metadata = { name: "列車種別一覧", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.train-type-viewer.main-panel",
      label: "列車種別",
      slot: "main",
      render: () => <TrainTypeViewer />,
    });
  }
  deactivate(): void {}
}
