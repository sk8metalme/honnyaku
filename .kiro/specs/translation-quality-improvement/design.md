# Design Document: translation-quality-improvement

## Overview

この機能は、Honnyaku翻訳アプリケーションのClaude CLI翻訳機能において、システムプロンプトを最適化することで翻訳品質を向上させます。現在の翻訳品質スコア50/100を80/100以上に改善することを目標とし、技術文書に特化した翻訳ルール、コンテキスト保持機能、専門用語辞書の活用、自然な表現変換ガイドラインを導入します。

**Users**: 開発者、技術ドキュメント翻訳作業者が、APIドキュメント、エラーメッセージ、技術ブログ記事、コードコメントを翻訳する際に利用します。

**Impact**: 既存のClaude CLI翻訳機能（`src-tauri/src/llm/claude_cli.rs`）のシステムプロンプト構築ロジックを拡張し、現在の1行のシンプルなプロンプトを構造化された3セクション構成のプロンプトテンプレートに置き換えます。既存のCLI実行フロー、Ollama翻訳機能、UI設定管理には影響を与えません。

### Goals
- Claude CLI翻訳の品質スコアを50/100から80/100以上に向上
- 技術文書特化型の翻訳ルール（プログラミング用語保持、APIドキュメント構造保持）を実装
- 専門用語辞書（10種類の基本用語）をシステムプロンプトに統合
- 自然な表現変換ガイドライン（「です・ます調」優先、能動態/受動態）を適用
- 翻訳品質を測定するためのテストケースセット（30サンプル）を準備

### Non-Goals
- Ollama翻訳のシステムプロンプト最適化（Claude CLI専用の改善）
- 新しい翻訳プロバイダーの追加
- UI/UXの変更（設定パネル、翻訳ポップアップ等）
- 翻訳以外の機能（要約、返信生成）への影響

## Architecture

### Existing Architecture Analysis

**現在のアーキテクチャ**:
- `src-tauri/src/llm/claude_cli.rs`: Claude CLI翻訳エンジン（170行、単一関数`translate_with_claude_cli`）
- `src-tauri/src/services/translation.rs`: Ollama翻訳エンジン、共通型定義（`Language`, `TranslationResult`, `TranslationError`）
- Tauri IPC境界: フロントエンド（`src/lib/claude-cli.ts`）→ バックエンド（`translate_with_claude_cli` IPCコマンド）

**現在のシステムプロンプト実装**（`claude_cli.rs:39-43`）:
```rust
let system_prompt = format!(
    "You are a professional translator. Translate the following text from {} to {} while preserving meaning, tone, and context.",
    source_lang.name(),
    target_lang.name()
);
```

**課題**:
- 極めてシンプルな役割定義のみで、技術文書特化型のルールが欠如
- プログラミング用語、コードスニペット、API構造への配慮なし
- 専門用語辞書や自然な表現ガイドラインが未実装

**既存のプロンプト構築パターン**（Ollama翻訳の参考）:
- `build_plamo_prompt()`: PLaMo-2-Translate用のシンプルなプロンプト
- `build_general_prompt()`: 汎用LLM用のシンプルなプロンプト
- `build_translation_prompt()`: モデルタイプで分岐し、言語方向に応じたプロンプトを生成

### Architecture Pattern & Boundary Map

**選択されたパターン**: **Extend Existing Components**（既存コンポーネント拡張）

**統合アプローチ**:
- `claude_cli.rs`の`translate_with_claude_cli`関数内でシステムプロンプト構築ロジックを拡張
- 新しい関数`build_system_prompt(source_lang, target_lang) -> String`を追加し、構造化されたプロンプトテンプレートを生成
- 既存のCLI実行フロー（タイムアウト、エラーハンドリング等）は変更せず、プロンプト内容のみ改善

**ドメイン境界**:
- **Claude CLI翻訳ドメイン** (`src-tauri/src/llm/claude_cli.rs`): Claude CLI実行とシステムプロンプト構築を担当
- **Ollama翻訳ドメイン** (`src-tauri/src/services/translation.rs`): Ollama API実行とプロンプト構築を担当（独立、影響なし）
- **共通型定義** (`src-tauri/src/services/translation.rs`): `Language`, `TranslationResult`, `TranslationError`を共有

**既存パターン維持**:
- Ollama翻訳の`build_plamo_prompt`, `build_general_prompt`パターンを踏襲
- `format!()`マクロで動的にプロンプトを構築
- 言語方向（英→日、日→英）で条件分岐

