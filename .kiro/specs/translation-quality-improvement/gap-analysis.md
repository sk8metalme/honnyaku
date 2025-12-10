# Implementation Gap Analysis: translation-quality-improvement

## 分析サマリー

**スコープ**: Claude CLI翻訳のシステムプロンプトを最適化し、翻訳品質を50/100から80/100以上に改善

**主な課題**:
- 現在のシステムプロンプトは1行のシンプルな指示のみ（39-43行目）
- 技術文書特化型の翻訳ルールが欠如
- 専門用語辞書や自然な表現ガイドラインが未実装
- 翻訳品質の測定・検証メカニズムが存在しない

**推奨アプローチ**: Option A（既存コンポーネント拡張） - `claude_cli.rs`のシステムプロンプト構築ロジックを拡張し、構造化されたプロンプトテンプレートを実装

**工数見積**: S-M（2-5日） - 既存パターンを踏襲し、プロンプト文字列を拡張するシンプルな変更

**リスク**: Low - 既存のCLI実行フローは変更せず、プロンプト内容のみ改善。プロンプトキャッシングは既に実装済み。

---

## 1. Current State Investigation

### 1.1 既存資産の分析

#### 主要ファイル
| ファイル | 役割 | 関連要件 | 現状 |
|---------|------|---------|------|
| `src-tauri/src/llm/claude_cli.rs` | Claude CLI翻訳エンジン | Req 1, 2, 3, 4, 5, 7 | **シンプルなプロンプト構築（39-43行目）** - 1行の基本的な指示のみ |
| `src-tauri/src/services/translation.rs` | Ollama翻訳エンジン、共通型定義 | Req 6（参考） | 複数のプロンプト構築関数（`build_plamo_prompt`, `build_general_prompt`等）を持つが、技術文書特化型ではない |
| `src-tauri/src/llm/mod.rs` | LLMモジュール定義 | - | `claude_cli`モジュールをエクスポート |
| `src-tauri/src/lib.rs` | Tauri IPCコマンド定義 | Req 7 | `translate_with_claude_cli`コマンドを登録済み |

#### 現在のシステムプロンプト実装（claude_cli.rs:39-43）
```rust
let system_prompt = format!(
    "You are a professional translator. Translate the following text from {} to {} while preserving meaning, tone, and context.",
    source_lang.name(),
    target_lang.name()
);
```

**課題**:
- **極めてシンプル**: 役割定義のみで、翻訳ルールや品質基準が欠如
- **技術文書非対応**: プログラミング用語、コードスニペット、API構造への配慮なし
- **専門用語辞書なし**: 一貫した訳語選択のガイドラインがない
- **自然な表現ガイドラインなし**: 「です・ます調」優先等の指示がない

#### 既存のプロンプト構築パターン（Ollama翻訳の参考）
`translation.rs`では、モデルタイプに応じて複数のプロンプト構築関数を使用:
- `build_plamo_prompt()`: PLaMo-2-Translate用のシンプルなプロンプト
- `build_general_prompt()`: 汎用LLM用のシンプルなプロンプト
- `build_translation_prompt()`: 上記を統合し、モデルタイプで分岐

**パターンの特徴**:
- 言語方向（日→英、英→日）で異なるプロンプトを使用
- `format!()`マクロで動的にプロンプトを構築
- シンプルで保守しやすい構造

### 1.2 アーキテクチャパターンと制約

**Claude CLI実行フロー**:
1. システムプロンプトを構築（`format!()`）
2. `tokio::process::Command`でClaude CLI実行
3. `--system-prompt`引数でプロンプトを渡す
4. `--output-format json`でJSON出力を取得
5. 30秒タイムアウト、`kill_on_drop(true)`でリソース管理

**制約**:
- **プロンプト長制限**: Claude CLIの制限はないが、プロンプトキャッシングの効率を考慮して2000トークン以内が推奨（Req 7.3）
- **コード埋め込み**: システムプロンプトはコード内に直接埋め込む方針（Req 7.5） - 設定ファイル管理は避ける
- **既存フロー維持**: タイムアウト、エラーハンドリング等の既存ロジックは変更しない（Req 7.6, 8.1）

### 1.3 統合ポイント

