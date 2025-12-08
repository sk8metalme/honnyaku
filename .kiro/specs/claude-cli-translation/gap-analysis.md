# Implementation Gap Analysis

## 分析サマリー

- **スコープ**: 既存のOllama専用翻訳アプリに、Claude CLI（`claude -p`）翻訳方式を第3の選択肢として追加
- **主な課題**: ステアリングドキュメントでは「デュアルLLM対応（Ollama/Claude API）」と記載されているが、現状はOllama専用実装。Claude API実装も存在しないため、Claude CLI追加は実質的に最初のClaude統合となる
- **推奨アプローチ**: ハイブリッド方式（既存パターン拡張 + 新規コンポーネント作成）で段階的に実装

---

## 1. 現状調査

### 1.1 既存アーキテクチャ

#### フロントエンド（TypeScript）

**主要ファイル:**
- `src/hooks/useTranslation.ts` - 翻訳ロジック、`invoke('translate', {...})`でバックエンド呼び出し
- `src/hooks/useTranslationFlow.ts` - ショートカット→クリップボード取得→翻訳の完全フロー
- `src/hooks/useSettings.ts` - 設定管理（`get_settings`, `save_settings`）
- `src/lib/language-detect.ts` - 言語検出ユーティリティ
- `src/types/index.ts` - 型定義（`AppSettings`, `TranslationResult`等）
- `src/components/SettingsPanel.tsx` - 設定UI（Ollamaモデル/エンドポイント選択）
- `src/components/TranslationPopup.tsx` - 翻訳結果表示

**既存パターン:**
- Tauri IPC経由で`translate`コマンドを呼び出し
- 言語検出はフロントエンド側で実行（`detectLanguage()`）
- 設定は`tauri-plugin-store`で永続化（JSON形式）

#### バックエンド（Rust）

**主要ファイル:**
- `src-tauri/src/lib.rs` - Tauriコマンド定義（`translate`, `get_settings`, `save_settings`等）
- `src-tauri/src/services/translation.rs` - Ollama専用翻訳実装（`translate_with_ollama`）
- `src-tauri/src/services/settings.rs` - 設定管理（`AppSettings`構造体）
- `src-tauri/src/services/clipboard.rs` - クリップボード操作
- `src-tauri/src/services/shortcut.rs` - グローバルショートカット

**既存パターン:**
- `reqwest`でOllama HTTP APIを呼び出し
- `AppSettings`構造体: `shortcut`, `ollama_model`, `ollama_endpoint`のみ
- 非同期処理: `tokio`ランタイム使用
- エラーハンドリング: `thiserror`による型付きエラー

#### ディレクトリ不在

以下のディレクトリは存在せず、ステアリングドキュメントの記載と異なる：
- `src-tauri/src/commands/` （存在しない）
- `src-tauri/src/llm/` （存在しない）

現在は`src-tauri/src/services/`にビジネスロジックを配置し、`lib.rs`でコマンド定義を行うパターン。

### 1.2 重要な発見: Claude API実装の不在

**ステアリングドキュメントとの不一致:**
- `product.md`には「デュアルLLM対応: Ollama（ローカル）とClaude API（リモート）を切り替え可能」と記載
- しかし実装はOllama専用で、Claude API関連のコードは一切存在しない
- `AppSettings`にもClaude API設定（APIキー等）のフィールドがない

**影響:**
- Claude CLI追加は、実質的に最初のClaude統合となる
- 翻訳方式選択機能（provider切り替え）も未実装のため、これも新規実装が必要

---

## 2. 要件実現性分析

### 2.1 技術要件マッピング

| 要件 | 必要な技術要素 | 既存資産 | ギャップ |
|------|--------------|---------|---------|
| **Req 1: Claude CLI翻訳エンジン** | `std::process::Command`でCLI実行 | - | **Missing**: CLI実行ロジック全体 |
| **Req 2: 翻訳方式選択UI** | 設定画面でprovider選択（Ollama/Claude CLI） | `SettingsPanel.tsx` | **Missing**: provider選択UI、AppSettings拡張 |
| **Req 3: バックエンド統合（Rust）** | Tauriコマンド`translate_with_claude_cli` | `lib.rs`のコマンド定義パターン | **Missing**: 新コマンド実装 |
| **Req 4: フロントエンド統合（TS）** | `src/lib/claude-cli.ts`クライアント | `useTranslation` hook | **Missing**: 新クライアント、hook拡張 |
| **Req 5: CLI パス設定** | カスタムパス入力フィールド | `SettingsPanel`のInputField | **Missing**: AppSettings拡張、検証ロジック |
| **Req 6: プロンプトキャッシング** | `claude -p`引数構成 | - | **Research Needed**: Claude CLIのキャッシング仕様 |
| **Req 7: エラーハンドリング** | エラーメッセージUI表示 | `TranslationPopup`エラー表示 | **Extend**: Claude CLI固有エラー追加 |
| **Req 8: 互換性維持** | 既存Ollama翻訳フロー無修正 | 全ての既存実装 | **Constraint**: 既存コード変更最小化 |