**新規コンポーネント**:
- `build_system_prompt(source_lang, target_lang) -> String`: 構造化されたシステムプロンプトを生成

**Steering Compliance**:
- **Type Safety**: Rustの型安全性を維持、`Language`型で言語方向を明示
- **Single Responsibility**: `translate_with_claude_cli`はCLI実行のみを担当、プロンプト構築は`build_system_prompt`に分離
- **Separation of Concerns**: プロンプトテンプレート構築ロジックを独立した関数に分離

### Technology Stack

| Layer | Choice / Version | Role in Feature | Notes |
|-------|------------------|-----------------|-------|
| Backend / Services | Rust 1.70+ | システムプロンプト構築ロジックの実装 | 既存の`claude_cli.rs`を拡張、新しい依存関係なし |
| Backend / LLM Client | Claude CLI (`claude -p`) | Claude Code CLIでシステムプロンプトを渡して翻訳実行 | `--system-prompt`引数で最適化されたプロンプトを渡す |
| Data / Prompt Caching | Claude CLI Prompt Caching | システムプロンプトの固定部分をキャッシュし、実行効率を向上 | 固定部分（翻訳ルール、ガイドライン）を最大化、2000トークン以内 |

**技術選択の根拠**:
- **Rust 1.70+**: 既存のバックエンドと一貫性を保ち、型安全性を維持
- **Claude CLI (`claude -p`)**: 既に統合済み、`--system-prompt`引数で構造化されたプロンプトを渡す
- **Prompt Caching**: `--system-prompt`引数で自動的に有効化、固定部分をキャッシュして実行効率を向上

## Requirements Traceability

| Requirement | Summary | Components | Interfaces | Flows |
|-------------|---------|------------|------------|-------|
| 1.1, 1.2 | システムプロンプト構造最適化（3セクション構成） | System Prompt Builder | `build_system_prompt()` | N/A |
| 1.3, 1.4, 1.5 | プロンプトキャッシング、役割定義、言語方向指定 | System Prompt Builder | `build_system_prompt()` | N/A |
| 2.1, 2.2, 2.3, 2.4, 2.5, 2.6 | 技術文書特化型翻訳ルール | System Prompt Builder (rules_section) | `build_system_prompt()` | N/A |
| 3.1, 3.2, 3.3, 3.4, 3.5 | コンテキスト保持機能 | System Prompt Builder (quality_guidelines) | `build_system_prompt()` | N/A |
| 4.1, 4.2, 4.3, 4.4, 4.5 | 専門用語辞書統合（10種類） | System Prompt Builder (rules_section) | `build_system_prompt()` | N/A |
| 5.1, 5.2, 5.3, 5.4, 5.5, 5.6 | 自然な表現変換ガイドライン | System Prompt Builder (language-specific rules) | `build_system_prompt()` | N/A |
| 6.1, 6.2, 6.3, 6.4, 6.5, 6.6 | 翻訳品質測定・検証（30サンプル） | Translation Quality Test Suite | テストケースファイル | N/A |
| 7.1, 7.2, 7.3, 7.4, 7.5, 7.6 | システムプロンプト実装・統合 | Claude CLI Translation Service | `translate_with_claude_cli()` | CLI実行フロー |
| 8.1, 8.2, 8.3, 8.4, 8.5 | 既存機能互換性維持 | Claude CLI Translation Service | `translate_with_claude_cli()` | 既存フロー維持 |
| 9.1, 9.2, 9.3, 9.4, 9.5 | ドキュメントとテスト整備 | Documentation, Test Suite | ドキュメントファイル、テストケース | N/A |

## Components and Interfaces

| Component | Domain/Layer | Intent | Req Coverage | Key Dependencies (P0/P1) | Contracts |
|-----------|--------------|--------|--------------|--------------------------|-----------|
| System Prompt Builder | Backend/LLM | 構造化されたシステムプロンプトを生成 | 1, 2, 3, 4, 5 | Language (P0) | Service |
| Claude CLI Translation Service | Backend/LLM | Claude CLI実行とエラーハンドリング | 7, 8 | System Prompt Builder (P0), tokio::process::Command (P0) | Service |
| Translation Quality Test Suite | Testing | 翻訳品質の測定と検証 | 6 | System Prompt Builder (P0), Claude CLI (P0) | Batch |

### Backend/LLM

#### System Prompt Builder