| 統合ポイント | 現状 | 必要な変更 |
|------------|------|----------|
| システムプロンプト構築 | 1行のシンプルな文字列 | 構造化されたマルチセクションプロンプトに拡張 |
| 言語方向対応 | `source_lang.name()`, `target_lang.name()` | 英→日、日→英で異なる指示（「です・ます調」等）を追加 |
| プロンプトキャッシング | `--system-prompt`引数で実装済み | キャッシング効率を維持（固定部分を最大化） |
| エラーハンドリング | CLI実行エラー、タイムアウト | **変更不要** - 既存ロジックをそのまま使用 |
| テスト | 基本的な統合テスト（#[ignore]付き） | 翻訳品質テストケースを追加（Req 6） |

---

## 2. Requirements Feasibility Analysis

### 2.1 技術的ニーズの抽出

| 要件 | 技術的ニーズ | 既存資産 | ギャップ |
|------|------------|---------|---------|
| **Req 1: システムプロンプト構造最適化** | 役割定義、翻訳ルール、品質ガイドラインの3セクション構成 | 1行のシンプルなプロンプト | **Missing**: 構造化されたプロンプトテンプレート |
| **Req 2: 技術文書特化型翻訳ルール** | プログラミング用語保持、コードスニペット非翻訳化、API構造保持の指示 | なし | **Missing**: 技術文書特化型のルール定義 |
| **Req 3: コンテキスト保持機能** | 文脈考慮、代名詞・接続詞翻訳、一貫した訳語使用の指示 | 基本的な"preserving meaning, tone, and context"のみ | **Missing**: 具体的なコンテキスト保持の指示 |
| **Req 4: 専門用語辞書統合** | 10種類の基本用語（API, framework, library等）の訳語ガイドライン | なし | **Missing**: 専門用語辞書リスト |
| **Req 5: 自然な表現変換ガイドライン** | 「です・ます調」優先、能動態/受動態、簡潔な表現の指示 | なし | **Missing**: 自然な表現ガイドライン |
| **Req 6: 翻訳品質測定・検証** | 30サンプルのテストケース、5項目評価基準（専門用語30点、文脈25点、表現25点、構造10点、技術10点） | テストケースなし | **Missing**: 品質評価テストスイート |
| **Req 7: システムプロンプト実装・統合** | `claude_cli.rs`へのプロンプト統合、2000トークン以内 | 基本的なCLI実行フローは実装済み | **Gap**: プロンプト拡張のみ必要 |
| **Req 8: 既存機能互換性維持** | Ollama翻訳、Claude CLI翻訳フローへの影響回避 | 独立したCLI実行フロー | **Satisfied**: 既存フローは独立 |
| **Req 9: ドキュメントとテスト整備** | プロンプト構造説明、用語リスト、品質評価基準のドキュメント化 | なし | **Missing**: ドキュメント作成 |

### 2.2 ギャップと制約の特定

#### Missing Capabilities（欠落機能）
1. **構造化されたシステムプロンプトテンプレート**:
   - 役割定義セクション
   - 翻訳ルールセクション（技術文書特化型）
   - 品質ガイドラインセクション

2. **専門用語辞書**:
   - 基本10用語の訳語リスト
   - プログラミング言語、クラウドサービス名の保持ルール

3. **翻訳品質評価システム**:
   - 30サンプルのテストケースセット
   - 5項目評価基準の実装

#### Constraints（制約）
1. **プロンプト長**: 2000トークン以内（プロンプトキャッシング効率のため）
2. **コード埋め込み**: 設定ファイル管理を避け、Rustコード内に直接埋め込む
3. **既存フロー維持**: `translate_with_claude_cli`関数の構造を変更しない

#### Research Needed（要調査事項）
1. **プロンプトエンジニアリング最適化**:
   - Claude APIの最新プロンプトエンジニアリングベストプラクティスを調査
   - 技術翻訳に特化したプロンプトパターンの研究
   - **Note**: 設計フェーズで外部リソース（Claude公式ドキュメント、プロンプトエンジニアリングガイド）を参照

2. **翻訳品質評価メトリクス**:
   - 専門用語の正確性を定量的に評価する方法
   - 自然な表現を客観的に測定する基準
   - **Note**: 設計フェーズでBLEUスコア、人間評価等の手法を調査

### 2.3 複雑性シグナル

