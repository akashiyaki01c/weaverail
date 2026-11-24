import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "../api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { useSelection } from "../app/src/SelectionContext";
import { useCallback, useEffect, useState } from "react";
import { Station, Track } from "@weaverail/types";
import { Button, Group, Stack, Table, TextInput } from "@mantine/core";

function StationViewer() {
  const { manager } = useExtensionManager();
  const { selectStation, clearSelection } = useSelection();
  const [stations, setStations] = useState<{ [x: string]: Station }>();
  const [newStationName, setNewStationName] = useState("");
  const [editingStationId, setEditingStationId] = useState<string | null>(null);
  const [editingStationName, setEditingStationName] = useState("");

  const reload = useCallback(async () => {
    setStations(await manager.api.data.getStations());
  }, [manager]);

  useEffect(() => {
    void reload();
  }, [reload]);

  const addStation = async () => {
    const name = newStationName.trim();
    if (!name) return;
    const id = await manager.api.ops.station.new_station_id();
    await manager.api.ops.station.add_station({
      id,
      name,
      properties: {},
    });
    setNewStationName("");
    await reload();
  };

  const renameStation = async (stationId: string) => {
    const name = editingStationName.trim();
    if (!name) return;
    await manager.api.ops.station.rename_station(stationId, name);
    setEditingStationId(null);
    selectStation(stationId, name);
    await reload();
  };

  const removeStation = async (stationId: string) => {
    await manager.api.ops.station.remove_station(stationId);
    clearSelection();
    await reload();
  };

  const components = [];
  for (const key in stations) {
    if (!Object.hasOwn(stations, key)) continue;

    const element = stations[key];

    components.push(
      <Table.Tr key={element.id}>
        <Table.Td>
          {editingStationId === element.id ? (
            <Group gap="xs">
              <TextInput
                value={editingStationName}
                onChange={(event) => setEditingStationName(event.currentTarget.value)}
                onKeyDown={(event) => {
                  if (event.key === "Enter") void renameStation(element.id);
                }}
                size="xs"
              />
              <Button size="xs" onClick={() => void renameStation(element.id)}>保存</Button>
              <Button size="xs" variant="subtle" onClick={() => setEditingStationId(null)}>取消</Button>
            </Group>
          ) : (
            <Button
              variant="subtle"
              size="compact-xs"
              onClick={() => {
                setEditingStationId(element.id);
                setEditingStationName(element.name);
                selectStation(element.id, element.name);
              }}
            >
              {element.name}
            </Button>
          )}
        </Table.Td>
        <Table.Td>{element.id}</Table.Td>
        <Table.Td>
          <Button size="xs" color="red" variant="subtle" onClick={() => void removeStation(element.id)}>削除</Button>
        </Table.Td>
      </Table.Tr>,
    );
  }

  return (
    <Stack>
      <Group>
        <TextInput
          label="駅名"
          placeholder="駅名"
          value={newStationName}
          onChange={(event) => setNewStationName(event.currentTarget.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter") void addStation();
          }}
        />
        <Button onClick={() => void addStation()}>駅を追加</Button>
      </Group>
      <Table>
      <Table.Thead>
        <Table.Tr>
		  <Table.Th>駅名</Table.Th>
          <Table.Th>ID</Table.Th>
          <Table.Th>操作</Table.Th>
        </Table.Tr>
      </Table.Thead>
	  <Table.Tbody>
		{components}
	  </Table.Tbody>
      </Table>
    </Stack>
  );
}

function TrackInspector() {
  const { manager } = useExtensionManager();
  const { selection } = useSelection();
  const stationId = selection?.kind === "station" ? selection.id : null;
  const stationName = selection?.kind === "station" ? selection.name : null;
  const [tracks, setTracks] = useState<{ [x: string]: Track }>({});
  const [newTrackName, setNewTrackName] = useState("");
  const [editingTrackId, setEditingTrackId] = useState<string | null>(null);
  const [editingTrackName, setEditingTrackName] = useState("");

  const reloadTracks = useCallback(async (selectedStationId: string) => {
    const allTracks = await manager.api.data.getTracks();
    setTracks(
      Object.fromEntries(
        Object.entries(allTracks).filter(
          ([, track]) => track.station_id === selectedStationId,
        ),
      ),
    );
  }, [manager]);

  useEffect(() => {
    setNewTrackName("");
    setEditingTrackId(null);
    setEditingTrackName("");
    setTracks({});
    if (stationId) {
      void reloadTracks(stationId);
    } else {
      setTracks({});
    }
  }, [stationId, reloadTracks]);

  const addTrack = async () => {
    if (!stationId) return;
    const name = newTrackName.trim();
    if (!name) return;
    const id = await manager.api.ops.station.new_track_id();
    await manager.api.ops.station.add_track({
      id,
      station_id: stationId,
      name,
      properties: {},
    });
    setNewTrackName("");
    await reloadTracks(stationId);
  };

  const renameTrack = async (trackId: string) => {
    const name = editingTrackName.trim();
    if (!name) return;
    await manager.api.ops.station.rename_track(trackId, name);
    setEditingTrackId(null);
    if (stationId) await reloadTracks(stationId);
  };

  const removeTrack = async (trackId: string) => {
    await manager.api.ops.station.remove_track(trackId);
    if (stationId) await reloadTracks(stationId);
  };

  if (!stationId) {
    return <Stack gap="xs"><strong>番線編集</strong><p>駅一覧から駅を選択してください。</p></Stack>;
  }

  return (
    <Stack gap="sm">
      <strong>{stationName ?? "番線編集"}</strong>
      {Object.values(tracks).map((track) => (
        <Group key={track.id} gap="xs" wrap="nowrap">
          {editingTrackId === track.id ? (
            <>
              <TextInput
                value={editingTrackName}
                onChange={(event) => setEditingTrackName(event.currentTarget.value)}
                size="xs"
              />
              <Button size="xs" onClick={() => void renameTrack(track.id)}>保存</Button>
              <Button size="xs" variant="subtle" onClick={() => setEditingTrackId(null)}>取消</Button>
            </>
          ) : (
            <>
              <Button
                variant="subtle"
                size="compact-xs"
                onClick={() => {
                  setEditingTrackId(track.id);
                  setEditingTrackName(track.name);
                }}
              >
                {track.name}
              </Button>
              <Button size="xs" color="red" variant="subtle" onClick={() => void removeTrack(track.id)}>削除</Button>
            </>
          )}
        </Group>
      ))}
      <Group gap="xs" wrap="nowrap">
        <TextInput
          placeholder="番線名"
          value={newTrackName}
          onChange={(event) => setNewTrackName(event.currentTarget.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter") void addTrack();
          }}
          size="xs"
        />
        <Button size="xs" onClick={() => void addTrack()}>追加</Button>
      </Group>
    </Stack>
  );
}

export class TrackInspectorExtension implements WeaverailExtension {
  id: string = "weaverail.core.track-inspector";
  metadata = { name: "番線編集", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.track-inspector.inspector-panel",
      label: "番線",
      slot: "inspector",
      render: function (): React.ReactNode {
        return <TrackInspector />;
      },
    });
  }
  deactivate(): void {}
}

export class StationViewerExtension implements WeaverailExtension {
  id: string = "weaverail.core.station-viewer";
  metadata = { name: "コア拡張機能", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.station-viewer.main-panel",
      label: "",
      slot: "main",
      render: function (): React.ReactNode {
        return <StationViewer />;
      },
    });
  }
  deactivate(): void {}
}
