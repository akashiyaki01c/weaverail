import { useCallback, useEffect, useState } from "react";
import { Stack, Table } from "@mantine/core";
import { WeaverailExtension } from "@weaverail/extensions";
import { WeaverailApi } from "@weaverail/api";
import { useExtensionManager } from "../app/src/ExtensionContext";
import type { TemplateTrain, TrainType } from "@weaverail/types";
import { EmptyMessage } from "./EmptyMessage";

export function TemplateTrainViewer() {
  const { manager } = useExtensionManager();
  const [templateTrains, setTemplateTrains] = useState<Record<string, TemplateTrain>>({});
  const [trainTypes, setTrainTypes] = useState<Record<string, TrainType>>({});

  const reload = useCallback(async () => {
    const [templates, types] = await Promise.all([
      manager.api.data.getTemplateTrains(),
      manager.api.data.getTrainTypes(),
    ]);
    setTemplateTrains(templates);
    setTrainTypes(types);
  }, [manager]);

  useEffect(() => {
    void reload();
  }, [reload]);

  const values = Object.values(templateTrains);

  return (
    <Stack>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>テンプレート列車名</Table.Th>
            <Table.Th>ID</Table.Th>
            <Table.Th>列車種別</Table.Th>
            <Table.Th>区間数</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {values.map((templateTrain) => (
            <Table.Tr key={templateTrain.id}>
              <Table.Td>{templateTrain.name}</Table.Td>
              <Table.Td>{templateTrain.id}</Table.Td>
              <Table.Td>
                {trainTypes[templateTrain.train_type_id]?.name ?? templateTrain.train_type_id}
              </Table.Td>
              <Table.Td>{templateTrain.segments.length}</Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
      {values.length === 0 && (
        <EmptyMessage text="テンプレート列車がありません。" />
      )}
    </Stack>
  );
}

export class TemplateTrainViewerExtension implements WeaverailExtension {
  id = "weaverail.core.template-train-viewer";
  metadata = { name: "テンプレート列車一覧", description: "" };
  activate(api: WeaverailApi) {
    api.ui.registerPanel({
      id: "weaverail.template-train-viewer.main-panel",
      label: "テンプレート列車",
      slot: "main",
      render: () => <TemplateTrainViewer />,
    });
  }
  deactivate(): void {}
}