**実装の性質**: **Simple - 文字列テンプレート拡張**
- プロンプトテンプレートの構築（複雑なアルゴリズムなし）
- 既存のCLI実行フローを変更せず、プロンプト内容のみ改善
- テストケースの追加（品質評価の自動化）

**統合の複雑性**: **Low - 単一ファイル内で完結**
- `claude_cli.rs`の`system_prompt`構築ロジックを拡張するのみ
- 外部依存なし（Claude CLIは既に統合済み）

**非機能要件**: **Medium - 翻訳品質の測定が必要**
- 品質スコア80/100以上の目標達成を客観的に測定
- テストケースセットの準備と評価基準の実装

---

## 3. Implementation Approach Options

### Option A: Extend Existing Components（推奨）

**Which files/modules to extend**:
- **`src-tauri/src/llm/claude_cli.rs`**: システムプロンプト構築ロジックを拡張
  - 現在: 1行のシンプルなプロンプト（39-43行目）
  - 変更: 構造化されたマルチセクションプロンプトテンプレートに置き換え
  - 影響範囲: `translate_with_claude_cli`関数内のプロンプト構築部分のみ

**Compatibility assessment**:
- ✅ **既存インターフェース維持**: 関数シグネチャは変更なし
- ✅ **既存テストパス**: プロンプト内容の変更のみで、CLI実行フローは不変
- ✅ **後方互換性**: ユーザー向けAPIは変更なし、自動的に改善されたプロンプトを使用

**Complexity and maintainability**:
- ✅ **低い複雑性**: プロンプト文字列の拡張のみ、新しいロジック追加なし
- ✅ **単一責任維持**: `translate_with_claude_cli`はClaude CLI実行のみを担当（変更なし）
- ✅ **ファイルサイズ管理可能**: プロンプトテンプレートを複数行の文字列リテラルとして定義（+100-200行程度）

**Implementation details**:
```rust
// 現在のシンプルなプロンプト
let system_prompt = format!(
    "You are a professional translator. Translate the following text from {} to {} while preserving meaning, tone, and context.",
    source_lang.name(),
    target_lang.name()
);

// ↓ 拡張後のプロンプト（3セクション構成）
let system_prompt = build_system_prompt(source_lang, target_lang);

fn build_system_prompt(source_lang: Language, target_lang: Language) -> String {
    let role_section = "You are a professional technical translator...";
    let rules_section = match (source_lang, target_lang) {
        (Language::English, Language::Japanese) => {
            // 英→日の技術文書翻訳ルール
            "..."
        },
        (Language::Japanese, Language::English) => {
            // 日→英の技術文書翻訳ルール
            "..."
        },
        _ => "...",
    };
    let quality_guidelines = "...";

    format!("{}\n\n{}\n\n{}", role_section, rules_section, quality_guidelines)
}
```

**Trade-offs**:
- ✅ **Pros**:
  - 最小限の変更（1ファイル内で完結）
  - 既存パターンを踏襲（Ollama翻訳の`build_*_prompt`関数と同様）
  - 高速な実装（2-3日）
  - テスト容易性（既存テストを維持しつつ、品質テストを追加）

- ❌ **Cons**:
  - プロンプトが長くなる（+100-200行） - ただし、コード埋め込み方針により管理可能
  - ファイルサイズ増加（170行 → 300-400行程度） - 許容範囲内

**Estimated Effort**: **S-M (2-5日)**
- Day 1-2: プロンプトテンプレート設計と実装
- Day 3-4: テストケース作成と品質評価
- Day 5: ドキュメント整備

**Risk**: **Low**
- 既存のCLI実行フローは変更なし
- プロンプト内容の変更のみで、システムへの影響は最小限
- プロンプトキャッシングは既に実装済み、効率維持

---

### Option B: Create New Components

**When to consider**: プロンプトテンプレート管理が複雑化し、独立したモジュールが必要な場合

**Rationale for new creation**:
- プロンプトテンプレートを独立したファイルに分離（例: `src-tauri/src/llm/prompt_templates.rs`）
- 複数の翻訳方式（Ollama, Claude CLI）で共通のプロンプトパターンを再利用

**Integration points**:
- `claude_cli.rs`から`prompt_templates::build_claude_cli_system_prompt()`を呼び出し
- Ollama翻訳でも同じテンプレートモジュールを使用可能

