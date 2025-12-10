# Research & Design Decisions

## Summary
- **Feature**: `translation-quality-improvement`
- **Discovery Scope**: Extension（既存システムの拡張）
- **Key Findings**:
  - Claude 4.x向けプロンプトエンジニアリングのベストプラクティスを確認
  - 既存のOllama翻訳パターン（`build_plamo_prompt`, `build_general_prompt`）を参考にClaude CLI向けプロンプト構築関数を追加
  - システムプロンプトの構造化（役割定義、翻訳ルール、品質ガイドライン）により品質向上を実現

## Research Log

### Claude 4.x Prompt Engineering Best Practices
- **Context**: 翻訳品質を50/100から80/100以上に向上させるため、最新のプロンプトエンジニアリング手法を調査
- **Sources Consulted**:
  - [Claude 4 prompt engineering best practices](https://docs.claude.com/en/docs/build-with-claude/prompt-engineering/claude-4-best-practices)
  - [System prompts - Anthropic](https://docs.anthropic.com/claude/docs/system-prompts)
  - [Prompt engineering overview](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/overview)
- **Findings**:
  - **Be Clear and Explicit**: Claude 4.xは明確で具体的な指示に強く反応する。「技術翻訳者として動作する」という役割定義を明示
  - **Provide Context**: 指示の背後にある動機や理由を説明することで、Claude 4.xがより的を絞った応答を提供
  - **XML Tags for Structure**: XML形式のタグを使用してプロンプトをセクション分割し、明確性を向上（ただし、今回はシンプルなテキスト構造を採用）
  - **Role-based Framing**: 「プロフェッショナルな技術翻訳者」という役割を設定し、技術的専門性を維持
  - **Specificity**: 専門用語の保持、コードスニペットの非翻訳化など、具体的なルールを列挙
- **Implications**:
  - システムプロンプトを3セクション構成（役割定義、翻訳ルール、品質ガイドライン）に構造化
  - 技術文書特化型のルール（プログラミング用語保持、API構造保持）を明示的に記述
  - 言語方向（英→日、日→英）で異なる指示（「です・ます調」優先等）を条件分岐

### System Prompt Structure Recommendations
- **Context**: システムプロンプトの最適な構造を調査
- **Sources Consulted**:
  - [System prompts - Anthropic](https://docs.anthropic.com/claude/docs/system-prompts)
  - [The 10-Step Prompt Structure Guide](https://aimaker.substack.com/p/the-10-step-system-prompt-structure-guide-anthropic-claude)
- **Findings**:
  - **推奨される順序**: Task context (役割定義) → Tone context (トーン設定) → Background data (背景情報) → Detailed task description and rules (詳細ルール)
  - **System vs User Messages**: システムメッセージは役割設定に最適、詳細指示はユーザーメッセージに配置（ただし、Claude CLIでは`--system-prompt`引数を使用）
  - **Clarity and Directness**: Claudeは文脈を持たない新入社員のように扱い、明示的な指示を提供
- **Implications**:
  - システムプロンプトで役割定義、翻訳ルール、品質基準を統合
  - `claude -p`コマンドの`--system-prompt`引数で構造化されたプロンプトを渡す
  - プロンプト長を2000トークン以内に制限（プロンプトキャッシング効率化）

### Existing Prompt Patterns in Codebase
- **Context**: Ollama翻訳の既存パターンを分析し、Claude CLI向けに応用可能なパターンを特定
- **Sources Consulted**:
  - `src-tauri/src/services/translation.rs` (lines 210-269)
  - `src-tauri/src/llm/claude_cli.rs` (lines 39-43)
- **Findings**:
  - **Ollama翻訳パターン**:
    - `build_plamo_prompt()`: 翻訳特化モデル向けシンプルプロンプト
    - `build_general_prompt()`: 汎用LLM向けシンプルプロンプト
    - `build_translation_prompt()`: モデルタイプで分岐し、言語方向に応じたプロンプトを生成
    - パターン: `format!()`マクロで動的にプロンプト構築、言語方向で条件分岐
  - **現在のClaude CLIプロンプト**:
    - 1行のシンプルなプロンプト（39-43行目）
    - 役割定義のみで、技術文書特化型のルールなし
- **Implications**:
  - Ollama翻訳と同様のパターンを採用: `build_system_prompt(source_lang, target_lang)`関数を追加
  - 言語方向に応じて異なるルール（英→日では「です・ます調」優先、日→英では能動態優先）を適用
  - 既存の`translate_with_claude_cli`関数の構造は変更せず、プロンプト構築ロジックのみ拡張

### Prompt Caching Optimization
- **Context**: Claude CLIのプロンプトキャッシング機能を活用し、実行効率を向上
- **Sources Consulted**:
  - gap-analysis.md (プロンプトキャッシングの制約)
  - requirements.md (Requirement 1.3, 7.3)
- **Findings**:
  - プロンプトキャッシングは`--system-prompt`引数で自動的に有効化
  - 固定部分（翻訳ルール、品質ガイドライン）を最大化し、可変部分（翻訳対象テキスト）を最小化
  - 2000トークン以内の制限により、プロンプトキャッシング効率を維持
- **Implications**:
  - 役割定義、翻訳ルール、品質ガイドラインをシステムプロンプトに配置（キャッシュ対象）
  - 翻訳対象テキストは`claude -p`の最後の引数として渡す（非キャッシュ部分）
  - プロンプト長を2000トークン以内に制限し、キャッシングの効率を最大化

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Option A: Extend Existing Components | `claude_cli.rs`の`translate_with_claude_cli`関数内でシステムプロンプト構築ロジックを拡張 | - 最小限の変更（1ファイル内で完結）<br>- 既存パターンを踏襲<br>- 高速な実装（2-5日） | - プロンプトが長くなる（+100-200行）<br>- ファイルサイズ増加（170行 → 300-400行） | **推奨**: 既存のOllama翻訳パターンと一貫性を保ち、保守性を維持 |
| Option B: Create New Components | プロンプトテンプレートを独立したモジュール（`prompt_templates.rs`）に分離 | - プロンプトテンプレートの再利用性向上<br>- `claude_cli.rs`のファイルサイズ抑制 | - 新しいファイル作成（+1ファイル）<br>- やや複雑なアーキテクチャ<br>- 実装時間増加（+1-2日） | 現時点では過剰設計の可能性あり |

## Design Decisions

### Decision: 3セクション構成のシステムプロンプト
- **Context**: 翻訳品質を向上させるため、システムプロンプトを構造化する必要がある
- **Alternatives Considered**:
  1. **シンプルな1行プロンプト（現状）**: 役割定義のみで、技術文書特化型のルールなし → 品質不足
  2. **3セクション構成**: 役割定義、翻訳ルール、品質ガイドラインを分割 → **採用**
  3. **XMLタグ構造**: `<role>`, `<rules>`, `<guidelines>`で明示的に分割 → 複雑性増加、2000トークン制限に影響
- **Selected Approach**: 3セクション構成のテキストベースプロンプト
  - **セクション1: 役割定義** - 「あなたはプロフェッショナルな技術翻訳者です」
  - **セクション2: 翻訳ルール** - 技術文書特化型のルール（プログラミング用語保持、コードスニペット非翻訳化、専門用語辞書）
  - **セクション3: 品質ガイドライン** - コンテキスト保持、自然な表現、技術的正確性
- **Rationale**:
  - Claude 4.xのベストプラクティスに従い、明確で構造化された指示を提供
  - シンプルなテキスト構造でXMLタグの複雑性を回避し、2000トークン以内に収める
  - 既存のOllama翻訳パターンと一貫性を保つ
- **Trade-offs**:
  - ✅ 品質向上（明確な指示により翻訳精度が向上）
  - ✅ 保守性（セクション分割により各部分の役割が明確）
  - ❌ プロンプト長増加（+100-200行） → 許容範囲内
- **Follow-up**: テストケースで品質スコア80/100以上を検証

### Decision: 言語方向別のルール分岐
- **Context**: 英→日と日→英で異なる表現ガイドライン（「です・ます調」vs 能動態優先）を適用する必要がある
- **Alternatives Considered**:
  1. **共通ルールのみ**: 言語方向に関係なく同じルールを適用 → 自然な表現が損なわれる
  2. **言語方向別にルールを分岐** → **採用**
- **Selected Approach**: `match (source_lang, target_lang)`で条件分岐し、英→日と日→英で異なるルールセクションを構築
- **Rationale**:
  - 日本語と英語では自然な表現の基準が異なる（「です・ます調」vs 能動態）
  - 既存のOllama翻訳パターン（`build_plamo_prompt`, `build_general_prompt`）と一貫性を保つ
- **Trade-offs**:
  - ✅ 自然な表現向上（言語固有のガイドラインを適用）
  - ❌ プロンプト長増加（+50-100行） → 許容範囲内
- **Follow-up**: テストケースで自然な表現（25点/25点）を評価

### Decision: 専門用語辞書の埋め込み
- **Context**: 頻出する技術用語（API, framework, library等）の訳語を一貫させる必要がある
- **Alternatives Considered**:
  1. **動的な辞書管理**: JSONファイルや設定ファイルで専門用語リストを管理 → Requirement 7.5（コード埋め込み方針）に反する
  2. **プロンプト内に辞書を埋め込み** → **採用**
- **Selected Approach**: 翻訳ルールセクション内に10種類の基本用語（API, framework, library, module, function, variable, interface, class, object, array）を列挙
- **Rationale**:
  - Requirement 7.5に従い、システムプロンプトの内容をコード内に直接埋め込む
  - プロンプトキャッシングにより、辞書部分がキャッシュされ効率的
- **Trade-offs**:
  - ✅ バージョン管理が容易（Gitで辞書の変更履歴を追跡）
  - ✅ 設定ファイル管理の複雑性を回避
  - ❌ 辞書の拡張にはコード変更が必要 → 基本10用語で十分（将来的に拡張可能）
- **Follow-up**: 専門用語の正確性（30点/30点）を評価

### Decision: Option A（既存コンポーネント拡張）を採用
- **Context**: プロンプトテンプレート管理の実装アプローチを決定
- **Alternatives Considered**:
  1. **Option A: Extend Existing Components** → **採用**
  2. **Option B: Create New Components** → 現時点では過剰設計
- **Selected Approach**: `claude_cli.rs`の`translate_with_claude_cli`関数内でシステムプロンプト構築ロジックを拡張
- **Rationale**:
  - 最小限の変更（1ファイル内で完結）
  - 既存のOllama翻訳パターンと一貫性を保つ
  - 高速な実装（2-5日）
  - 低リスク（既存機能への影響なし）
- **Trade-offs**:
  - ✅ シンプルで保守しやすい
  - ✅ 既存パターンを踏襲
  - ❌ ファイルサイズ増加（170行 → 300-400行） → 許容範囲内
- **Follow-up**: MVP実装後、必要に応じてOption Bにリファクタリング（Phase 2）

## Risks & Mitigations

- **Risk 1: プロンプト長超過（2000トークン制限）** — プロンプトテンプレートを設計フェーズで検証し、2000トークン以内に収める
- **Risk 2: 翻訳品質目標未達（80/100未満）** — 30サンプルのテストケースで品質スコアを測定し、必要に応じてプロンプト調整
- **Risk 3: プロンプトキャッシング効率低下** — 固定部分（翻訳ルール、ガイドライン）を最大化し、可変部分（翻訳対象テキスト）を最小化

## References
- [Claude 4 prompt engineering best practices](https://docs.claude.com/en/docs/build-with-claude/prompt-engineering/claude-4-best-practices) — Claude 4.x向けプロンプトエンジニアリングのベストプラクティス
- [System prompts - Anthropic](https://docs.anthropic.com/claude/docs/system-prompts) — システムプロンプトの推奨構造と使用方法
- [Prompt engineering overview](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/overview) — プロンプトエンジニアリングの概要
- [Prompt engineering best practices | Claude](https://www.claude.com/blog/best-practices-for-prompt-engineering) — 公式ブログのベストプラクティス
- [Be clear, direct, and detailed - Claude Docs](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/be-clear-and-direct) — 明確で直接的な指示の重要性
