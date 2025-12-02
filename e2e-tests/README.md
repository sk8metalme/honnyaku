# E2E統合テスト

このディレクトリには、Honnyaku アプリケーションの E2E (End-to-End) 統合テストが含まれています。

## セットアップ

### 必要な依存関係

すべての依存関係は `package.json` で管理されています：

```bash
npm install
```

### tauri-driver のインストール

tauri-driver は Tauri アプリケーションのテストに必要です：

```bash
cargo install tauri-driver
```

## テストの実行

### 完全な E2E テスト（ビルド込み）

```bash
npm run test:e2e
```

このコマンドは以下を実行します：
1. プロダクションビルドの作成 (`npm run tauri build`)
2. tauri-driver の起動
3. WebdriverIO テストの実行
4. クリーンアップ

### 注意事項

- テスト実行には macOS アクセシビリティ権限が必要です
- 初回実行時はビルドに時間がかかります（5-10分程度）
- Ollama がローカルで起動している必要はありませんが、一部のテストでは Ollama 接続テストが含まれる場合があります

## テストケース

### app-launch.spec.js

アプリケーションの起動と基本的な UI 要素の確認：

- アプリケーションが正常に起動すること
- メインコンテンツが表示されること
- ステータスインジケーター（アクセシビリティ権限、ショートカット、モデル）が表示されること
- 設定ボタンが存在すること
- 使い方セクションが表示されること

### settings-panel.spec.js

設定パネルの動作確認：

- 設定ボタンクリックでパネルが開くこと
- Ollama 設定フィールド（モデル名、エンドポイント）が表示されること
- ショートカット設定フィールドが表示されること
- 保存・閉じるボタンが存在すること
- 閉じるボタンクリックでパネルが閉じること

## トラブルシューティング

### tauri-driver が見つからない

```bash
which tauri-driver
# ~/.cargo/bin/tauri-driver が表示されることを確認
```

表示されない場合は、再インストール：

```bash
cargo install tauri-driver --force
```

### アプリケーションバイナリが見つからない

プロダクションビルドを手動で作成：

```bash
npm run tauri build
```

### テストがタイムアウトする

`wdio.conf.js` の `mochaOpts.timeout` を増やしてください（デフォルト: 60000ms）

## 制限事項

現在の E2E テストには以下の制限があります：

1. **グローバルショートカットのテスト不可**: WebDriver ではシステムレベルのショートカットをシミュレートできません
2. **クリップボード操作のテスト制限**: セキュリティ上の理由から、システムクリップボードへの直接アクセスは制限されます
3. **翻訳フローの完全テスト不可**: Ollama API との実際の通信は、テスト環境に依存します

これらの制限により、E2E テストは主に UI 要素の存在確認と基本的な操作に焦点を当てています。より詳細な機能テストは、フロントエンドの単体テスト（Vitest）とバックエンドの単体テスト（Rust）で行われます。

## 参考資料

- [Tauri v2 Testing Documentation](https://v2.tauri.app/develop/tests/)
- [WebdriverIO Documentation](https://webdriver.io/)
- [WebdriverIO Tauri Example](https://v2.tauri.app/develop/tests/webdriver/example/webdriverio/)