| Field | Detail |
|-------|--------|
| Intent | 構造化された3セクション構成のシステムプロンプトを生成し、技術文書特化型の翻訳ルール、専門用語辞書、自然な表現ガイドラインを統合 |
| Requirements | 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 3.1, 3.2, 3.3, 3.4, 3.5, 4.1, 4.2, 4.3, 4.4, 4.5, 5.1, 5.2, 5.3, 5.4, 5.5, 5.6 |

**Responsibilities & Constraints**
- 言語方向（英→日、日→英）に応じて異なる翻訳ルールを適用
- 専門用語辞書（10種類）をプロンプト内に埋め込み、一貫した訳語を保証
- プロンプト長を2000トークン以内に制限し、プロンプトキャッシングの効率を維持

**Dependencies**
- Inbound: `translate_with_claude_cli` — システムプロンプト生成リクエスト (P0)
- Outbound: なし
- External: `Language`型 — 翻訳元言語と翻訳先言語を表現 (P0)

**Contracts**: Service [X]

##### Service Interface
```rust
/// 構造化されたシステムプロンプトを生成
///
/// # Arguments
/// * `source_lang` - 翻訳元言語
/// * `target_lang` - 翻訳先言語
///
/// # Returns
/// 3セクション構成のシステムプロンプト文字列
/// - セクション1: 役割定義
/// - セクション2: 翻訳ルール（技術文書特化型、専門用語辞書）
/// - セクション3: 品質ガイドライン（コンテキスト保持、自然な表現）
///
/// # Constraints
/// - プロンプト長は2000トークン以内
/// - 言語方向（英→日、日→英）で異なるルールを適用
fn build_system_prompt(source_lang: Language, target_lang: Language) -> String;
```

**Preconditions**:
- `source_lang`と`target_lang`は`Language::Japanese`または`Language::English`のいずれか

**Postconditions**:
- 3セクション構成のプロンプト文字列が返される
- プロンプト長は2000トークン以内
- 言語方向に応じた翻訳ルールが適用される

**Invariants**:
- プロンプトは常に役割定義、翻訳ルール、品質ガイドラインの3セクションを含む
- 専門用語辞書は常に10種類の基本用語を含む

**Implementation Notes**
- **Integration**: `translate_with_claude_cli`関数内で`build_system_prompt(source_lang, target_lang)`を呼び出し、生成されたプロンプトを`--system-prompt`引数に渡す
- **Validation**: プロンプト長を2000トークン以内に制限。ユニットテスト`test_system_prompt_token_limit`でトークン数を計測（英語: 1トークン≒4文字、日本語: 1トークン≒2文字の近似値を使用）し、英→日と日→英の両方向で2000トークン以内であることを検証（詳細はTesting Strategyセクションを参照）
- **Risks**: プロンプト長超過の可能性 → ユニットテストで継続的に検証し、超過した場合は翻訳ルールまたは品質ガイドラインを簡潔化

