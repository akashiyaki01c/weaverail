# 操作分離のアーキテクチャ案

## 背景

現状の構成では、`weaverail-model` が以下を同時に担っている。

- ドメインのデータ構造定義
- 参照整合性の検証
- ルート上の CRUD 的な操作
- 計算や変換ロジックの一部
- `weft-rail` を直接利用する演算処理

この結果、モデルが「データ」ではなく「振る舞い」を持ち、依存関係が複雑化している。
特に `weft-rail` が `weaverail-model` を前提にしている一方で、`weaverail-model` 側が `weft-rail` に依存している場合、境界が曖昧になりやすい。

この状態は、以下のような限界を生みやすい。

- モデルの再利用が難しい
- ルールとデータの責務が混ざる
- 依存方向が逆方向に近くなり、変更に弱い
- テストが現実の計算ロジックまで踏み込んでしまう

## 目標

「`weaverail-model` は純粋なデータ構造だけを持ち、操作・計算・制約解決は `weaverail-operation` やアルゴリズムクレートへ分離する」
という構造を目指したい。

## 前提の整理

### 1. `weaverail-model` の責務

`weaverail-model` は、できるだけ以下のみに限定する。

- `DiagramRoot` などの状態保持構造
- `Station`, `Track`, `Train`, `Timetable` などのドメインモデル
- 参照 ID と構造の型定義
- シリアライズ / TS 移行 / 生成系のユーティリティ
- 最小限の「整合性チェック」だけを持つ

### 2. `weaverail-operation` の責務

`weaverail-operation` は、モデルを入力として受け取り、状態遷移や計算結果を返す。

- 追加 / 削除 / 並び替え / 更新の処理
- ルートからの導出処理
- 時刻計算やルート整理の操作
- `weft-rail` / `warp-rail` と協調するアプリケーション層

### 3. `weft-rail` の責務

`weft-rail` は純粋な計算エンジンに寄せる。

- 制約付きの列車時刻計算
- グラフ生成や探索
- 差分更新アルゴリズム
- 計算結果を `ResultWeftTrain` のような出力へ変換

## 提案案

### 案A: 最小分離案

#### 方向性

- `weaverail-model` は純粋な DTO とする
- `weaverail-operation` に `DiagramRootOps`, `TrainOps`, `TimetableOps` などを置く
- `weft-rail` は `weaverail-model` を読むだけにして、`weaverail-operation` から呼び出す

#### 例

```rust
// weaverail-model
pub struct DiagramRoot {
    pub stations: IndexMap<StationId, Station>,
    pub tracks: IndexMap<TrackId, Track>,
    pub trains: IndexMap<TrainId, Train>,
    // ...
}

// weaverail-operation
pub fn add_station(root: &mut DiagramRoot, station: Station) -> Result<(), ModelError> {
    // 追加ロジック
}

pub fn weave(root: &DiagramRoot, timetable_id: TimetableId) -> Result<Vec<ResultWeftTrain>, ModelError> {
    weft_rail::weave(root, timetable_id)
}
```

#### メリット

- 変更範囲が比較的小さい
- 既存の model を壊しにくい
- 運用的には最も安全

#### デメリット

- 依然として `weaverail-operation` にロジックが集中しやすい
- `DiagramRoot` の操作 API が散らばる可能性がある

#### 向いているケース

- 既存のコードを段階的に整理したい
- すぐに責務分離を始めたい

---

### 案B: Application Layer + Domain Layer に分ける案

#### 方向性

- `weaverail-model` = ドメインモデル
- `weaverail-operation` = アプリケーションサービス
- `weft-rail` = アルゴリズム実装 / 計算サービス

#### 役割分担

- `weaverail-model` : 何を持つかを定義
- `weaverail-operation` : 「何をするか」を定義
- `weft-rail` : 「どう計算するか」を定義

#### 例

```rust
// model
pub struct Timetable {
    pub id: TimetableId,
    pub name: String,
    // ...
}

// operation
pub trait TimetableService {
    fn create_timetable(&self, root: &mut DiagramRoot, name: &str) -> Result<TimetableId, ModelError>;
    fn build_train_schedule(&self, root: &DiagramRoot, timetable_id: TimetableId) -> Result<Vec<ResultWeftTrain>, ModelError>;
}

// weft-rail
pub fn weave(root: &DiagramRoot, timetable_id: TimetableId) -> Result<Vec<ResultWeftTrain>, ModelError> {
    // グラフ計算
}
```

#### メリット

- 設計として明快
- ドメインとアプリケーションの境界がはっきりする
- 将来のリプレースがしやすい

#### デメリット

- ひとつの機能をたくさんのレイヤーにまたがって作る必要がある
- 既存コードとの整合に少しコストがかかる

#### 向いているケース

- アーキテクチャを大きく整理したい
- 長期的な設計に対して明確な境界を作りたい

---

### 案C: ルールエンジン分離案

#### 方向性

`weaverail-model` を最小限の状態保持にし、`weaverail-operation` で処理ルールを束ねる。

