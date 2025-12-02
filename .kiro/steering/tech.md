# Technology Stack

## Architecture

Tauri v2を採用したハイブリッドデスクトップアプリケーション。Rustバックエンドがシステム機能（ショートカット、クリップボード）を担当し、React/TypeScriptフロントエンドがUIを描画する。

```
┌─────────────────────────────────────┐
│         Frontend (WebView)          │
│    React + TypeScript + Tailwind    │
├─────────────────────────────────────┤
│           Tauri IPC                 │
├─────────────────────────────────────┤
│         Backend (Rust)              │
│  Global Shortcut, Clipboard, LLM   │
├─────────────────────────────────────┤
│    Ollama (localhost:11434)         │
│    Claude API (api.anthropic.com)   │
└─────────────────────────────────────┘
```

## Core Technologies

- **Language**: TypeScript (Frontend), Rust (Backend)
- **Framework**: Tauri v2, React 18
- **Runtime**: Node.js 20+, Rust 1.70+

## Key Libraries

### Frontend
- `@tauri-apps/api` - Tauri APIバインディング
- `@tauri-apps/plugin-clipboard-manager` - クリップボード操作
- `@tauri-apps/plugin-global-shortcut` - グローバルホットキー
- `@tauri-apps/plugin-store` - 設定永続化
- `tailwindcss` - スタイリング

### Backend (Rust)
- `tauri` - デスクトップアプリフレームワーク
- `reqwest` - HTTP クライアント（Ollama/Claude API）
- `serde` - JSON シリアライズ
- `tokio` - 非同期ランタイム

## Development Standards

### Type Safety
- TypeScript strict mode有効
- `any`型の使用禁止
- Rustは`#![deny(unsafe_code)]`を推奨

### Code Quality
- ESLint + Prettier（Frontend）
- Clippy + rustfmt（Backend）

### Testing
- Vitest（Frontend単体テスト）
- Rust標準テスト（Backend単体テスト）
- カバレッジ目標: 80%以上

## Development Environment

### Required Tools
- Node.js 20+
- Rust 1.70+ (rustup)
- Tauri CLI 2.x (`cargo install tauri-cli`)
- Ollama（ローカルLLM実行用）

### Common Commands
```bash
# Dev: 開発サーバー起動
npm run tauri dev

# Build: プロダクションビルド
npm run tauri build

# Test: テスト実行
npm test && cargo test

# Lint: コード品質チェック
npm run lint && cargo clippy
```

## Key Technical Decisions

1. **Tauri v2選択理由**: Electron比で軽量（~10MB）、Rustによる高パフォーマンス、macOSネイティブ機能への直接アクセス

2. **Ollama + Claude API**: ローカルLLMでプライバシー保護とオフライン対応、Claude APIで高精度翻訳のバックアップ

3. **React + TypeScript**: 豊富なエコシステム、型安全、Tauri公式サポート

4. **tauri-plugin-store**: SQLiteなどを使わず、JSONベースの軽量設定管理でシンプルさを維持

---
_Document standards and patterns, not every dependency_
