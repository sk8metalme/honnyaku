# Research & Design Decisions

## Summary
- **Feature**: `ai-translation-desktop`
- **Discovery Scope**: New Feature（グリーンフィールド開発）
- **Key Findings**:
  - Tauri v2プラグインエコシステムがグローバルショートカット、クリップボード、設定管理を完全にサポート
  - Ollama APIはOpenAI互換エンドポイントを提供し、統一的なLLMクライアント実装が可能
  - macOSアクセシビリティ権限には専用プラグイン（tauri-plugin-macos-permissions）が利用可能

## Research Log

### Tauri v2 Global Shortcut Plugin
- **Context**: グローバルショートカット機能（要件1）の実装方法調査
- **Sources Consulted**:
  - [Tauri v2 Global Shortcut公式ドキュメント](https://v2.tauri.app/plugin/global-shortcut/)
  - [tauri-plugin-global-shortcut crates.io](https://crates.io/crates/tauri-plugin-global-shortcut)
- **Findings**:
  - `@tauri-apps/plugin-global-shortcut` パッケージでJavaScript APIを提供
  - `CommandOrControl+Shift+C` 形式でショートカット指定
  - `ShortcutState::Pressed` と `ShortcutState::Released` の両イベントをハンドリング可能
  - 他アプリとの競合時はハンドラが呼ばれない（`isRegistered`でチェック可能）
- **Implications**:
  - フロントエンドでショートカット登録、Rustバックエンドでシステム操作を実行
  - capabilities設定で権限（`global-shortcut:allow-register`等）の明示的許可が必要

### Tauri v2 Clipboard Manager Plugin
- **Context**: 選択テキスト取得機能（要件2）の実装方法調査
- **Sources Consulted**:
  - [Tauri v2 Clipboard公式ドキュメント](https://v2.tauri.app/plugin/clipboard/)
  - [tauri-plugin-macos-permissions](https://github.com/ayangweb/tauri-plugin-macos-permissions)
- **Findings**:
  - `@tauri-apps/plugin-clipboard-manager` でクリップボードの読み書きが可能
  - macOSアクセシビリティ権限には別途 `tauri-plugin-macos-permissions` が必要
  - 権限は `clipboard-manager:allow-read-text`, `clipboard-manager:allow-write-text` で制御
- **Implications**:
  - Cmd+C送信にはアクセシビリティ権限が必須
  - 初回起動時に権限リクエストダイアログを表示する必要あり

### Ollama REST API
- **Context**: ローカルLLM翻訳機能（要件4）の実装方法調査
- **Sources Consulted**:
  - [Ollama API公式ドキュメント](https://github.com/ollama/ollama/blob/main/docs/api.md)
  - [Ollama OpenAI互換](https://ollama.com/blog/openai-compatibility)
- **Findings**:
  - ベースURL: `http://localhost:11434/api`
  - `/api/chat` エンドポイントで会話形式の翻訳が可能
  - リクエスト: `{ model, messages, stream?, options? }`
  - レスポンス: `{ model, created_at, message: { role, content }, done }`
  - OpenAI互換エンドポイントも提供（`/v1/chat/completions`）
- **Implications**:
  - ストリーミングを無効化（`stream: false`）して単一レスポンスで取得がシンプル
  - qwen2.5:3bモデルを推奨（日本語/英語両対応、軽量）

### Claude Messages API
- **Context**: リモートLLM翻訳機能（要件4）の実装方法調査
- **Sources Consulted**:
  - [Claude Messages API公式ドキュメント](https://platform.claude.com/docs/en/api/messages)
  - [anthropic-sdk-typescript](https://github.com/anthropics/anthropic-sdk-typescript)
- **Findings**:
  - エンドポイント: `POST https://api.anthropic.com/v1/messages`
  - 必須パラメータ: `model`, `messages`, `max_tokens`
  - オプション: `system`, `temperature`, `stream`
  - 認証: `x-api-key` ヘッダー、`anthropic-version` ヘッダー必須
- **Implications**:
  - TypeScript SDKは存在するが、Tauriバックエンド（Rust）からの直接呼び出しが望ましい
  - `reqwest` クレートでHTTPSリクエストを実装

### macOS Keychain API (Rust)
- **Context**: APIキーの安全な保存（要件8.7）の実装方法調査
- **Sources Consulted**:
  - [security-framework crate](https://crates.io/crates/security-framework)
  - [rust-security-framework GitHub](https://github.com/kornelski/rust-security-framework)
- **Findings**:
  - `security-framework` クレートでmacOS Keychainにアクセス可能
  - `SecKeychain::set_generic_password` でパスワード保存
  - 署名されたアプリでないと一部機能に制限あり
- **Implications**:
  - プロダクションビルドでは署名が必要
  - 開発時は `tauri-plugin-store` でフォールバック可能

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Tauri Hybrid | Rust Backend + React Frontend | 軽量、型安全、macOSネイティブアクセス | 学習コスト | ステアリングで決定済み |
| Pure Rust | Rust + egui/iced | 最小サイズ、高速 | UI構築の複雑さ | 却下：React UIの方が開発効率高い |
| Electron | Node.js + Chromium | 豊富なエコシステム | 重い（100MB+） | 却下：サイズ要件（50MB以下）に適合しない |

**選択**: Tauri Hybrid - ステアリングドキュメントで決定済み、要件に最適

## Design Decisions

### Decision: LLMクライアントをRustバックエンドに実装

- **Context**: Ollama/Claude APIをどのレイヤーで呼び出すか
- **Alternatives Considered**:
  1. フロントエンド（TypeScript）から直接API呼び出し
  2. Rustバックエンドから呼び出し、IPCでフロントエンドに結果返却
- **Selected Approach**: Rustバックエンドから呼び出し
- **Rationale**:
  - Claude APIキーをフロントエンドに露出させない（セキュリティ）
  - HTTPクライアント処理をRustの`reqwest`で統一（パフォーマンス）
  - Keychainアクセスがバックエンドで完結
- **Trade-offs**: IPCオーバーヘッドが発生するが、テキスト量程度では無視できる
- **Follow-up**: エラーハンドリングのIPC設計を詳細化

### Decision: 言語検出をフロントエンドで実装

- **Context**: 言語検出ロジックの配置場所
- **Alternatives Considered**:
  1. Rustバックエンドで実装（`whatlang`クレート等）
  2. フロントエンドで実装（文字コード範囲判定）
  3. LLMに言語検出を依頼
- **Selected Approach**: フロントエンドで実装（文字コード範囲判定）
- **Rationale**:
  - 日本語/英語の判定は文字コード範囲（ひらがな、カタカナ、漢字）で十分
  - 追加ライブラリ不要、高速
  - LLM呼び出し回数を削減
- **Trade-offs**: 複雑な混合テキストでは精度が下がる可能性
- **Follow-up**: 将来の韓国語対応時に再評価

### Decision: 設定管理に tauri-plugin-store を使用

- **Context**: アプリ設定の永続化方法
- **Alternatives Considered**:
  1. SQLite（tauri-plugin-sql）
  2. tauri-plugin-store（JSONベース）
  3. ファイル直接書き込み
- **Selected Approach**: tauri-plugin-store
- **Rationale**:
  - 設定データは単純なKey-Value
  - SQLは過剰設計
  - Tauri公式サポートで安定性高い
- **Trade-offs**: 大量データには不向き（翻訳履歴実装時に再検討）
- **Follow-up**: APIキーのみKeychainに分離

## Risks & Mitigations

- **Risk 1: アクセシビリティ権限の拒否** — ユーザーが権限を拒否した場合、手動コピー（Cmd+C）後の翻訳にフォールバック
- **Risk 2: Ollamaが起動していない** — 起動状態チェックAPIを呼び出し、未起動時は設定画面でClaude API切り替えを促す
- **Risk 3: Claude APIキーの漏洩** — Keychainに保存、ログ出力時はマスク処理
- **Risk 4: ポップアップがフォーカスを奪う** — `decorations: false`, `skip_taskbar: true` で最小限の侵入性を実現

## References

- [Tauri v2 Global Shortcut](https://v2.tauri.app/plugin/global-shortcut/) — グローバルショートカットの登録・管理
- [Tauri v2 Clipboard](https://v2.tauri.app/plugin/clipboard/) — クリップボード読み書き
- [tauri-plugin-macos-permissions](https://github.com/ayangweb/tauri-plugin-macos-permissions) — macOS権限管理
- [Ollama API](https://github.com/ollama/ollama/blob/main/docs/api.md) — ローカルLLM API
- [Claude Messages API](https://platform.claude.com/docs/en/api/messages) — リモートLLM API
- [security-framework](https://crates.io/crates/security-framework) — macOS Keychainアクセス
