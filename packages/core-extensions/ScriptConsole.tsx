import { useState } from "react";
import { Button, Group, Stack, Text, Textarea } from "@mantine/core";
import ts from "typescript";
import { useExtensionManager } from "../app/src/ExtensionContext";
import type { ScriptExecutionResult } from "@weaverail/api";

const FORBIDDEN_IDENTIFIERS = new Set([
  "fetch",
  "XMLHttpRequest",
  "WebSocket",
  "eval",
  "Function",
  "require",
  "process",
  "window",
  "globalThis",
  "tauri",
  "__TAURI__",
]);

export function ScriptConsole() {
  const { manager } = useExtensionManager();
  const [source, setSource] = useState(
    '// station/track API example\nconst stations = weaverail.project.stations.list();\nconsole.log(stations.length + " station(s)");',
  );
  const [output, setOutput] = useState("");
  const [running, setRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const transpile = (): string | null => {
    if (source.length > 256 * 1024) {
      setError("スクリプトは256KB以下にしてください。");
      return null;
    }

    const sourceFile = ts.createSourceFile(
      "script.ts",
      source,
      ts.ScriptTarget.ES2020,
      true,
      ts.ScriptKind.TS,
    );

    let blocked: string | null = null;
    const visit = (node: ts.Node) => {
      if (ts.isImportDeclaration(node) || ts.isExportAssignment(node) || ts.isExportDeclaration(node)) {
        blocked = "モジュール構文とexportは使用できません。";
      }
      if (ts.isAwaitExpression(node)) {
        blocked = "トップレベルのawaitは使用できません。";
      }
      if (ts.isIdentifier(node) && FORBIDDEN_IDENTIFIERS.has(node.text)) {
        blocked = `禁止された識別子: ${node.text}`;
      }
      if (!blocked) ts.forEachChild(node, visit);
    };
    visit(sourceFile);
    if (blocked) {
      setError(blocked);
      return null;
    }

    const result = ts.transpileModule(source, {
      compilerOptions: {
        target: ts.ScriptTarget.ES2020,
        module: ts.ModuleKind.None,
      },
      fileName: "script.ts",
      reportDiagnostics: true,
    });

    const diagnostics = (result.diagnostics ?? []).filter(
      (diagnostic) => diagnostic.category === ts.DiagnosticCategory.Error,
    );
    if (diagnostics.length > 0) {
      setError(
        diagnostics
          .map((diagnostic) =>
            ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"),
          )
          .join("\n"),
      );
      return null;
    }

    return result.outputText;
  };

  const run = async () => {
    setError(null);
    setOutput("");
    const javascript = transpile();
    if (!javascript) return;

    setRunning(true);
    try {
      const result: ScriptExecutionResult = await manager.api.data.runScript(javascript);
      setOutput(result.output);
      if (!result.success) {
        setError(result.error?.message ?? "スクリプト実行に失敗しました。");
      }
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setRunning(false);
    }
  };

  return (
    <Stack>
      <Group>
        <Button onClick={() => void run()} loading={running} disabled={running}>
          実行
        </Button>
      </Group>
      <Textarea
        label="スクリプト"
        value={source}
        onChange={(event) => setSource(event.currentTarget.value)}
        autosize
        minRows={18}
        disabled={running}
      />
      {error && (
        <Text c="red" style={{ whiteSpace: "pre-wrap" }}>
          {error}
        </Text>
      )}
      <Textarea
        label="出力"
        value={output}
        readOnly
        autosize
        minRows={8}
      />
    </Stack>
  );
}