### 2.2 制約と未知事項

**制約:**
- 既存のOllama翻訳フローを変更してはならない（Req 8）
- 既存テストは全てパスし続ける必要がある（Req 8）
- macOS専用アプリ（他OS対応不要）

**未知事項（Research Needed）:**
1. **Claude CLIのプロンプトキャッシング仕様**
   - `claude -p`コマンドの正確な引数フォーマット
   - キャッシュブロックの構成方法
   - システムプロンプトと可変プロンプトの分離方法
2. **Claude CLIの出力フォーマット**
   - 標準出力から翻訳結果のみを抽出する方法
   - エラーメッセージの形式（stderr）
   - exit codeの意味

### 2.3 実装複雑度シグナル

- **シンプルCRUD**: ❌（設定のCRUD自体は簡単だが、翻訳ロジック統合が必要）
- **アルゴリズムロジック**: ⚠️（CLI出力パース、プロンプト構成）
- **ワークフロー/状態管理**: ✅（provider選択、翻訳フロー分岐）
- **外部統合**: ✅（CLI実行、プロセス管理、タイムアウト処理）

---

## 3. 実装アプローチオプション

### Option A: 既存コンポーネント拡張

**適用範囲:**
- `AppSettings`構造体に`claude_cli_path`, `provider`フィールドを追加
- `useTranslation` hookでprovider分岐を追加
- `SettingsPanel`にClaude CLI設定UIを追加

**拡張対象ファイル:**
- `src-tauri/src/services/settings.rs` - AppSettings拡張
- `src/hooks/useTranslation.ts` - provider分岐ロジック追加
- `src/types/index.ts` - 型定義拡張（Provider型追加）
- `src/components/SettingsPanel.tsx` - UI拡張

**互換性評価:**
- ✅ 既存のデフォルト値でOllama動作を維持
- ✅ AppSettings拡張は下位互換（Option型で新フィールド追加）
- ✅ useTranslation内の分岐は既存ロジックに影響なし

**複雑度とメンテナンス性:**
- ⚠️ `useTranslation` hookが複雑化（3つの翻訳方式を管理）
- ✅ 既存パターンに沿うため学習コスト低い
- ⚠️ SettingsPanelのUIが混雑する可能性

**トレードオフ:**
- ✅ 最小限のファイル追加、既存パターンを踏襲
- ✅ 既存インフラ（tauri-plugin-store）をそのまま活用
- ❌ provider選択ロジックが複数ファイルに散在するリスク
- ❌ 将来的にproviderが増えた場合にリファクタリング必要

---

### Option B: 新規コンポーネント作成

**新規作成ファイル:**
- `src/lib/claude-cli.ts` - Claude CLIクライアント（`translate()`関数）
- `src-tauri/src/llm/claude_cli.rs` - Claude CLI実行サービス
- `src/components/ProviderSelector.tsx` - provider選択専用コンポーネント

**統合ポイント:**
- `useTranslation` hookから`src/lib/claude-cli.ts`を呼び出し
- `lib.rs`に新しいTauriコマンド`translate_with_claude_cli`を定義
- `SettingsPanel`から`ProviderSelector`をインポート

**責任境界:**
- `claude-cli.ts`: Tauri IPC呼び出し、エラーハンドリング
- `claude_cli.rs`: `std::process::Command`実行、出力パース、タイムアウト管理
- `ProviderSelector`: provider選択UI、説明テキスト表示

**トレードオフ:**
- ✅ 責任分離が明確、テスト容易性向上
- ✅ 既存コンポーネントへの影響最小化
- ✅ 将来的な拡張性（provider追加時に影響範囲が限定的）
- ❌ ファイル数増加、ナビゲーションコスト
- ❌ 新しいインターフェース設計が必要

---

### Option C: ハイブリッドアプローチ【推奨】

**組み合わせ戦略:**

**Phase 1: 新規コンポーネント作成（コア機能）**
- `src/lib/claude-cli.ts` - Claude CLIクライアント（新規）
- `src-tauri/src/llm/claude_cli.rs` - CLI実行サービス（新規、既存ollama.rsと同じディレクトリ）
- `src-tauri/src/lib.rs` - `translate_with_claude_cli`コマンド追加（拡張）

**Phase 2: 既存コンポーネント拡張（統合）**
- `AppSettings` - `provider`, `claude_cli_path`フィールド追加（拡張）
- `useTranslation` - provider分岐ロジック追加（拡張）
- `SettingsPanel` - provider選択UI追加（拡張）

**段階的実装:**
1. **最小限の実装**: Claude CLI実行のみ（provider選択なし、設定でハードコード）
2. **設定追加**: AppSettingsにClaude CLIパス設定を追加
3. **provider選択**: 3つの翻訳方式から選択可能に