**Trade-offs**:
- ✅ **Pros**:
  - プロンプトテンプレートの再利用性向上
  - `claude_cli.rs`のファイルサイズを抑制
  - 将来的に複数のプロンプトバリエーションを管理しやすい

- ❌ **Cons**:
  - 新しいファイル作成（+1ファイル）
  - やや複雑なアーキテクチャ（現時点では過剰設計の可能性）
  - 実装時間増加（+1-2日）

**Estimated Effort**: **M (4-7日)**

**Risk**: **Low-Medium**
- 新しいモジュールの設計が必要
- 既存コードへの影響は最小限

**Note**: 現時点ではClaude CLI翻訳のみがシステムプロンプトを使用するため、Option Aの方が適切

---

### Option C: Hybrid Approach

**When to consider**: 将来的にプロンプトテンプレートの管理が複雑化する可能性がある場合

**Combination strategy**:
1. **Phase 1 (MVP)**: Option Aで実装 - `claude_cli.rs`内でプロンプトテンプレートを構築
2. **Phase 2 (Refactoring)**: 必要に応じてOption Bにリファクタリング - プロンプトテンプレートを独立モジュールに分離

**Trade-offs**:
- ✅ **Pros**:
  - 段階的な実装で、初期リスクを低減
  - MVP（Phase 1）で品質改善効果を早期に検証
  - 必要に応じてリファクタリング（Phase 2）を実施

- ❌ **Cons**:
  - リファクタリングコスト（Phase 2が必要な場合）

**Estimated Effort**: **S-M (Phase 1: 2-5日) + オプションでM (Phase 2: 2-3日)**

**Risk**: **Low**

**Note**: Phase 1でMVPを実装し、翻訳品質の向上を検証した後、必要に応じてPhase 2に進む

---

## 4. Requirement-to-Asset Map

| 要件 | 既存資産 | ギャップ | 実装アプローチ |
|------|---------|---------|--------------|
| **Req 1: システムプロンプト構造最適化** | `claude_cli.rs` (39-43行目) | **Missing**: 3セクション構成のプロンプト | Option A: `build_system_prompt()`関数を追加 |
| **Req 2: 技術文書特化型翻訳ルール** | なし | **Missing**: 技術文書ルール定義 | Option A: `rules_section`にルールを追加 |
| **Req 3: コンテキスト保持機能** | 基本的な指示のみ | **Missing**: 具体的なコンテキスト保持指示 | Option A: `quality_guidelines`に指示を追加 |
| **Req 4: 専門用語辞書統合** | なし | **Missing**: 10用語の訳語リスト | Option A: `rules_section`に辞書を埋め込み |
| **Req 5: 自然な表現変換ガイドライン** | なし | **Missing**: 表現ガイドライン | Option A: 言語方向別に`rules_section`を分岐 |
| **Req 6: 翻訳品質測定・検証** | 基本テストのみ | **Missing**: 品質評価テストスイート | 新規: テストケースファイル作成（例: `src-tauri/tests/translation_quality.rs`） |
| **Req 7: システムプロンプト実装・統合** | CLI実行フロー実装済み | **Gap**: プロンプト拡張のみ | Option A: プロンプト構築ロジック拡張 |
| **Req 8: 既存機能互換性維持** | 独立したCLI実行フロー | **Satisfied**: 影響なし | **変更不要** |
| **Req 9: ドキュメントとテスト整備** | なし | **Missing**: ドキュメント | 新規: ドキュメントファイル作成（例: `docs/translation-prompt-structure.md`） |

---

## 5. Implementation Complexity & Risk

### Effort Estimation

**Overall Effort**: **S-M (2-5日)**

**Breakdown**:
- **Day 1-2**: プロンプトテンプレート設計と実装
  - 役割定義セクション作成
  - 技術文書特化型翻訳ルール作成
  - 専門用語辞書（10用語）統合
  - 自然な表現ガイドライン追加
  - 品質ガイドラインセクション作成

- **Day 3-4**: テストケース作成と品質評価
  - 30サンプルのテストケースセット準備
  - 5項目評価基準の実装
  - 最適化前後の品質スコア比較

