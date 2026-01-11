# Research & Design Decisions

## Summary
- **Feature**: `translation-summary-reply`
- **Discovery Scope**: Extension（既存システムの拡張）
- **Key Findings**:
  - 既存の翻訳フローは`useTranslationFlow`フックで管理され、状態パターンが確立されている
  - `TranslationPopup`コンポーネントは拡張可能な設計で、新しいpropsを追加しやすい
  - `translation.rs`はOllama統合パターンを持ち、新しいコマンドを追加可能

## Research Log

### 既存の状態管理パターン
- **Context**: useTranslationFlowの状態管理方法を調査
- **Sources Consulted**: `src/hooks/useTranslationFlow.ts`
- **Findings**:
  - 状態は`TranslationFlowState`型で管理（`idle`, `getting-selection`, `translating`, `completed`, `error`）
  - React hooksパターンを使用し、`useState`で状態を保持
  - エラーハンドリングは専用の`TranslationFlowError`型で型安全
- **Implications**: 新しい`ActionState`型を追加し、同じパターンで要約・返信状態を管理する

### Ollama API統合パターン
- **Context**: translation.rsのOllama呼び出しパターンを調査
- **Sources Consulted**: `src-tauri/src/services/translation.rs`
- **Findings**:
  - `reqwest`クライアントを使用してHTTP POST
  - グローバルクライアント（`HTTP_CLIENT`）でコネクションプーリング
  - エラーハンドリングは`TranslationError`型でマッピング（接続失敗、タイムアウト、APIエラー）
  - 処理時間は`Instant::now()`で計測
- **Implications**: 既存の`translate_with_ollama`関数と同じパターンで`summarize_with_ollama`と`generate_reply_with_ollama`を実装

### Tauri IPCコマンドパターン
- **Context**: 既存のTauriコマンドの登録方法を調査
- **Sources Consulted**: `src-tauri/src/lib.rs`
- **Findings**:
  - `#[tauri::command]`マクロでコマンド定義
  - `invoke_handler`に`generate_handler!`マクロで登録
  - 設定は`tauri-plugin-store`から取得（`ollamaModel`, `ollamaEndpoint`）
- **Implications**: 新しい`summarize`と`generate_reply`コマンドを同じパターンで追加

### UIコンポーネント拡張パターン
- **Context**: TranslationPopupの拡張方法を調査
- **Sources Consulted**: `src/components/TranslationPopup.tsx`
- **Findings**:
  - Props interfaceは明確に型定義されている
  - 状態ごとに条件レンダリング（`state === 'completed'`など）
  - Tailwind CSSクラスで色分け（`bg-blue-50`, `bg-gray-50`など）
  - コピーボタンは`CopyButton`コンポーネントとして分離
- **Implications**: 新しいpropsを追加し、`state === 'completed'`時に要約・返信ボタンと結果エリアを表示

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| 既存フック拡張 | useTranslationFlowに要約・返信機能を追加 | 一貫性、既存パターンの再利用 | フックが大きくなる | 選択：既存パターンとの整合性を重視 |
| 新規フック作成 | useActionFlow等の新しいフックを作成 | 関心の分離 | 状態の二重管理、複雑性増加 | 却下：シンプルさを損なう |

## Design Decisions

### Decision: 状態管理の拡張方法
- **Context**: 要約・返信の状態をどこで管理するか
- **Alternatives Considered**:
  1. useTranslationFlowに追加 — 既存フックを拡張
  2. 新規useActionFlowフック — 独立したフックで管理
- **Selected Approach**: useTranslationFlowに`actionState`, `summaryText`, `replyText`などの状態を追加
- **Rationale**: 翻訳完了後の追加アクションであり、翻訳フローと密結合しているため同じフックで管理するのが自然
- **Trade-offs**: フックが若干大きくなるが、状態の整合性が保たれやすい
- **Follow-up**: フックの行数が500行を超える場合はリファクタリングを検討

### Decision: プロンプト設計
- **Context**: 要約・返信のプロンプトをどのように構築するか
- **Alternatives Considered**:
  1. 言語別プロンプト — 日本語/英語で別々のプロンプト
  2. 言語パラメータ — プロンプトに言語を埋め込む
- **Selected Approach**: 言語別プロンプトを`build_summarize_prompt()`と`build_reply_prompt()`で構築
- **Rationale**: 既存の翻訳機能と同じパターン。言語に応じた自然なプロンプトを提供可能
- **Trade-offs**: 言語追加時はプロンプト関数の修正が必要
- **Follow-up**: 将来的に韓国語対応時はLanguage enumに`Korean`を追加

### Decision: UI色分け戦略
- **Context**: 要約・返信結果をどのように視覚的に区別するか
- **Alternatives Considered**:
  1. 色分け — 紫（要約）、緑（返信）
  2. アイコン — 同色だがアイコンで区別
- **Selected Approach**: 色分け（紫: `bg-purple-50`, 緑: `bg-green-50`）
- **Rationale**: 色は最も直感的な識別方法。Tailwind CSSで簡単に実装可能
- **Trade-offs**: 色覚障害者への配慮が必要（ラベルとアイコンでも区別可能にする）
- **Follow-up**: アクセシビリティレビューで検証

## Risks & Mitigations
- **Risk 1**: Ollamaモデルによっては要約・返信の品質が低い — 提案：qwen2.5:3b以上のモデルを推奨、設定画面でモデル選択を明示
- **Risk 2**: 長文翻訳結果の要約・返信生成時にタイムアウト — 提案：タイムアウトを60秒に設定（既存の翻訳と同じ）、エラーメッセージで再試行を促す
- **Risk 3**: 要約・返信ボタンの配置でUIが煩雑になる — 提案：ボタンはコンパクトに配置、結果エリアは折りたたみ可能にする

## References
- [Ollama API Documentation](https://github.com/ollama/ollama/blob/main/docs/api.md) — Chat APIエンドポイントの仕様
- [Tauri v2 IPC Guide](https://v2.tauri.app/develop/calling-rust/) — コマンド定義とフロントエンドからの呼び出し
- [React Hooks Best Practices](https://react.dev/learn/reusing-logic-with-custom-hooks) — カスタムフックの設計パターン