**リスク軽減:**
- 増分ロールアウト: まず基本機能を実装し、段階的に統合
- 機能フラグ不要（provider選択で切り替え）
- ロールバック戦略: provider設定をOllamaに戻せば既存動作に復帰

**トレードオフ:**
- ✅ バランスの取れたアプローチ、柔軟性と実用性の両立
- ✅ 段階的な検証が可能、リスク低減
- ❌ 計画が複雑、フェーズ間の調整が必要
- ⚠️ フェーズ1完了時は未完成状態（ユーザーに公開不可）

---

## 4. 実装複雑度とリスク評価

### 工数見積もり: **M（3-7日）**

**内訳:**
- Claude CLI実行ロジック実装（Rust）: 1-2日
  - `std::process::Command`非同期実行
  - 標準出力/エラー出力のパース
  - タイムアウト処理
- フロントエンド統合: 1-2日
  - `claude-cli.ts`クライアント実装
  - `useTranslation` hook拡張
  - 型定義追加
- 設定管理拡張: 1日
  - `AppSettings`拡張（provider, claude_cli_path）
  - 設定UI追加（SettingsPanel）
- プロンプトキャッシング最適化: 1-2日
  - Claude CLI仕様調査
  - プロンプト構造設計
  - キャッシング検証
- テスト・デバッグ: 1日

**理由:**
- 既存パターンが明確で、拡張ポイントが特定済み
- Rust/TypeScript両方の実装が必要だが、それぞれ中規模
- プロンプトキャッシングの仕様調査が不確定要素

### リスク: **Medium**

**リスク要因:**
- **Claude CLI仕様の不確実性**（Medium）
  - プロンプトキャッシングの正確な使い方が不明
  - 出力フォーマットが想定と異なる可能性
  - **軽減策**: 早期に小規模な検証スクリプトで仕様確認
- **provider分岐の複雑化**（Low）
  - 3つの翻訳方式を管理する必要がある
  - **軽減策**: Strategy パターンで分岐ロジックを抽象化
- **既存機能への影響**（Low）
  - Ollama翻訳フローに影響を与えないこと
  - **軽減策**: 既存テストを全て実行し、回帰がないことを確認
- **パフォーマンス懸念**（Low）
  - CLI起動オーバーヘッドが翻訳速度に影響する可能性
  - **軽減策**: タイムアウト設定（30秒）で最悪ケースを制限

---

## 5. 設計フェーズへの推奨事項

### 優先アプローチ

**Option C（ハイブリッド）** を推奨します。

**理由:**
1. 新規コアロジックを独立ファイルに分離し、テスト容易性を確保
2. 既存コンポーネントへの最小限の拡張で統合を実現
3. 段階的実装により、各フェーズでの検証が可能

### 重要な設計決定事項

設計フェーズで以下を明確化する必要があります：

1. **Provider抽象化設計**
   - provider選択ロジックをどこに配置するか（hookレベル vs serviceレイヤー）
   - Strategy パターンの適用範囲
2. **AppSettings構造**
   - `provider: 'ollama' | 'claude-cli'`の型定義
   - `claude_cli_path?: string`の検証ルール
3. **エラーハンドリング戦略**
   - Claude CLI固有エラー（コマンド不在、タイムアウト等）の型定義
   - ユーザー向けエラーメッセージの文言
4. **プロンプトキャッシング実装**
   - システムプロンプトと可変プロンプトの分離方法
   - `claude -p`コマンドの正確な引数構成

### 研究項目（設計フェーズで調査）

1. **Claude CLI仕様の詳細調査**
   - 公式ドキュメント確認
   - `claude -p --help`で引数仕様を確認
   - サンプルプロンプトで動作検証
2. **プロンプトキャッシングのベストプラクティス**
   - Anthropic公式のキャッシング推奨パターン
   - 翻訳タスクに適したプロンプト構造
3. **Rustプロセス管理のベストプラクティス**
   - `tokio::process::Command`の非同期実行パターン
   - 子プロセスのリソースリーク防止策

---

## 6. まとめ

### 実装可能性: ✅ **高い**

- 既存アーキテクチャが明確で拡張ポイントが特定済み
- 必要な技術スタック（Rust, TypeScript, Tauri IPC）は既に使用中
- 段階的実装により、リスクを管理しながら進められる

### 注意事項

1. **ステアリングドキュメントの更新**
   - Claude API実装が存在しないため、現状は「Ollama専用アプリ」
   - Claude CLI追加後も、`product.md`の「デュアルLLM」記載を「Ollama & Claude CLI」に修正すべき
2. **将来的なClaude API実装**
   - もし将来Claude API（HTTP経由）も追加する場合、provider選択を3つ→4つに拡張
   - その場合は、現在のハイブリッドアプローチが拡張性を提供

### 次のステップ

設計フェーズ（`/kiro:spec-design claude-cli-translation`）で以下を実施：
1. Provider抽象化の詳細設計
2. Claude CLI仕様の調査と検証
3. ファイル構造とインターフェース設計
4. プロンプトキャッシングの実装戦略
