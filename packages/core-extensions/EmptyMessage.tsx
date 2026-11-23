import { Text } from "@mantine/core";

export function EmptyMessage({ text }: { text: string }) {
  return <Text c="dimmed">{text}</Text>;
}