**Prompt Template Structure**:
```rust
fn build_system_prompt(source_lang: Language, target_lang: Language) -> String {
    // セクション1: 役割定義
    let role_section = "あなたはプロフェッショナルな技術翻訳者です。\
        技術文書の翻訳において、技術的な正確性を最優先し、\
        開発者が理解しやすい翻訳を提供します。";

    // セクション2: 翻訳ルール（言語方向別に分岐）
    let rules_section = match (source_lang, target_lang) {
        (Language::English, Language::Japanese) => {
            // 英→日の技術文書翻訳ルール
            "# 翻訳ルール\n\
            - プログラミング言語のキーワードや識別子は原文のまま保持\n\
            - コードブロック（```で囲まれた部分）は翻訳しない\n\
            - APIエンドポイント、HTTPメソッド、ファイルパス、コマンドライン引数は原文のまま保持\n\
            - 専門用語は以下の訳語を使用:\n\
              - API → API（訳さない）\n\
              - framework → フレームワーク\n\
              - library → ライブラリ\n\
              - module → モジュール\n\
              - function → 関数\n\
              - variable → 変数\n\
              - interface → インターフェース\n\
              - class → クラス\n\
              - object → オブジェクト\n\
              - array → 配列\n\
            - プログラミング言語名（React、TypeScript、Rust等）は原文のまま保持\n\
            - クラウドサービス名やツール名（GitHub、Docker、Kubernetes等）は原文のまま保持\n\
            - 専門用語の訳語が不明な場合は、原文を括弧付きで併記（例: インターフェース (interface)）\n\
            - 技術文書では「です・ます調」を優先\n\
            - 技術的な正確性を最優先し、意訳よりも直訳を重視"
        },
        (Language::Japanese, Language::English) => {
            // 日→英の技術文書翻訳ルール
            "# Translation Rules\n\
            - Preserve programming language keywords and identifiers in original form\n\
            - Do not translate code blocks (enclosed by ```)\n\
            - Preserve API endpoints, HTTP methods, file paths, command-line arguments in original form\n\
            - Use the following terminology:\n\
              - API → API (do not translate)\n\
              - フレームワーク → framework\n\
              - ライブラリ → library\n\
              - モジュール → module\n\
              - 関数 → function\n\
              - 変数 → variable\n\
              - インターフェース → interface\n\
              - クラス → class\n\
              - オブジェクト → object\n\
              - 配列 → array\n\
            - Preserve programming language names (React, TypeScript, Rust, etc.)\n\
            - Preserve cloud service names and tool names (GitHub, Docker, Kubernetes, etc.)\n\
            - Use active voice when appropriate\n\
            - Prioritize technical accuracy over idiomatic expressions"
        },
        _ => {
            // その他の言語方向（予期しない組み合わせ）
            "# Translation Rules\n\
            - Preserve technical terms and code elements in original form\n\
            - Maintain consistency in terminology"
        }
    };

    // セクション3: 品質ガイドライン
    let quality_guidelines = "# 品質ガイドライン\n\
        - 翻訳対象テキストの前後の文脈を考慮し、代名詞や接続詞の翻訳を適切に行う\n\
        - 複数文からなるテキストでは、文間の論理的なつながりを保持\n\
        - 技術文書では、同一概念に対して一貫した訳語を使用\n\
        - 箇条書きやリスト構造を含むテキストでは、構造を保持し、各項目の翻訳の一貫性を保つ\n\
        - 翻訳結果は元のテキストの意図とニュアンスを正確に反映\n\
        - 翻訳結果は対象言語のネイティブスピーカーが読んで自然に感じる表現を使用\n\
        - 冗長な表現や不自然な直訳を避け、簡潔で明瞭な表現を使用\n\
        - 技術的な正確性を損なわない範囲で、読みやすさを向上";

    format!("{}\n\n{}\n\n{}", role_section, rules_section, quality_guidelines)
}
```

#### Claude CLI Translation Service

| Field | Detail |
|-------|--------|
| Intent | Claude CLI実行、タイムアウト管理、エラーハンドリングを担当し、既存のCLI実行フローを維持 |
| Requirements | 7.1, 7.2, 7.3, 7.4, 7.5, 7.6, 8.1, 8.2, 8.3, 8.4, 8.5 |

**Responsibilities & Constraints**
- システムプロンプトを構築し、`claude -p --system-prompt`引数で渡す
- 30秒タイムアウトでCLI実行を管理
- CLI実行エラー、タイムアウト、JSON出力パースエラーを適切にハンドリング

**Dependencies**
- Inbound: フロントエンド（`src/lib/claude-cli.ts`） — 翻訳リクエスト (P0)
- Outbound: System Prompt Builder — システムプロンプト生成 (P0)
- External: Claude CLI (`claude -p`) — テキスト翻訳実行 (P0)

**Contracts**: Service [X]

##### Service Interface
```rust
/// Claude CLIで翻訳を実行
///
/// # Arguments
/// * `text` - 翻訳するテキスト
/// * `source_lang` - 翻訳元言語
/// * `target_lang` - 翻訳先言語
/// * `cli_path` - Claude CLIの実行パス（Noneの場合はデフォルトの"claude"を使用）
///
/// # Returns
/// 翻訳結果、またはエラー
pub async fn translate_with_claude_cli(
    text: &str,
    source_lang: Language,
    target_lang: Language,
    cli_path: Option<&str>,
) -> Result<TranslationResult, TranslationError>;
```

**Preconditions**:
- `text`は非空文字列
- `source_lang`と`target_lang`は`Language::Japanese`または`Language::English`のいずれか
- Claude CLIが実行可能（`cli_path`が有効なパス、またはデフォルトの`claude`コマンドが利用可能）

**Postconditions**:
- 成功時: `TranslationResult`が返され、`translated_text`に翻訳結果が含まれる
- 失敗時: `TranslationError`が返され、適切なエラーメッセージが含まれる（タイムアウト、CLI実行エラー、JSON出力パースエラー）

**Invariants**:
- 既存のCLI実行フロー（タイムアウト、エラーハンドリング）は変更されない
- プロンプト内容のみが変更され、関数シグネチャは不変

**Implementation Notes**
- **Integration**: `build_system_prompt(source_lang, target_lang)`を呼び出し、生成されたプロンプトを`--system-prompt`引数に渡す
- **Validation**: 既存のテスト（`#[ignore]`付き統合テスト）がパスすることを確認
- **Risks**: なし（既存ロジックは不変）

