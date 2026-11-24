import { useCallback, useEffect, useState } from "react";
import { Button, Group, NumberInput, Select, Stack, Table, Text } from "@mantine/core";
import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { useSelection } from "../app/src/SelectionContext";
import type {
  LineSegment,
  Station,
  StopType,
  TemplateTrain,
  TemplateTrainSection,
  TemplateTrainSegment,
  TemplateTrainStation,
  Track,
} from "@weaverail/types";

export function TemplateTrainSegmentInspector() {
  const { manager } = useExtensionManager();
  const { selection } = useSelection();
  const trainId =
    selection?.kind === "template-train" ? selection.id : null;
  const trainName =
    selection?.kind === "template-train" ? selection.name : null;
  const [train, setTrain] = useState<TemplateTrain | null>(null);
  const [segments, setSegments] = useState<Record<string, LineSegment>>({});
  const [stations, setStations] = useState<Record<string, Station>>({});
  const [tracks, setTracks] = useState<Record<string, Track>>({});
  const [selectedSegmentId, setSelectedSegmentId] = useState<string | null>(null);
  const [runningTimeInput, setRunningTimeInput] = useState<number | string>(60);

  const reload = useCallback(async (selectedTrainId: string) => {
    const [trains, allSegments, allStations, allTracks] = await Promise.all([
      manager.api.data.getTemplateTrains(),
      manager.api.data.getSegments(),
      manager.api.data.getStations(),
      manager.api.data.getTracks(),
    ]);
    setTrain(trains[selectedTrainId] ?? null);
    setSegments(allSegments);
    setStations(allStations);
    setTracks(allTracks);
  }, [manager]);

  useEffect(() => {
    setTrain(null);
    if (trainId) void reload(trainId);
  }, [trainId, reload]);

  if (!trainId) {
    return (
      <Stack gap="xs">
        <strong>駅間一覧</strong>
        <Text c="dimmed">テンプレート列車一覧から列車を選択してください。</Text>
      </Stack>
    );
  }

  if (!train) {
    return (
      <Stack gap="xs">
        <strong>駅間一覧</strong>
        <Text c="dimmed">読み込み中...</Text>
      </Stack>
    );
  }

  const lastStationId =
    train.segments.length > 0
      ? train.segments[train.segments.length - 1].station.station_id
      : train.start_station.station_id;

  const addableSegments = Object.values(segments).filter((segment) =>
    segment.start_station === lastStationId ||
    segment.end_station === lastStationId,
  );

  const addSegment = async () => {
    if (!selectedSegmentId) return;
    const segment = segments[selectedSegmentId];
    if (!segment) return;

    const isReversed = segment.end_station === lastStationId;
    const nextStationId = isReversed
      ? segment.start_station
      : segment.end_station;
    const destinationTracks = Object.values(tracks).filter(
      (track) => track.station_id === nextStationId,
    );
    if (destinationTracks.length === 0) return;

    const [segmentId, stationId] = await Promise.all([
      manager.api.data.newTemplateTrainSegmentId(),
      manager.api.data.newTemplateTrainStationId(),
    ]);
    const templateSegment: TemplateTrainSegment = {
      id: segmentId,
      segment_id: segment.id,
      is_reversed: isReversed,
      running_time: Number(runningTimeInput) || 0,
      properties: {},
    };
    const templateStation: TemplateTrainStation = {
      id: stationId,
      station_id: nextStationId,
      track_id: destinationTracks[0].id,
      stop_time: "Pass",
      properties: {},
    };
    await manager.api.data.addTemplateTrainSegment(
      train.id,
      templateSegment,
      templateStation,
    );
    await reload(train.id);
  };

  const removeFront = async () => {
    await manager.api.data.popFrontTemplateTrainSegment(train.id);
    await reload(train.id);
  };

  const removeBack = async () => {
    await manager.api.data.popBackTemplateTrainSegment(train.id);
    await reload(train.id);
  };

  const updateSection = async (
    index: number,
    section: TemplateTrainSection,
  ) => {
    await manager.api.data.updateTemplateTrainSection(train.id, index, section);
  };

  const setRunningTime = async (
    index: number,
    section: TemplateTrainSection,
    value: number | string,
  ) => {
    const nextValue = Number(value) || 0;
    await updateSection(index, {
      ...section,
      segment: { ...section.segment, running_time: nextValue },
    });
  };

  const setStopType = async (
    index: number,
    section: TemplateTrainSection,
    value: string | null,
  ) => {
    const stopTime: StopType =
      value === "stop" ? { Stop: 30 } : "Pass";
    await updateSection(index, {
      ...section,
      station: { ...section.station, stop_time: stopTime },
    });
  };

  return (
    <Stack gap="sm">
      <strong>{trainName ?? "テンプレート列車"}</strong>
      <Group grow>
        <Select
          label="追加する駅間"
          placeholder="駅間を選択"
          data={addableSegments.map((segment) => ({
            value: segment.id,
            label: `${stations[segment.start_station]?.name ?? segment.start_station} - ${stations[segment.end_station]?.name ?? segment.end_station}`,
          }))}
          value={selectedSegmentId}
          onChange={setSelectedSegmentId}
        />
        <NumberInput
          label="運転時分"
          min={0}
          value={runningTimeInput}
          onChange={setRunningTimeInput}
        />
      </Group>
      <Button
        size="compact-sm"
        disabled={!selectedSegmentId || addableSegments.length === 0}
        onClick={() => void addSegment()}
      >
        末尾に追加
      </Button>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>順序</Table.Th>
            <Table.Th>駅間</Table.Th>
            <Table.Th>方向</Table.Th>
            <Table.Th>運転時分</Table.Th>
            <Table.Th>到着駅</Table.Th>
            <Table.Th>停車</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {train.segments.map((section, index) => {
            const segment = segments[section.segment.segment_id];
            if (!segment) return null;
            const startName =
              stations[segment.start_station]?.name ?? segment.start_station;
            const endName =
              stations[segment.end_station]?.name ?? segment.end_station;
            return (
              <Table.Tr key={section.segment.id}>
                <Table.Td>{index + 1}</Table.Td>
                <Table.Td>
                  {section.segment.is_reversed
                    ? `${endName} - ${startName}`
                    : `${startName} - ${endName}`}
                </Table.Td>
                <Table.Td>
                  {section.segment.is_reversed ? "逆行" : "順行"}
                </Table.Td>
                <Table.Td>
                  <NumberInput
                    size="xs"
                    min={0}
                    value={section.segment.running_time}
                    onChange={(value) =>
                      void setRunningTime(index, section, value)
                    }
                  />
                </Table.Td>
                <Table.Td>
                  {stations[section.station.station_id]?.name ??
                    section.station.station_id}
                </Table.Td>
                <Table.Td>
                  <Select
                    size="xs"
                    data={[
                      { value: "pass", label: "通過" },
                      { value: "stop", label: "停車" },
                    ]}
                    value={
                      typeof section.station.stop_time === "string"
                        ? "pass"
                        : "stop"
                    }
                    onChange={(value) =>
                      void setStopType(index, section, value)
                    }
                  />
                </Table.Td>
              </Table.Tr>
            );
          })}
        </Table.Tbody>
      </Table>
      {train.segments.length === 0 && (
        <Text c="dimmed">駅間がありません。</Text>
      )}
      <Group>
        <Button
          variant="light"
          size="compact-sm"
          disabled={train.segments.length === 0}
          onClick={() => void removeFront()}
        >
          先頭を削除
        </Button>
        <Button
          variant="light"
          color="red"
          size="compact-sm"
          disabled={train.segments.length === 0}
          onClick={() => void removeBack()}
        >
          末尾を削除
        </Button>
      </Group>
    </Stack>
  );
}

export class TemplateTrainSegmentInspectorExtension implements WeaverailExtension {
  id = "weaverail.core.template-train-segment-inspector";
  metadata = { name: "テンプレート列車駅間一覧", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.template-train-segment-inspector.inspector-panel",
      label: "駅間",
      slot: "inspector",
      render: () => <TemplateTrainSegmentInspector />,
    });
  }
  deactivate(): void {}
}
