# Weaverail

<p align="center">
  <img src="docs/image/diagram_logo.svg" alt="Weaverail Logo" width="200"/>
</p>

Weaverail は直感的で強力な次世代の鉄道ダイヤグラムエディタです。
複数路線を跨ぐ列車の定義、スクリプトによるデータ変換、拡張機能プラグインに対応します。

## 特徴

* 複数の路線を跨いだ列車が直感的に設定可能
* スクリプト機能によるデータの自動加工
* 拡張機能による機能追加が可能

## インストール

### リリースからダウンロード (推奨)

1. [Releaseページ](https://github.com/akashiyaki01c/weaverail/releases) から対応するOSの最新バージョンをダウンロードします。
2. 解凍し、実行ファイルを実行します。

### ソースからビルド

```bash
git clone https://github.com/akashiyaki01c/weaverail.git
cd weaverail

cargo tauri build
```

## ロードマップ

- [x] 基本機能(編集/表示/読み込み保存機能)
- [ ] スクリプト機能
- [ ] 拡張機能対応
- [ ] 安定版

## ライセンス

Weaverail は [MIT License](./LICENSE) の下で公開されています。