**Modified Implementation**:
```rust
pub async fn translate_with_claude_cli(
    text: &str,
    source_lang: Language,
    target_lang: Language,
    cli_path: Option<&str>,
) -> Result<TranslationResult, TranslationError> {
    let start = Instant::now();
    let cli_command = cli_path.unwrap_or("claude");

    // システムプロンプトの構築（最適化版）
    let system_prompt = build_system_prompt(source_lang, target_lang);

    // コマンドの構築と実行（既存ロジック維持）
    let child = Command::new(cli_command)
        .arg("-p")
        .arg("--system-prompt")
        .arg(&system_prompt)
        .arg("--output-format")
        .arg("json")
        .arg(text)
        .kill_on_drop(true)
        .output();

    // 30秒タイムアウトで実行（既存ロジック維持）
    let output = timeout(Duration::from_secs(30), child)
        .await
        .map_err(|_| TranslationError::Timeout)?
        .map_err(|e| {
            TranslationError::ConnectionFailed(format!(
                "Claude CLIの実行に失敗しました: {}",
                e
            ))
        })?;

    // Exit codeの検証（既存ロジック維持）
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TranslationError::ApiError(format!(
            "Claude CLIがエラーで終了しました (exit code: {}): {}",
            output.status.code().unwrap_or(-1),
            stderr
        )));
    }

    // JSON出力のパース（既存ロジック維持）
    let stdout = String::from_utf8_lossy(&output.stdout);
    let cli_output: ClaudeCliOutput = serde_json::from_str(&stdout).map_err(|e| {
        TranslationError::ApiError(format!("JSON出力のパースに失敗しました: {}", e))
    })?;

    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(TranslationResult {
        translated_text: cli_output.output,
        source_lang,
        target_lang,
        duration_ms,
    })
}
```

### Testing

#### Translation Quality Test Suite

| Field | Detail |
|-------|--------|
| Intent | 翻訳品質を測定し、目標スコア80/100以上を検証 |
| Requirements | 6.1, 6.2, 6.3, 6.4, 6.5, 6.6 |

**Responsibilities & Constraints**
- 30サンプルのテストケースセット（APIドキュメント10、エラーメッセージ5、技術ブログ10、コードコメント5）を準備
- 5項目評価基準（専門用語30点、文脈25点、表現25点、構造10点、技術10点）で品質スコアを算出
- 最適化前と最適化後のシステムプロンプトで翻訳を実行し、結果を比較

**Dependencies**
- Inbound: CI/CDパイプライン — テストケース自動実行 (P1)
- Outbound: System Prompt Builder — 最適化されたプロンプト生成 (P0)
- External: Claude CLI (`claude -p`) — テストケース翻訳実行 (P0)

**Contracts**: Batch [X]

##### Batch / Job Contract
- **Trigger**: CI/CDパイプライン（GitHub Actions等）で自動実行、またはローカルで`cargo test`実行
- **Input / validation**: 30サンプルのテストケースセット（JSON形式、各サンプルに元テキスト、翻訳方向、カテゴリを含む）
- **Output / destination**: 品質スコア（0-100点）、5項目評価の詳細、最適化前後の比較結果をログファイルまたはCI/CD出力に記録
- **Idempotency & recovery**: 各テストケースは独立して実行可能、失敗したケースのみ再実行可能

**Implementation Notes**
- **Integration**: テストケースファイルを`src-tauri/tests/translation_quality_test_cases.json`に配置し、Rustテストで読み込んで実行
- **Validation**: 品質スコア80/100以上を達成することを確認、80点未満の場合はテスト失敗
- **Risks**: テストケースの主観性 → 評価基準を明確に定義し、再現可能な形で保存