- ドメインモデルは state only
- ルールや操作は `OperationContext` / `ConstraintChecker` に切り出す
- `weft-rail` は「制約ソルバ」に特化する

#### 例

```rust
pub struct DiagramContext<'a> {
    pub root: &'a DiagramRoot,
    pub current_timetable: Option<TimetableId>,
}

pub fn validate_station_links(ctx: &DiagramContext, station_id: StationId) -> Result<(), ModelError> {
    // 参照検証
}

pub fn solve_schedule(ctx: &DiagramContext) -> Result<Vec<ResultWeftTrain>, ModelError> {
    weft_rail::weave(ctx.root, ctx.current_timetable.unwrap())
}
```

#### メリット

- ビジネスルールと状態の切り離しが明確
- 制約解決と状態変更の責任が分離しやすい
- 将来的に「評価器」や「制約チェック」を増やしやすい

#### デメリット

- 文脈管理が増え、API がやや抽象的になる
- 過剰に設計を複雑化しやすい

#### 向いているケース

- 制約が多く、ルールが複雑な設計に向く
- 「計算エンジン」と「ドメイン状態」を厳密に分離したい

---

### 案D: 外部サービス化（Ports & Adapters）案

#### 方向性

`weaverail-model` を純粋なデータ層として扱い、計算や外部依存は `trait` に抽象化する。

```rust
pub trait ScheduleEngine {
    fn weave(&self, root: &DiagramRoot, timetable_id: TimetableId) -> Result<Vec<ResultWeftTrain>, ModelError>;
}
```

実装として `weft-rail` がその `trait` を満たす。

#### メリット

- テストでエンジンを差し替えやすい
- 将来的に別計算実装への移行が容易
- アーキテクチャとして筋が良い

#### デメリット

- 抽象化が増え、コード量が増える
- 小さなプロジェクトにはやや重い

#### 向いているケース

- 将来的に計算ロジックを差し替える可能性が高い
- 複数アルゴリズムを持ちたい

---

## 推奨方向

個人的には、現状の限界を解消する最適解は「案A をベースに、必要に応じて案B の思想を取り入れる」形だと考えている。

### 推奨構成

```text
weaverail-model
  └─ pure data model / typed IDs / serialization

weaverail-operation
  ├─ DiagramRootOps
  ├─ TrainOperations
  ├─ TimetableOperations
  └─ ScheduleService

weft-rail
  └─ pure algorithm engine

warp-rail
  └─ geometry / layout calculation engine
```

### 原則

1. `weaverail-model` は依存してはいけない
   - `weft-rail` や `warp-rail` の型を参照しない
   - 方向は `model -> operation -> algorithm` でまとめる

2. `weaverail-operation` はモデルを受け取り、結果を返す
   - ルート変更や計算の起点をここに置く

3. アルゴリズム層は副作用を最小化する
   - 乱数やグローバル状態に依存しない
   - 返り値を明確にしてテストしやすくする

4. 依存の方向は一方向に保つ
   - `weft-rail -> weaverail-model` は許容
   - `weaverail-model -> weft-rail` は避ける

## 他クレートの責務境界

ここからは、今回の構成に含まれる他のクレートを対象に、責務の境界を明確化する。

### 1. `warp-rail`

#### 役割

- ダイヤグラム上の座標やレイアウト計算
- 列車・駅・線路の幾何計算
- 画面上の配置計算を担当する

#### 望ましい境界

- `DiagramRoot` を読み取り、計算結果だけを返す
- バックエンドの状態変更やファイル保存を行わない
- ドメインルールを持たない純粋な計算層にする

#### 不都合

- `warp-rail` が `weaverail-model` の内部実装や CRUD を直接踏んでしまうと、レイアウト計算が domain へ強く依存してしまう
- 座標計算がルール変更に巻き込まれ、テストしにくくなる

#### 解決策

- `warp-rail` には `LayoutInput` などの計算専用入力を与える
- 返り値は `WarpRailResult` のような純粋な DTO にする
- 変換や検証の責務は `weaverail-operation` 側へ寄せる

---

### 2. `weaverail-io`

#### 役割

- プロジェクトデータの保存・読み込み
- serde / ron / zstd を使った永続化
- スキーマ互換やバージョン間変換

#### 望ましい境界

- `weaverail-model` を読み書きする adapter に限定する
- ビジネスロジックや時刻計算を持たない
- 直に演算や計算を行わない

#### 不都合

- `weaverail-io` に「保存形式の変換」だけでなく、ドメインロジックが混ざると、I/O と業務ルールが一体化する
- データファイルの仕様変更が、ドメインを巻き込む

#### 解決策

- `ProjectCodec` などのアダプタ trait を導入する
- `weaverail-io` は「読み書き」「変換」だけに限定し、業務的な検証は `weaverail-operation` に委譲する
- 互換性変換は versioned codec に分離する

---

### 3. `weaverail-object`

#### 役割

