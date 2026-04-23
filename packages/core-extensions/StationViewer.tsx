import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "../api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import { useEffect, useState } from "react";
import { Station } from "@weaverail/types";
import { List, Table } from "@mantine/core";

function StationViewer() {
  const { manager } = useExtensionManager();
  const [stations, setStations] = useState<{ [x: string]: Station }>();

  useEffect(() => {
    (async () => {
      setStations(await manager.api.data.getStations());
    })();
  }, []);

  const components = [];
  for (const key in stations) {
    if (!Object.hasOwn(stations, key)) continue;

    const element = stations[key];

    components.push(
      <Table.Tr key={element.id}>
        <Table.Td>{element.name}</Table.Td>
		<Table.Td>{element.id}</Table.Td>
      </Table.Tr>,
    );
  }

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>#</Table.Th>
		  <Table.Th>駅名</Table.Th>
        </Table.Tr>
      </Table.Thead>
	  <Table.Tbody>
		{components}
	  </Table.Tbody>
    </Table>
  );
}

export class StationViewerExtension implements WeaverailExtension {
  id: string = "weaverail.core.station-viewer";
  metadata = { name: "コア拡張機能", description: "" };
  init(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.station-viewer.main-panel",
      label: "",
      slot: "main",
      render: function (): React.ReactNode {
        return <StationViewer />;
      },
    });
  }
  destroy(): void {}
}