**Test Case Structure**:
```rust
#[derive(Debug, Deserialize)]
struct TranslationQualityTestCase {
    /// テストケースID
    id: String,
    /// カテゴリ（api_doc, error_message, tech_blog, code_comment）
    category: String,
    /// 元テキスト
    source_text: String,
    /// 翻訳方向
    source_lang: Language,
    target_lang: Language,
    /// 期待される翻訳（参考用、スコア算出には使用しない）
    expected_translation: Option<String>,
}

#[tokio::test]
#[ignore] // CI環境でClaude CLIが利用できない場合はスキップ
async fn test_translation_quality_all_cases() {
    // テストケースを読み込み
    let test_cases = load_test_cases("tests/translation_quality_test_cases.json").unwrap();

    let mut total_score = 0;
    for case in test_cases {
        // 最適化されたプロンプトで翻訳実行
        let result = translate_with_claude_cli(
            &case.source_text,
            case.source_lang,
            case.target_lang,
            None,
        )
        .await
        .unwrap();

        // 品質スコアを算出（5項目評価）
        let score = evaluate_translation_quality(&case, &result.translated_text);
        total_score += score;
    }

    let average_score = total_score / test_cases.len();
    assert!(average_score >= 80, "翻訳品質スコアが80点未満: {}", average_score);
}
```

## Error Handling

### Error Strategy
既存のエラーハンドリングパターンを維持し、新しいエラー種別は導入しません。

### Error Categories and Responses
**User Errors** (4xx): 入力テキストが空 → フロントエンドで検証（既存ロジック）
**System Errors** (5xx): Claude CLI実行エラー → `TranslationError::ConnectionFailed`、タイムアウト → `TranslationError::Timeout`、JSON出力パースエラー → `TranslationError::ApiError`（既存ロジック）
**Business Logic Errors** (422): なし（翻訳品質は自動的に向上、ユーザー入力なし）

### Monitoring
既存のエラーログ機構を維持し、新しいログ記録は不要です。

## Testing Strategy

### Unit Tests
- `build_system_prompt`関数のテスト: 英→日、日→英の両方向でプロンプトが正しく生成されることを確認
- プロンプト長が2000トークン以内であることを確認: トークン数計測（英語: 1トークン≒4文字、日本語: 1トークン≒2文字の近似値を使用）し、英→日と日→英の両方向で生成されたプロンプト文字列のトークン数が2000以内であることをアサート。テスト実装例:
  ```rust
  #[test]
  fn test_system_prompt_token_limit() {
      let prompt_en_to_ja = build_system_prompt(Language::English, Language::Japanese);
      let prompt_ja_to_en = build_system_prompt(Language::Japanese, Language::English);

      // トークン数近似（英語: 4文字/トークン、日本語: 2文字/トークン）
      let estimate_en_to_ja = prompt_en_to_ja.chars()
          .filter(|c| c.is_ascii()).count() / 4
          + prompt_en_to_ja.chars().filter(|c| !c.is_ascii()).count() / 2;
      let estimate_ja_to_en = prompt_ja_to_en.chars()
          .filter(|c| c.is_ascii()).count() / 4
          + prompt_ja_to_en.chars().filter(|c| !c.is_ascii()).count() / 2;

      assert!(estimate_en_to_ja <= 2000, "英→日プロンプトが2000トークン超過: {}", estimate_en_to_ja);
      assert!(estimate_ja_to_en <= 2000, "日→英プロンプトが2000トークン超過: {}", estimate_ja_to_en);
  }
  ```
- 専門用語辞書（10種類）がプロンプトに含まれることを確認

### Integration Tests
- `translate_with_claude_cli`関数のテスト（`#[ignore]`付き）: 最適化されたプロンプトでClaude CLI実行が成功することを確認
- 既存のテスト（`test_translate_success`, `test_translate_cli_not_found`, `test_translate_timeout`）がパスすることを確認

### E2E/UI Tests
- なし（フロントエンドUIは変更なし、バックエンドのみの変更）

### Performance/Load
- プロンプトキャッシング効率の検証: 同じ言語方向で複数回翻訳を実行し、2回目以降の実行時間が短縮されることを確認
- 翻訳品質スコア80/100以上の達成確認: 30サンプルのテストケースで品質スコアを測定

## Optional Sections

### Performance & Scalability
- **Target Metrics**: 翻訳品質スコア80/100以上、プロンプトキャッシングによる2回目以降の実行時間短縮（10-20%削減目標）
- **Caching Strategy**: システムプロンプトの固定部分（翻訳ルール、品質ガイドライン）をClaude CLIのプロンプトキャッシングで自動的にキャッシュ

### Migration Strategy
移行は不要です。既存ユーザーはアプリケーションアップデート時に自動的に最適化されたプロンプトを使用します。

## Supporting References

なし（すべての情報は本設計書に含まれています）
