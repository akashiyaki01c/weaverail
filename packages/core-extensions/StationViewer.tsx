import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "../api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { useCallback, useEffect, useState } from "react";
import { Station, Track } from "@weaverail/types";
import { Button, Group, Stack, Table, TextInput } from "@mantine/core";

function StationViewer() {
  const { manager } = useExtensionManager();
  const [stations, setStations] = useState<{ [x: string]: Station }>();
  const [tracks, setTracks] = useState<{ [x: string]: Track }>();
  const [newStationName, setNewStationName] = useState("");
  const [newTrackNames, setNewTrackNames] = useState<{ [stationId: string]: string }>({});
  const [editingStationId, setEditingStationId] = useState<string | null>(null);
  const [editingStationName, setEditingStationName] = useState("");
  const [editingTrackId, setEditingTrackId] = useState<string | null>(null);
  const [editingTrackName, setEditingTrackName] = useState("");

  const reload = useCallback(async () => {
    setStations(await manager.api.data.getStations());
    setTracks(await manager.api.data.getTracks());
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
    await reload();
  };

  const removeStation = async (stationId: string) => {
    await manager.api.ops.station.remove_station(stationId);
    await reload();
  };

  const addTrack = async (stationId: string) => {
    const name = newTrackNames[stationId]?.trim();
    if (!name) return;
    const id = await manager.api.ops.station.new_track_id();
    await manager.api.ops.station.add_track({
      id,
      station_id: stationId,
      name,
      properties: {},
    });
    setNewTrackNames((current) => ({ ...current, [stationId]: "" }));
    await reload();
  };

  const renameTrack = async (trackId: string) => {
    const name = editingTrackName.trim();
    if (!name) return;
    await manager.api.ops.station.rename_track(trackId, name);
    setEditingTrackId(null);
    await reload();
  };

  const removeTrack = async (trackId: string) => {
    await manager.api.ops.station.remove_track(trackId);
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
              }}
            >
              {element.name}
            </Button>
          )}
        </Table.Td>
        <Table.Td>{element.id}</Table.Td>
        <Table.Td>
          <Stack gap="xs">
            {Object.values(tracks ?? {}).filter((track) => track.station_id === element.id).map((track) => (
              <Group key={track.id} gap="xs">
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
            <Group gap="xs">
              <TextInput
                placeholder="番線名"
                value={newTrackNames[element.id] ?? ""}
                onChange={(event) => setNewTrackNames((current) => ({
                  ...current,
                  [element.id]: event.currentTarget.value,
                }))}
                size="xs"
              />
              <Button size="xs" onClick={() => void addTrack(element.id)}>追加</Button>
            </Group>
          </Stack>
        </Table.Td>
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
          <Table.Th>番線</Table.Th>
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