- **Day 5**: ドキュメント整備とレビュー
  - プロンプト構造説明ドキュメント作成
  - 専門用語リストドキュメント化
  - 品質評価基準のドキュメント化

**Justification**:
- **S (1-3日)** ではない理由: テストケース準備と品質評価が必要
- **M (3-7日)** を超えない理由: 既存パターンを踏襲し、プロンプト文字列の拡張のみ
- **L (1-2週間)** ではない理由: 新しいアーキテクチャ変更や外部統合が不要

### Risk Assessment

**Overall Risk**: **Low**

**Justification**:
- **既知の技術スタック**: Rustの`format!()`マクロ、既存のCLI実行パターンを使用
- **明確なスコープ**: プロンプト内容の改善のみ、システムアーキテクチャは変更なし
- **独立した変更**: Ollama翻訳や他の機能への影響なし
- **既存パフォーマンス維持**: プロンプトキャッシングは既に実装済み

**Potential Risks**:
1. **プロンプト長超過** (Low Risk):
   - 対策: 2000トークン以内に制限（設計フェーズで検証）

2. **翻訳品質目標未達** (Medium Risk):
   - 対策: テストケースで品質スコア80/100以上を検証、必要に応じてプロンプト調整

3. **プロンプトキャッシング効率低下** (Low Risk):
   - 対策: 固定部分（翻訳ルール、ガイドライン）を最大化し、可変部分（翻訳対象テキスト）を最小化

---

## 6. Recommendations for Design Phase

### Preferred Approach

**Option A: Extend Existing Components（既存コンポーネント拡張）**

**理由**:
- ✅ 最小限の変更でMVPを実装可能（2-5日）
- ✅ 既存パターンを踏襲し、保守性を維持
- ✅ 低リスクで、既存機能への影響なし
- ✅ プロンプトキャッシングの効率を維持

### Key Decisions for Design Phase

1. **プロンプトテンプレート構造**:
   - 3セクション構成（役割定義、翻訳ルール、品質ガイドライン）を採用
   - 言語方向（英→日、日→英）で異なるルールを適用
   - 専門用語辞書を翻訳ルールセクションに埋め込み

2. **品質評価基準**:
   - 5項目評価基準（専門用語30点、文脈25点、表現25点、構造10点、技術10点）を定義
   - 30サンプルのテストケースセット（API doc 10, エラーメッセージ5, 技術ブログ10, コードコメント5）を準備

3. **実装優先度**:
   - **Phase 1 (MVP)**: 基本的なプロンプトテンプレート実装と品質評価（2-5日）
   - **Phase 2 (Optional)**: 必要に応じてプロンプトテンプレートを独立モジュールに分離（+2-3日）

### Research Items to Carry Forward

1. **プロンプトエンジニアリング最適化**:
   - Claude APIの公式ドキュメントを参照し、最新のベストプラクティスを調査
   - 技術翻訳に特化したプロンプトパターン（例: Chain-of-Thought、Few-Shot Examples）を研究
   - **リソース**: [Anthropic Prompt Engineering Guide](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering)

2. **翻訳品質評価メトリクス**:
   - BLEUスコア、METEOR等の自動評価メトリクスを調査
   - 人間評価（MQM: Multidimensional Quality Metrics）の活用を検討
   - **リソース**: 機械翻訳品質評価の研究論文、業界標準

3. **専門用語辞書の拡張**:
   - 基本10用語から、より包括的な技術用語リスト（50-100用語）への拡張を検討
   - 動的な用語辞書管理（将来的な機能）の可能性を調査

---

## Conclusion

translation-quality-improvementの実装は、**既存のClaude CLI翻訳エンジン（`claude_cli.rs`）のシステムプロンプト構築ロジックを拡張する**ことで達成可能です。現在のシンプルな1行プロンプトを、構造化された3セクション構成のプロンプトテンプレートに置き換えることで、翻訳品質を50/100から80/100以上に改善します。

**実装アプローチ**: Option A（既存コンポーネント拡張）を推奨し、工数2-5日、リスクLowで実装可能です。設計フェーズでは、プロンプトテンプレートの詳細設計、品質評価基準の定義、テストケースセットの準備を実施します。

**次のステップ**: 設計フェーズ（`/kiro:spec-design translation-quality-improvement`）に進み、プロンプトテンプレートの具体的な構造を設計します。
