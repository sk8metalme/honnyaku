//! Claude CLI翻訳サービス
//!
//! Claude Code CLIを使用したテキスト翻訳機能を提供

use crate::services::translation::{Language, TranslationError, TranslationResult};
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tokio::time::timeout;

/// Claude CLI実行結果のJSON構造
#[derive(Debug, Deserialize)]
struct ClaudeCliOutput {
    result: String,
}

/// 構造化されたシステムプロンプトを構築
///
/// 3セクション構成のプロンプトを生成:
/// 1. 役割定義セクション
/// 2. 翻訳ルールセクション（基本構造、詳細はタスク1.2-1.3で実装）
/// 3. 品質ガイドラインセクション（基本構造、詳細はタスク1.4で実装）
///
/// # Arguments
/// * `source_lang` - 翻訳元言語
/// * `target_lang` - 翻訳先言語
///
/// # Returns
/// 構造化されたシステムプロンプト文字列
fn build_system_prompt(source_lang: Language, target_lang: Language) -> String {
    // 言語方向に応じて異なるプロンプトを生成
    match (source_lang, target_lang) {
        (Language::English, Language::Japanese) => {
            // 英→日のプロンプト
            r#"あなたはプロフェッショナルな技術翻訳者です。

翻訳ルール:
- Englishから日本語への翻訳を行います
- 技術文書に特化した翻訳を提供します

プログラミング用語とコードの保持:
- プログラミング言語のキーワード（if, for, class, function等）や識別子（変数名、関数名、クラス名）は原文のまま保持してください
- コードブロック（バッククォート3つ```で囲まれた部分）内のコードは翻訳せず、そのまま保持してください
- インラインコード（バッククォート1つ`で囲まれた部分）も原文のまま保持してください
- APIエンドポイント（例: /api/users, GET /posts）、HTTPメソッド（GET, POST, PUT, DELETE）は原文のまま保持してください
- ファイルパス（例: /src/main.rs, config.json）やコマンドライン引数（例: --verbose, -h）は原文のまま保持してください

技術用語の一貫性:
- 技術用語の日本語訳が複数存在する場合（例: cache→キャッシュ/キャッシ）、文脈に応じて最も一般的な訳語を統一して使用してください
- 同一文書内では同じ技術用語に対して同じ訳語を使用してください

専門用語辞書（基本10用語）:
以下の基本的な技術用語は、文脈に応じて適切な日本語訳を使用してください:
- API: API（そのまま）またはアプリケーションプログラミングインターフェース
- framework: フレームワーク
- library: ライブラリ
- module: モジュール
- function: 関数
- variable: 変数
- interface: インターフェース
- class: クラス
- object: オブジェクト
- array: 配列

プログラミング言語名・ツール名の保持:
- プログラミング言語名（React、TypeScript、Rust、Python、JavaScript等）は原文のまま保持してください
- クラウドサービス名やツール名（GitHub、Docker、Kubernetes、AWS、Azure等）は原文のまま保持してください
- 専門用語の訳語が不明な場合は、原文を括弧付きで併記してください（例: 「レンダリング（rendering）」）

翻訳の方針:
- 開発者向けドキュメントの翻訳では、技術的な正確性を最優先してください
- 意訳よりも直訳を重視し、原文の意味を正確に伝えてください
- ただし、日本語として不自然にならないよう配慮してください

品質ガイドライン:

コンテキスト保持:
- 翻訳対象テキストの前後の文脈を考慮し、代名詞（it, they, this等）や接続詞（however, therefore等）の翻訳を適切に行ってください
- 複数文からなるテキストでは、文間の論理的なつながり（因果関係、対比、列挙等）を保持してください
- 技術文書では同一概念に対して一貫した訳語を使用してください（例: 「コンテナ」と「コンテナー」を混在させない）
- 箇条書きやリスト構造を含むテキストでは、構造を保持し、各項目の翻訳の一貫性を保ってください

自然な表現:
- 翻訳結果は対象言語のネイティブスピーカーが読んで自然に感じる表現を使用してください
- 日本語への翻訳では、「です・ます調」の丁寧語を優先してください
- 冗長な表現や不自然な直訳を避け、簡潔で明瞭な表現を使用してください
- ただし、技術的正確性を損なわない範囲で自然な表現を心がけてください
"#.to_string()
        }
        (Language::Japanese, Language::English) => {
            // 日→英のプロンプト
            r#"You are a professional technical translator.

Translation Rules:
- Translate from Japanese to English
- Provide technical document-focused translations

Programming Terms and Code Preservation:
- Preserve programming language keywords (if, for, class, function, etc.) and identifiers (variable names, function names, class names) as-is
- Do not translate code blocks (enclosed in triple backticks ```) - preserve them as-is
- Preserve inline code (enclosed in single backticks `) as-is
- Preserve API endpoints (e.g., /api/users, GET /posts) and HTTP methods (GET, POST, PUT, DELETE) as-is
- Preserve file paths (e.g., /src/main.rs, config.json) and command-line arguments (e.g., --verbose, -h) as-is

Technical Terminology Consistency:
- When technical terms have multiple possible translations, use the most common one consistently based on context
- Use the same translation for the same technical term within the same document

Technical Terminology Dictionary (Basic 10 Terms):
Use appropriate translations for the following basic technical terms based on context:
- API: API (as-is)
- framework: framework (as-is)
- library: library (as-is)
- module: module (as-is)
- function: function (as-is)
- variable: variable (as-is)
- interface: interface (as-is)
- class: class (as-is)
- object: object (as-is)
- array: array (as-is)

Programming Language Names and Tool Names Preservation:
- Preserve programming language names (React, TypeScript, Rust, Python, JavaScript, etc.) as-is
- Preserve cloud service names and tool names (GitHub, Docker, Kubernetes, AWS, Azure, etc.) as-is
- If the translation of a technical term is unclear, provide the original term in parentheses (e.g., "rendering (レンダリング)")

Translation Policy:
- For developer documentation, prioritize technical accuracy above all
- Prefer literal translation over paraphrasing to convey the original meaning accurately
- However, ensure the English remains natural and readable

Quality Guidelines:

Context Preservation:
- Consider the context before and after the translation target text, and translate pronouns (それ, これ, etc.) and conjunctions (しかし, したがって, etc.) appropriately
- For multi-sentence texts, preserve logical connections between sentences (causality, contrast, enumeration, etc.)
- Use consistent terminology for the same concept in technical documents (e.g., don't mix "container" and "containerization" inconsistently)
- For texts with bullet points or list structures, preserve the structure and maintain consistency in translating each item

Natural Expression:
- Use expressions that feel natural to native speakers of the target language
- For English translations, appropriately use active and passive voice based on context
- Avoid verbose expressions and unnatural literal translations; use concise and clear expressions
- However, maintain natural expression within the scope that doesn't compromise technical accuracy
"#.to_string()
        }
        _ => {
            // その他の言語方向（将来の拡張用）
            format!(
                r#"You are a professional technical translator.

Translation Rules:
- Translate from {} to {}
- Provide technical document-focused translations

Quality Guidelines:
- Translation results should be natural and readable
- Preserve context
"#,
                source_lang.name(),
                target_lang.name()
            )
        }
    }
}

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
) -> Result<TranslationResult, TranslationError> {
    let start = Instant::now();

    // Claude CLIパスの決定
    let cli_command = cli_path.unwrap_or("claude");

    // システムプロンプトの構築（最適化された3セクション構成）
    let system_prompt = build_system_prompt(source_lang, target_lang);

    // コマンドの構築と実行
    let child = Command::new(cli_command)
        .arg("-p")
        .arg("--system-prompt")
        .arg(&system_prompt)
        .arg("--output-format")
        .arg("json")
        .arg(text)
        .kill_on_drop(true)
        .output();

    // 30秒タイムアウトで実行
    let output = timeout(Duration::from_secs(30), child)
        .await
        .map_err(|_| TranslationError::Timeout)?
        .map_err(|e| {
            TranslationError::ConnectionFailed(format!("Claude CLIの実行に失敗しました: {}", e))
        })?;

    // Exit codeの検証
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TranslationError::ApiError(format!(
            "Claude CLIがエラーで終了しました (exit code: {}): {}",
            output.status.code().unwrap_or(-1),
            stderr
        )));
    }

    // JSON出力のパース
    let stdout = String::from_utf8_lossy(&output.stdout);
    let cli_output: ClaudeCliOutput = serde_json::from_str(&stdout).map_err(|e| {
        TranslationError::ApiError(format!("JSON出力のパースに失敗しました: {}", e))
    })?;

    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(TranslationResult {
        translated_text: cli_output.result,
        source_lang,
        target_lang,
        duration_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // 注: 実際のClaude CLI実行が必要なため、統合テストとしてマーク
    // 単体テストではモックを使用することを推奨

    #[tokio::test]
    #[ignore] // CI環境でClaude CLIが利用できない場合はスキップ
    async fn test_translate_success() {
        // 正常系: 翻訳成功
        // Note: この テストは実際のClaude CLIが必要
        let result = translate_with_claude_cli(
            "Hello, how are you?",
            Language::English,
            Language::Japanese,
            None,
        )
        .await;

        assert!(result.is_ok());
        let translation = result.unwrap();
        assert!(!translation.translated_text.is_empty());
        assert_eq!(translation.source_lang, Language::English);
        assert_eq!(translation.target_lang, Language::Japanese);
        assert!(translation.duration_ms > 0);
    }

    #[tokio::test]
    async fn test_translate_cli_not_found() {
        // 異常系: CLIが見つからない
        let result = translate_with_claude_cli(
            "Hello",
            Language::English,
            Language::Japanese,
            Some("/nonexistent/path/to/claude"),
        )
        .await;

        assert!(result.is_err());
        match result {
            Err(TranslationError::ConnectionFailed(msg)) => {
                assert!(msg.contains("Claude CLIの実行に失敗"));
            }
            _ => panic!("Expected ConnectionFailed error"),
        }
    }

    #[tokio::test]
    #[ignore] // タイムアウトテストは時間がかかるためスキップ
    async fn test_translate_timeout() {
        // 異常系: タイムアウト
        // Note: 実際にタイムアウトをテストするには、
        // 30秒以上かかるコマンドが必要（モック推奨）

        // この例では、存在しないコマンドでタイムアウトをシミュレート
        // 実際の実装では、モックを使用してタイムアウトをテストする
    }

    #[tokio::test]
    async fn test_translate_invalid_json() {
        // 異常系: JSONパースエラー
        // Note: 実際のClaude CLIは正しいJSONを返すため、
        // このテストはモックでのみ実施可能

        // 実装例: カスタムコマンドで不正なJSONを返す
        // 実際の実装では、モックを使用してテストする
    }

    #[test]
    fn test_claude_cli_output_deserialization() {
        // ClaudeCliOutput構造体のデシリアライズテスト
        let json = r#"{"result": "こんにちは、お元気ですか?"}"#;
        let output: ClaudeCliOutput = serde_json::from_str(json).unwrap();
        assert_eq!(output.result, "こんにちは、お元気ですか?");
    }

    #[test]
    fn test_build_system_prompt_english_to_japanese() {
        // 英→日のシステムプロンプトが3セクション構成になっていることを確認
        let prompt = build_system_prompt(Language::English, Language::Japanese);

        // 役割定義セクションが含まれることを確認
        assert!(
            prompt.contains("プロフェッショナルな技術翻訳者"),
            "役割定義セクションが見つかりません"
        );

        // 翻訳ルールセクションの存在を確認（基本的な構造のみ、詳細はタスク1.2-1.4で実装）
        assert!(
            prompt.contains("翻訳ルール"),
            "翻訳ルールセクションが見つかりません"
        );

        // 品質ガイドラインセクションの存在を確認（基本的な構造のみ、詳細はタスク1.4で実装）
        assert!(
            prompt.contains("品質ガイドライン"),
            "品質ガイドラインセクションが見つかりません"
        );

        // 言語方向が正しく反映されていることを確認
        assert!(prompt.contains("English") || prompt.contains("英語"));
        assert!(prompt.contains("Japanese") || prompt.contains("日本語"));
    }

    #[test]
    fn test_build_system_prompt_japanese_to_english() {
        // 日→英のシステムプロンプトが3セクション構成になっていることを確認
        let prompt = build_system_prompt(Language::Japanese, Language::English);

        // 役割定義セクションが含まれることを確認
        assert!(
            prompt.contains("professional technical translator"),
            "役割定義セクションが見つかりません"
        );

        // 翻訳ルールセクションの存在を確認
        assert!(
            prompt.contains("Translation Rules") || prompt.contains("translation rules"),
            "翻訳ルールセクションが見つかりません"
        );

        // 品質ガイドラインセクションの存在を確認
        assert!(
            prompt.contains("Quality Guidelines") || prompt.contains("quality guidelines"),
            "品質ガイドラインセクションが見つかりません"
        );

        // 言語方向が正しく反映されていることを確認
        assert!(prompt.contains("Japanese") || prompt.contains("日本語"));
        assert!(prompt.contains("English") || prompt.contains("英語"));
    }

    #[test]
    fn test_build_system_prompt_different_for_directions() {
        // 英→日と日→英で異なるプロンプトが生成されることを確認
        let prompt_en_to_ja = build_system_prompt(Language::English, Language::Japanese);
        let prompt_ja_to_en = build_system_prompt(Language::Japanese, Language::English);

        assert_ne!(
            prompt_en_to_ja, prompt_ja_to_en,
            "言語方向が異なる場合、プロンプトも異なるべきです"
        );
    }

    #[test]
    fn test_build_system_prompt_includes_technical_rules() {
        // 技術文書特化型ルールがプロンプトに含まれることを確認（タスク1.2）
        let prompt = build_system_prompt(Language::English, Language::Japanese);

        // プログラミング用語の保持ルール
        assert!(
            prompt.contains("プログラミング")
                || prompt.contains("キーワード")
                || prompt.contains("識別子"),
            "プログラミング用語保持ルールが見つかりません"
        );

        // コードブロック非翻訳ルール
        assert!(
            prompt.contains("コード") && (prompt.contains("翻訳しない") || prompt.contains("保持")),
            "コードブロック非翻訳ルールが見つかりません"
        );

        // APIエンドポイント等の保持ルール
        assert!(
            prompt.contains("API") || prompt.contains("HTTP") || prompt.contains("エンドポイント"),
            "API/HTTPメソッド保持ルールが見つかりません"
        );

        // 技術用語の一貫性ルール
        assert!(
            prompt.contains("一貫") || prompt.contains("統一"),
            "技術用語一貫性ルールが見つかりません"
        );

        // 技術的正確性優先ルール
        assert!(
            prompt.contains("正確") || prompt.contains("直訳"),
            "技術的正確性優先ルールが見つかりません"
        );
    }

    #[test]
    fn test_build_system_prompt_technical_rules_english() {
        // 日→英でも技術文書特化型ルールが含まれることを確認
        let prompt = build_system_prompt(Language::Japanese, Language::English);

        // Programming terms preservation
        assert!(
            prompt.contains("programming")
                || prompt.contains("keywords")
                || prompt.contains("identifiers"),
            "プログラミング用語保持ルール（英語）が見つかりません"
        );

        // Code block non-translation rule
        assert!(
            prompt.contains("code")
                && (prompt.contains("preserve") || prompt.contains("not translate")),
            "コードブロック非翻訳ルール（英語）が見つかりません"
        );

        // Technical accuracy priority
        assert!(
            prompt.contains("accuracy") || prompt.contains("literal"),
            "技術的正確性優先ルール（英語）が見つかりません"
        );
    }

    #[test]
    fn test_build_system_prompt_includes_terminology_dictionary() {
        // 専門用語辞書がプロンプトに含まれることを確認（タスク1.3）
        let prompt = build_system_prompt(Language::English, Language::Japanese);

        // 10種類の基本用語が含まれることを確認
        let required_terms = vec![
            "API",
            "framework",
            "library",
            "module",
            "function",
            "variable",
            "interface",
            "class",
            "object",
            "array",
        ];

        for term in required_terms {
            assert!(
                prompt.contains(term),
                "専門用語辞書に '{}' が見つかりません",
                term
            );
        }

        // プログラミング言語名の保持ルール
        assert!(
            prompt.contains("React") || prompt.contains("TypeScript") || prompt.contains("Rust"),
            "プログラミング言語名保持ルールが見つかりません"
        );

        // クラウドサービス名やツール名の保持ルール
        assert!(
            prompt.contains("GitHub") || prompt.contains("Docker") || prompt.contains("Kubernetes"),
            "クラウドサービス/ツール名保持ルールが見つかりません"
        );

        // 不明な用語の括弧付き併記ルール
        assert!(
            prompt.contains("括弧") || prompt.contains("原文"),
            "不明用語の括弧付き併記ルールが見つかりません"
        );
    }

    #[test]
    fn test_build_system_prompt_terminology_dictionary_english() {
        // 日→英でも専門用語辞書が含まれることを確認
        let prompt = build_system_prompt(Language::Japanese, Language::English);

        // 10種類の基本用語が含まれることを確認
        let required_terms = vec![
            "API",
            "framework",
            "library",
            "module",
            "function",
            "variable",
            "interface",
            "class",
            "object",
            "array",
        ];

        for term in required_terms {
            assert!(
                prompt.contains(term),
                "専門用語辞書に '{}' が見つかりません",
                term
            );
        }

        // Programming language names preservation
        assert!(
            prompt.contains("React") || prompt.contains("TypeScript") || prompt.contains("Rust"),
            "プログラミング言語名保持ルール（英語）が見つかりません"
        );

        // Cloud service/tool names preservation
        assert!(
            prompt.contains("GitHub") || prompt.contains("Docker") || prompt.contains("Kubernetes"),
            "クラウドサービス/ツール名保持ルール（英語）が見つかりません"
        );
    }

    #[test]
    fn test_build_system_prompt_includes_context_and_natural_expression_guidelines() {
        // コンテキスト保持と自然な表現ガイドラインが含まれることを確認（タスク1.4）
        let prompt = build_system_prompt(Language::English, Language::Japanese);

        // コンテキスト保持ルール
        assert!(
            prompt.contains("文脈") || prompt.contains("コンテキスト"),
            "文脈/コンテキスト保持ルールが見つかりません"
        );

        // 代名詞・接続詞の適切な翻訳
        assert!(
            prompt.contains("代名詞") || prompt.contains("接続詞"),
            "代名詞・接続詞の翻訳ルールが見つかりません"
        );

        // 文間の論理的つながり保持
        assert!(
            prompt.contains("論理") || prompt.contains("つながり"),
            "文間の論理的つながり保持ルールが見つかりません"
        );

        // 箇条書き・リスト構造の保持
        assert!(
            prompt.contains("箇条書き") || prompt.contains("リスト"),
            "箇条書き・リスト構造保持ルールが見つかりません"
        );

        // 自然な表現ルール
        assert!(
            prompt.contains("自然") && prompt.contains("表現"),
            "自然な表現ルールが見つかりません"
        );

        // 日本語特有のルール（です・ます調）
        assert!(
            prompt.contains("です・ます調") || prompt.contains("丁寧語"),
            "日本語の文体ルールが見つかりません"
        );

        // 冗長表現回避ルール
        assert!(
            prompt.contains("冗長") || prompt.contains("簡潔"),
            "冗長表現回避ルールが見つかりません"
        );
    }

    #[test]
    fn test_build_system_prompt_natural_expression_guidelines_english() {
        // 日→英でもコンテキスト保持と自然な表現ガイドラインが含まれることを確認
        let prompt = build_system_prompt(Language::Japanese, Language::English);

        // Context preservation
        assert!(
            prompt.contains("context"),
            "コンテキスト保持ルール（英語）が見つかりません"
        );

        // Natural expression
        assert!(
            prompt.contains("natural")
                && (prompt.contains("expression") || prompt.contains("readable")),
            "自然な表現ルール（英語）が見つかりません"
        );

        // Active/passive voice
        assert!(
            prompt.contains("active") || prompt.contains("passive"),
            "能動態/受動態ルール（英語）が見つかりません"
        );

        // Conciseness
        assert!(
            prompt.contains("concise") || prompt.contains("clear"),
            "簡潔性ルール（英語）が見つかりません"
        );
    }

    #[test]
    fn test_system_prompt_token_limit() {
        // プロンプト長が2000トークン以内であることを確認（タスク1.5）
        let prompt_en_to_ja = build_system_prompt(Language::English, Language::Japanese);
        let prompt_ja_to_en = build_system_prompt(Language::Japanese, Language::English);

        // トークン数近似計算（英語: 4文字/トークン、日本語: 2文字/トークン）
        let estimate_en_to_ja = prompt_en_to_ja.chars().filter(|c| c.is_ascii()).count() / 4
            + prompt_en_to_ja.chars().filter(|c| !c.is_ascii()).count() / 2;

        let estimate_ja_to_en = prompt_ja_to_en.chars().filter(|c| c.is_ascii()).count() / 4
            + prompt_ja_to_en.chars().filter(|c| !c.is_ascii()).count() / 2;

        assert!(
            estimate_en_to_ja <= 2000,
            "英→日プロンプトが2000トークン超過: 推定{}トークン",
            estimate_en_to_ja
        );

        assert!(
            estimate_ja_to_en <= 2000,
            "日→英プロンプトが2000トークン超過: 推定{}トークン",
            estimate_ja_to_en
        );

        // デバッグ情報として実際のトークン数を出力
        println!("英→日プロンプト推定トークン数: {}", estimate_en_to_ja);
        println!("日→英プロンプト推定トークン数: {}", estimate_ja_to_en);
    }
}
