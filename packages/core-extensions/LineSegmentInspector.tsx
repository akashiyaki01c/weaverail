import { useCallback, useEffect, useState } from "react";
import { Stack, Table, Text } from "@mantine/core";
import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import type { Line, LineSegment, Station } from "@weaverail/types";

export function LineSegmentInspector() {
  const { manager } = useExtensionManager();
  const [lineId, setLineId] = useState<string | null>(null);
  const [lineName, setLineName] = useState<string | null>(null);
  const [line, setLine] = useState<Line | null>(null);
  const [segments, setSegments] = useState<Record<string, LineSegment>>({});
  const [stations, setStations] = useState<Record<string, Station>>({});

  const reload = useCallback(async (selectedLineId: string) => {
    const [lines, allSegments, allStations] = await Promise.all([
      manager.api.data.getLines(),
      manager.api.data.getSegments(),
      manager.api.data.getStations(),
    ]);
    setLine(lines[selectedLineId] ?? null);
    setSegments(allSegments);
    setStations(allStations);
  }, [manager]);

  useEffect(() => {
    const unbind = manager.on("line-selected", (payload) => {
      setLineId(payload.lineId);
      setLineName(payload.lineName);
      setLine(null);
      if (payload.lineId) {
        void reload(payload.lineId);
      }
    });
    return unbind;
  }, [manager, reload]);

  useEffect(() => {
    if (lineId) {
      void reload(lineId);
    }
  }, [lineId, reload]);

  if (!lineId) {
    return (
      <Stack gap="xs">
        <strong>駅間一覧</strong>
        <Text c="dimmed">路線一覧から路線を選択してください。</Text>
      </Stack>
    );
  }

  const refs = line?.segments ?? [];

  return (
    <Stack gap="sm">
      <strong>{lineName ?? "駅間一覧"}</strong>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>順序</Table.Th>
            <Table.Th>開始駅</Table.Th>
            <Table.Th>終了駅</Table.Th>
            <Table.Th>方向</Table.Th>
            <Table.Th>ID</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {refs.map((ref, index) => {
            const segment = segments[ref.segment_id];
            if (!segment) return null;
            const start = stations[segment.start_station]?.name ?? segment.start_station;
            const end = stations[segment.end_station]?.name ?? segment.end_station;
            return (
              <Table.Tr key={segment.id}>
                <Table.Td>{index + 1}</Table.Td>
                <Table.Td>{ref.is_reversed ? end : start}</Table.Td>
                <Table.Td>{ref.is_reversed ? start : end}</Table.Td>
                <Table.Td>{ref.is_reversed ? "逆行" : "順行"}</Table.Td>
                <Table.Td>{segment.id}</Table.Td>
              </Table.Tr>
            );
          })}
        </Table.Tbody>
      </Table>
      {refs.length === 0 && <Text c="dimmed">駅間がありません。</Text>}
    </Stack>
  );
}

export class LineSegmentInspectorExtension implements WeaverailExtension {
  id = "weaverail.core.line-segment-inspector";
  metadata = { name: "駅間一覧", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.line-segment-inspector.inspector-panel",
      label: "駅間",
      slot: "inspector",
      render: () => <LineSegmentInspector />,
    });
  }
  deactivate(): void {}
}
