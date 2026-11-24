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
  const [history, setHistory] = useState<string[]>([]);
  const [historyIndex, setHistoryIndex] = useState(-1);
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
    let scriptSource = source;

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

    const lastStatement = sourceFile.statements.at(-1);
    if (lastStatement && ts.isExpressionStatement(lastStatement)) {
      const expressionText = lastStatement.getText(sourceFile);
      if (!expressionText.trimStart().startsWith("console.")) {
        const start = lastStatement.getStart(sourceFile);
        const end = lastStatement.getEnd();
        scriptSource =
          source.slice(0, start) +
          `console.log(${expressionText})` +
          source.slice(end);
      }
    }

    const result = ts.transpileModule(scriptSource, {
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
      setOutput((current) =>
        [current, result.output, result.success ? "" : result.error?.message ?? "実行に失敗しました。"]
          .filter((value) => value.length > 0)
          .join("\n"),
      );
      setHistory((current) => {
        const next = [...current, source];
        setHistoryIndex(next.length);
        return next;
      });
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
        <Button
          variant="light"
          disabled={running}
          onClick={async () => {
            await manager.api.data.resetScriptSession();
            setHistory([]);
            setHistoryIndex(-1);
            setOutput("");
            setError(null);
          }}
        >
          セッションリセット
        </Button>
        <Text c="dimmed" size="xs">Ctrl/Cmd+Enterでブロック実行</Text>
      </Group>
      <Textarea
        label="スクリプト"
        value={source}
        onChange={(event) => setSource(event.currentTarget.value)}
        autosize
        minRows={18}
        disabled={running}
        onKeyDown={(event) => {
          if (event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
            event.preventDefault();
            void run();
          } else if (event.key === "ArrowUp" && history.length > 0 && event.ctrlKey) {
            event.preventDefault();
            const nextIndex = Math.max(0, historyIndex - 1);
            setHistoryIndex(nextIndex);
            setSource(history[nextIndex]);
          } else if (event.key === "ArrowDown" && history.length > 0 && event.ctrlKey) {
            event.preventDefault();
            const nextIndex = Math.min(history.length - 1, historyIndex + 1);
            setHistoryIndex(nextIndex);
            setSource(history[nextIndex]);
          }
        }}
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
