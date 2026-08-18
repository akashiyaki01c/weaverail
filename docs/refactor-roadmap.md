# 段階的なリファクタ計画

## 目的

現在のアーキテクチャでは、`weaverail-model` が「状態定義」と「操作ロジック」を同時に持っており、責務の境界が曖昧になっている。

このリファクタでは、以下を順番に解決する。

- model の純粋さを回復する
- algorithm と operation の境界を明確にする
- 将来のクレート再編を安全に進められるようにする

---

## フェーズ 0: 現状整理

### 目標

- 依存の方向と責務の混在箇所を明文化する
- 現在の境界の問題をドキュメントに明示する

### 実施内容

- `weaverail-model` の責務を「データ構造」として固定する
- `weaverail-operation` を実体化する
- `weft-rail` / `warp-rail` を計算層として扱う

### 完了条件

- `weaverail-operation` が空ではなく、最初の operation API を持つ
- 計算層と状態層の境界が文書化されている

---

## フェーズ 1: operation 層の最小抽出

### 目標

- `DiagramRoot` の基本操作を `weaverail-operation` に移す最小形を確立する

### 実施内容

- `add_station`, `delete_station`, `find_station_by_name` を operation API として定義する
- この時点では `weaverail-model` の既存メソッドを利用するラッパーとして実装する
- `weaverail-operation` にテストを追加して、既存の挙動を保証する

### 完了条件

- `cargo test -p weaverail-operation` が通る
- 既存の `weaverail-model` と比較して振る舞いが同じであることを確認できる

---

## フェーズ 2: 操作 API の標準化

### 目標

- 操作の命名規則と戻り値規約を揃える

### 実施内容

- `add_*`, `delete_*`, `find_*`, `validate_*` の命名を統一する
- `ModelError` を使ってエラーを一貫させる
- `weaverail-operation` の API を `command` / `application service` に近づける

### 完了条件

- operation API が複数のドメインに再利用できる
- ルート周りのコードが model から分離されている

---

## フェーズ 3: 計算層の分離

### 目標

- `weft-rail` と `warp-rail` を「アルゴリズム層」として独立させる

### 実施内容

- ルートに対する計算は `weaverail-operation` が起点とする
- 計算結果を `Result*` として返す
- model 自体から計算ロジックを削除する

### 完了条件

- `weaverail-model` がドメイン保持に専念している
- `weft-rail` が計算のみに集中している

---

## フェーズ 4: model の純粋化

### 目標

- `weaverail-model` の API を data-only に近づける

### 実施内容

- `DiagramRoot` の内包 CRUD を減らす
- 参照整合性チェックは `operation` に寄せる
- モデルは構造定義と serialization を中心に保つ

### 完了条件

- model がライブラリとして再利用しやすい状態になる
- 依存方向が `model -> operation -> algorithm` のルールを守る

---

## 実行状況

現在の実施段階は「フェーズ 1: operation 層の最小抽出」を開始している。

- operation crate を実体化した
- station に対する基本操作のラッパーを実装した
- ここから他のモデルに対しても同じように API を広げていく

次の工程では、`train`, `timetable`, `track` などの基本 CRUD を同じパターンで外へ出す。
