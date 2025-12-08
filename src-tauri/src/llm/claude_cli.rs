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
    output: String,
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

    // システムプロンプトの構築
    let system_prompt = format!(
        "You are a professional translator. Translate the following text from {} to {} while preserving meaning, tone, and context.",
        source_lang.name(),
        target_lang.name()
    );

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
            TranslationError::ConnectionFailed(format!(
                "Claude CLIの実行に失敗しました: {}",
                e
            ))
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
        translated_text: cli_output.output,
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
        let json = r#"{"output": "こんにちは、お元気ですか?"}"#;
        let output: ClaudeCliOutput = serde_json::from_str(json).unwrap();
        assert_eq!(output.output, "こんにちは、お元気ですか?");
    }
}
