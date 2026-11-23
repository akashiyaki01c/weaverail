import { useCallback, useEffect, useState } from "react";
import { Stack, Table } from "@mantine/core";
import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import type { Timetable } from "@weaverail/types";
import { EmptyMessage } from "./EmptyMessage";

export function TimetableViewer() {
  const { manager } = useExtensionManager();
  const [timetables, setTimetables] = useState<Record<string, Timetable>>({});

  const reload = useCallback(async () => {
    setTimetables(await manager.api.data.getTimetables());
  }, [manager]);

  useEffect(() => {
    void reload();
  }, [reload]);

  const values = Object.values(timetables);

  return (
    <Stack>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>時刻表名</Table.Th>
            <Table.Th>ID</Table.Th>
            <Table.Th>駅間順序数</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {values.map((timetable) => (
            <Table.Tr key={timetable.id}>
              <Table.Td>{timetable.name}</Table.Td>
              <Table.Td>{timetable.id}</Table.Td>
              <Table.Td>{Object.keys(timetable.segment_train_orders).length}</Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
      {values.length === 0 && <EmptyMessage text="時刻表がありません。" />}
    </Stack>
  );
}

export class TimetableViewerExtension implements WeaverailExtension {
  id = "weaverail.core.timetable-viewer";
  metadata = { name: "時刻表一覧", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.timetable-viewer.main-panel",
      label: "時刻表",
      slot: "main",
      render: () => <TimetableViewer />,
    });
  }
  deactivate(): void {}
}