- `weaverail_id!` のような proc macro
- 共通の型定義・derive ヘルパー
- モデル定義の記述を簡潔にする

#### 望ましい境界

- ルール層・ビジネス層・計算層には入らない
- ただし、各モデルの ID 定義や Object 系の補助を担う

#### 不都合

- ここに domain-specific なロジックを置くと、macro が意味を持ちすぎて保守しにくくなる
- どのクラスが「共通部品」なのかが曖昧になる

#### 解決策

- `weaverail-object` は「メタプログラミング共通」だけに留める
- ビジネスロジックは model / operation に寄せる
- macro で生成されるコードはなるべく単純に保つ

---

### 4. `src-tauri` / IPC 層

#### 役割

- UI からの command を受ける
- ルート上の API を呼ぶ
- 依存の境界としてのアダプタ層

#### 望ましい境界

- command は薄くする
- 変換やバリデーションそのものを command に持たせない
- 実際の振る舞いは `weaverail-operation` に委譲する

#### 不都合

- ここにドメイン処理が入りやすく、UI の実装や command 定義が肥大化する
- Tauri の command が「アプリケーションサービス」になってしまう

#### 解決策

- command は DTO 変換と入力検証に限定する
- 実処理は `operation::` の function に委譲する
- command output を API 経由で明確に決める

---

### 5. `weaverail-testdata`

#### 役割

- テスト用の fixture 生成
- 事前に定義されたダイアグラムやデータセットの提供

#### 望ましい境界

- ただし、テスト用のワークフローそのものを作るのではなく、データの材料供給に限定する
- 既存のロジックに依存しすぎない

#### 不都合

- テストデータが実運用コードのロジックを強く参照していると、テストが本来のモデル設計に縛られる
- fixture 生成のルールが「仕様」そのものになりやすい

#### 解決策

- `weaverail-testdata` には「fixture builder」だけを置く
- 実際の変換ロジックや計算結果生成は別のドメイン層へ移す
- 実データとテストデータの関係を明示的に定義する

---

## 各クレートにおける最終的な責務図

```text
UI / Tauri
  └─ command adapter layer
       ↓
weaverail-operation
  ├─ mutation / validation / orchestration
  ├─ domain service
  └─ algorithm orchestration
       ↓
weaverail-model
  └─ immutable-ish domain state + typed IDs + serialization schema

weft-rail
  └─ schedule / constraint solving engine

warp-rail
  └─ geometry / layout engine

weaverail-io
  └─ persistence adapter

weaverail-object
  └─ macro helper / code-generation support
```

## 原則として守るべきルール

1. 「状態」を持つのは model と operation のみとする
2. 「計算」を持つのは algorithm 系 (weft-rail, warp-rail) とする
3. 「保存」を持つのは weaverail-io とする
4. 「UI との接続」を持つのは Tauri / command とする
5. 「共通 codegen」は weaverail-object に限定する
6. 依存方向は原則一方向に保つ

## 解決策の要点

今回の不都合は、責務が混ざっている場所が多いことが根本原因である。
特に以下が大きい。

- `weaverail-model` が操作ロジックを持つ
- `weft-rail` が model 自体にロジックを持つように見える
- `Tauri command` が domain を直接扱っている
- `weaverail-io` がビジネスルールまで含んでいる

これを解決するには、次のように整理するのが安全である。

- 「データを持つ層」
- 「状態を変える層」
- 「計算をする層」
- 「IO を担う層」
- 「UI を繋ぐ層」

を分離し、それぞれの依存方向を制約することだ。

## 結論

crates ごとの責務を明確に分けることは、単なる整理ではなく、変更耐性・テスト容易性・将来の置換性を高める設計判断である。

今回の構成では、特に `weaverail-model` と `weft-rail` の境界、`src-tauri` と `weaverail-operation` の境界、`weaverail-io` と model の境界を明確にすることで、十分に現実的な改善が可能になる。

この整理を守ることで、今後のリプレースやクレート再編が比較的安全に行えるようになる。

## 実装の進め方

### フェーズ1: 責務の分離だけを行う

- `weaverail-model` から操作メソッドを削る
- `weaverail-operation` に関数を置く
- 既存呼び出しを順次切り替える

### フェーズ2: 計算器を明確化する

- `weft-rail` を計算専用に寄せる
- `DiagramRoot` を読み取り専用として扱う

### フェーズ3: インターフェース化する

- `ScheduleEngine` などの trait を導入
- 将来の置き換えを容易にする

## まとめ

今回の課題は、「モデルがデータとロジックの両方を持ちすぎている」ことにある。
その意味で、`weaverail-model` を純粋なデータ構造へ戻し、`weaverail-operation` と `weft-rail` に責務を分離する方向は非常に妥当である。

特に、依存方向を「データ層 → 操作層 → 計算層」に揃えると、テストしやすさ、拡張しやすさ、変更耐性が大きく改善する。

次の実務では、まず `weaverail-model` のメソッド群を削り、`weaverail-operation` に移すところから始めるのが一番安全で、設計としても筋が良い。
