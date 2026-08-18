# Test Architecture

## 目的

Weaverail のモデル層は、構造体の生成・状態遷移・参照整合性・入力検証・拡張プロパティ管理を中心に扱うため、テストは「仕様を壊す変更を早期に発見できる」ことを重視する。

## テスト戦略

### 1. 生成テスト

各モデルが持つ初期状態と必須フィールドを確認する。

- 例: `Train::new`, `Station::new`, `Line::new`, `Timetable::new`
- 確認項目:
  - ID が正しく設定されているか
  - 名前や参照が正しいか
  - デフォルト値が期待どおりか
  - `ExtensionProperty::new()` が使われているか

### 2. CRUD テスト

`DiagramRoot` に対する追加・取得・削除操作をテストする。

- `add_*` が `DuplicateKey` を返すか
- `delete_*` が `ObjectNotFound` を返すか
- `find_*_by_name` が妥当な検索結果を返すか
- 複数要素を追加した後に一括削除できるか

### 3. 参照整合性テスト

モデル同士の関連が正しいかを確認する。

- `Track` が `Station` を参照できるか
- `LineSegment` が `Station` を参照できるか
- `TemplateTrain` が `TrainType` と `Track` を参照できるか
- `Train` が `Timetable` と `TemplateSegment` を保持しているか

### 4. 外部参照制御テスト

削除時に他オブジェクトから参照されている場合は拒否する設計を確認する。

- `delete_segment` が `ExternalReferenced` を返すか
- `delete_track` が `ExternalReferenced` を返すか
- `delete_train_type` が `ExternalReferenced` を返すか

### 5. プロパティ管理テスト

`PropertiableObject` の実装を確認する。

- `set_property` で新規追加できるか
- `get_property` で値を取り出せるか
- `remove_property` で削除できるか
- 上書き時に前の値が返るか

### 6. バリデーションテスト

`validate_*` メソッドで、存在しない ID や不正な参照が検出されるか確認する。

- 参照先が存在しないとき `ObjectNotFound` を返すか
- 正常データで `Ok(())` になるか
- 列車/路線/時刻表の整合性が維持されているか

## 実施方針

- 1 モジュールごとに最小限の正常系と異常系のセットを持つ
- `#[cfg(test)] mod tests` でファイル内に閉じた構成にする
- `DiagramRoot::default()` を使って独立したルートを作成する
- `WeaverailId::new(n)` で ID を明示的に生成し、テストの再現性を保つ
- 失敗しやすい境界条件を優先して書く

## カバレッジ目標

- 生成: 100%
- CRUD: 100%
- 参照整合性: 90% 以上
- 検証/例外: 90% 以上
- プロパティ管理: 100%

## 現在の実装状況

現在、`weaverail-model/src/model` の主要モデルは以下を含むテストを保持している。

- `station`
- `train`
- `line`
- `timetable`
- `template_train`
- `line_segment`
- `track`
- `train_type`
- `diagram_view_settings`
- `segment_train_order`

これらは、モデルの基本動作と整合性の守り込みを目的とした unit test セットとして構成されている。
