//! 翻訳サービス
//!
//! Ollama APIを使用したテキスト翻訳機能を提供

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tauri::Emitter;
use thiserror::Error;

/// 言語
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Japanese,
    English,
}

impl Language {
    /// 言語名を取得
    pub fn name(&self) -> &'static str {
        match self {
            Language::Japanese => "Japanese",
            Language::English => "English",
        }
    }
}

/// 翻訳結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationResult {
    /// 翻訳されたテキスト
    pub translated_text: String,
    /// 翻訳元言語
    pub source_lang: Language,
    /// 翻訳先言語
    pub target_lang: Language,
    /// 翻訳にかかった時間（ミリ秒）
    pub duration_ms: u64,
}

/// 要約結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarizeResult {
    /// 要約テキスト
    pub summary: String,
    /// 元テキストの文字数
    pub original_length: usize,
    /// 要約テキストの文字数
    pub summary_length: usize,
    /// 処理時間（ミリ秒）
    pub duration_ms: u64,
}

/// 返信結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyResult {
    /// 返信テキスト（翻訳先言語）
    pub reply: String,
    /// 返信の説明（翻訳元言語）
    pub explanation: String,
    /// 返信の言語
    pub language: Language,
    /// 処理時間（ミリ秒）
    pub duration_ms: u64,
}

/// プロバイダー接続状態
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ProviderStatus {
    Available,
    #[serde(rename = "unavailable")]
    Unavailable {
        reason: String,
    },
}

/// 翻訳エラー
#[derive(Debug, Error)]
pub enum TranslationError {
    #[error("翻訳リクエストがタイムアウトしました")]
    Timeout,
    #[error("接続に失敗しました: {0}")]
    ConnectionFailed(String),
    #[error("APIエラー: {0}")]
    ApiError(String),
}

impl Serialize for TranslationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// モデル種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelType {
    /// PLaMo-2-Translate（翻訳特化モデル）
    PlamoTranslate,
    /// Qwen等の汎用LLM
    GeneralPurpose,
}

/// モデル名からモデル種別を判定
fn detect_model_type(model: &str) -> ModelType {
    let model_lower = model.to_lowercase();
    if model_lower.contains("plamo") && model_lower.contains("translate") {
        ModelType::PlamoTranslate
    } else {
        ModelType::GeneralPurpose
    }
}

/// モデル種別に応じたAPIパラメータを構築
fn build_api_options(model_type: ModelType) -> serde_json::Value {
    match model_type {
        ModelType::PlamoTranslate => {
            // PLaMo: 翻訳特化モデル向け設定
            serde_json::json!({
                "temperature": 0.1,      // 一貫性重視（翻訳特化モデルはより低く）
                "repeat_penalty": 1.4,   // 繰り返し防止を強化（評価レポート2より）
                "num_predict": 4096,     // 長文翻訳対応（途中終了防止）
            })
        }
        ModelType::GeneralPurpose => {
            // Qwen等: 汎用LLM向け設定
            serde_json::json!({
                "temperature": 0.2,      // 一貫性重視
                "repeat_penalty": 1.1,   // 繰り返し防止
                "num_predict": 4096,     // 長文翻訳対応（途中終了防止）
                "top_p": 0.9,            // 確率的サンプリング
            })
        }
    }
}

/// PLaMo-2-Translate用プロンプトを構築（シンプル）
fn build_plamo_prompt(text: &str, source_lang: Language, target_lang: Language) -> String {
    match (source_lang, target_lang) {
        (Language::Japanese, Language::English) => {
            format!(
                "Translate the following Japanese text to English:\n{}",
                text
            )
        }
        (Language::English, Language::Japanese) => {
            format!("以下の英文を日本語に翻訳してください:\n{}", text)
        }
        _ => {
            format!(
                "Translate from {} to {}:\n{}",
                source_lang.name(),
                target_lang.name(),
                text
            )
        }
    }
}

/// 汎用LLM用プロンプトを構築（シンプル）
fn build_general_prompt(text: &str, source_lang: Language, target_lang: Language) -> String {
    match (source_lang, target_lang) {
        (Language::Japanese, Language::English) => {
            // 日本語→英語: 超シンプル
            format!(
                "Translate the following Japanese text to English:\n{}",
                text
            )
        }
        (Language::English, Language::Japanese) => {
            // 英語→日本語: 超シンプル
            format!("以下の英文を日本語に翻訳してください:\n{}", text)
        }
        _ => {
            format!(
                "Translate from {} to {}:\n{}",
                source_lang.name(),
                target_lang.name(),
                text
            )
        }
    }
}

/// 翻訳用プロンプトを構築（モデルと言語方向に応じて最適化）
fn build_translation_prompt(
    text: &str,
    source_lang: Language,
    target_lang: Language,
    model: &str,
) -> String {
    let model_type = detect_model_type(model);
    match model_type {
        ModelType::PlamoTranslate => build_plamo_prompt(text, source_lang, target_lang),
        ModelType::GeneralPurpose => build_general_prompt(text, source_lang, target_lang),
    }
}

/// 翻訳結果をクリーニング
fn clean_translation_result(text: &str, source_text: &str) -> String {
    let mut result = text.trim().to_string();

    // 1. 先頭・末尾の引用符を除去
    let quotes = ['"', '「', '」', '『', '』', '\''];
    for &quote in &quotes {
        if result.starts_with(quote) {
            result = result.trim_start_matches(quote).trim().to_string();
        }
        if result.ends_with(quote) {
            result = result.trim_end_matches(quote).trim().to_string();
        }
    }

    // 2. プレフィックス除去パターン（大文字小文字無視）
    let prefixes_to_remove = [
        "translation:",
        "translated text:",
        "翻訳:",
        "翻訳結果:",
        "訳文:",
        "訳:",
        "english:",
        "japanese:",
        "日本語:",
        "英語:",
        "here is the translation:",
        "the translation is:",
    ];

    let result_lower = result.to_lowercase();
    for prefix in prefixes_to_remove {
        if result_lower.starts_with(prefix) {
            result = result[prefix.len()..].trim().to_string();
            break;
        }
    }

    // 3. 元のテキストが含まれている場合の除去
    // パターン1: "Original text\n\nTranslation" の形式
    if let Some(pos) = result.find("\n\n") {
        let first_part = &result[..pos];
        let second_part = &result[pos + 2..];

        // 最初の部分が元のテキストと類似している場合、2番目の部分のみを使用
        if first_part.contains(source_text) || source_text.contains(first_part) {
            result = second_part.trim().to_string();
        }
    }

    // パターン2: 元のテキストで始まる場合
    if result.starts_with(source_text) {
        result = result[source_text.len()..].trim().to_string();
    }

    result
}

/// Ollamaレスポンス（chat API）
#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
}

#[derive(Debug, Deserialize)]
struct OllamaMessage {
    content: String,
}

/// Ollamaストリーミングレスポンス
#[derive(Debug, Deserialize)]
struct OllamaStreamResponse {
    message: Option<OllamaMessage>,
    done: bool,
}

/// ストリーミングチャンクイベント
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamChunk {
    pub chunk: String,
    pub accumulated: String,
    pub done: bool,
}

/// ストリーミング完了イベント
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamComplete {
    pub translated_text: String,
    pub duration_ms: u64,
}

/// グローバルHTTPクライアント（コネクションプーリング）
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

/// HTTPクライアントを取得または初期化
fn get_http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(5)
            .build()
            .expect("Failed to create HTTP client")
    })
}

/// Ollamaで翻訳を実行
pub async fn translate_with_ollama(
    text: &str,
    source_lang: Language,
    target_lang: Language,
    endpoint: &str,
    model: &str,
) -> Result<TranslationResult, TranslationError> {
    let start = Instant::now();
    let client = get_http_client();

    // モデル種別を判定
    let model_type = detect_model_type(model);

    // プロンプト構築（モデルと言語方向に応じて最適化）
    let prompt = build_translation_prompt(text, source_lang, target_lang, model);
    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));

    // APIパラメータ構築
    let options = build_api_options(model_type);

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "stream": false,
        "options": options,
        "keep_alive": "10m"
    });

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                TranslationError::Timeout
            } else if e.is_connect() {
                TranslationError::ConnectionFailed(
                    "Ollamaが起動していません。Ollamaを起動してください。".to_string(),
                )
            } else {
                TranslationError::ConnectionFailed(e.to_string())
            }
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(TranslationError::ApiError(format!(
            "ステータス {}: {}",
            status, error_text
        )));
    }

    let chat_response: OllamaChatResponse = response
        .json()
        .await
        .map_err(|e| TranslationError::ApiError(format!("レスポンスのパースに失敗: {}", e)))?;

    let duration_ms = start.elapsed().as_millis() as u64;

    // ポストプロセシング（強化版）
    let translated = clean_translation_result(&chat_response.message.content, text);

    Ok(TranslationResult {
        translated_text: translated,
        source_lang,
        target_lang,
        duration_ms,
    })
}

/// Ollama接続状態を確認
pub async fn check_ollama_status(endpoint: &str) -> ProviderStatus {
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return ProviderStatus::Unavailable {
                reason: e.to_string(),
            }
        }
    };

    let url = format!("{}/api/tags", endpoint.trim_end_matches('/'));

    match client.get(&url).send().await {
        Ok(response) if response.status().is_success() => ProviderStatus::Available,
        Ok(response) => ProviderStatus::Unavailable {
            reason: format!("HTTPエラー: {}", response.status()),
        },
        Err(e) => ProviderStatus::Unavailable {
            reason: if e.is_connect() {
                "Ollamaが起動していません".to_string()
            } else if e.is_timeout() {
                "接続がタイムアウトしました".to_string()
            } else {
                e.to_string()
            },
        },
    }
}

/// Ollamaモデルをプリロード（ウォームアップ）
///
/// 空のリクエストを送信してモデルをメモリにロードし、
/// 初回翻訳時のレイテンシを削減する
pub async fn preload_ollama_model(endpoint: &str, model: &str) -> Result<(), String> {
    let client = get_http_client();

    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": "hi"
            }
        ],
        "stream": false,
        "keep_alive": "10m"
    });

    client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                "Ollamaが起動していません".to_string()
            } else if e.is_timeout() {
                "プリロードがタイムアウトしました".to_string()
            } else {
                e.to_string()
            }
        })?;

    Ok(())
}

/// ストリーミング翻訳を実行
pub async fn translate_with_ollama_stream<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    text: &str,
    source_lang: Language,
    target_lang: Language,
    endpoint: &str,
    model: &str,
) -> Result<(), TranslationError> {
    let start = Instant::now();
    let client = get_http_client();

    // プロンプト構築
    let prompt = build_translation_prompt(text, source_lang, target_lang, model);
    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));
    let model_type = detect_model_type(model);
    let options = build_api_options(model_type);

    let request_body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "stream": true,
        "options": options,
        "keep_alive": "10m"
    });

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                TranslationError::Timeout
            } else if e.is_connect() {
                TranslationError::ConnectionFailed(
                    "Ollamaが起動していません。Ollamaを起動してください。".to_string(),
                )
            } else {
                TranslationError::ConnectionFailed(e.to_string())
            }
        })?;

    let mut stream = response.bytes_stream();
    let mut accumulated = String::new();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| TranslationError::ConnectionFailed(e.to_string()))?;
        let line = String::from_utf8_lossy(&bytes);

        if let Ok(resp) = serde_json::from_str::<OllamaStreamResponse>(&line) {
            if let Some(msg) = resp.message {
                accumulated.push_str(&msg.content);

                // チャンクイベント発行
                let _ = app.emit(
                    "translation-chunk",
                    StreamChunk {
                        chunk: msg.content,
                        accumulated: accumulated.clone(),
                        done: resp.done,
                    },
                );
            }

            if resp.done {
                break;
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;
    let translated = clean_translation_result(&accumulated, text);

    // 完了イベント発行
    let _ = app.emit(
        "translation-complete",
        StreamComplete {
            translated_text: translated,
            duration_ms,
        },
    );

    Ok(())
}

/// 要約用プロンプトを構築（より具体的な指示で短い要約を生成）
fn build_summarize_prompt(text: &str, language: Language) -> String {
    match language {
        Language::Japanese => {
            format!(
                r#"以下のテキストを要約してください。

【ルール】
- 最大3文以内で要約すること
- 元のテキストの主旨・結論を優先すること
- 詳細や例は省略すること
- 要約のみを出力し、「要約:」などの接頭辞は不要

【テキスト】
{}"#,
                text
            )
        }
        Language::English => {
            format!(
                r#"Summarize the following text.

【Rules】
- Maximum 3 sentences
- Focus on the main point and conclusion
- Omit details and examples
- Output only the summary without any prefix like "Summary:"

【Text】
{}"#,
                text
            )
        }
    }
}

/// 返信用プロンプトを構築（ビジネス向け丁寧な文体 + 翻訳元言語での翻訳付き）
///
/// language: 返信を作成する言語（翻訳先言語）
/// source_language: 翻訳を作成する言語（翻訳元言語）
fn build_reply_prompt(text: &str, language: Language, source_language: Language) -> String {
    match language {
        Language::Japanese => {
            // 返信は日本語、翻訳は英語
            format!(
                r#"以下のメッセージに対して、ビジネス向けの丁寧で礼儀正しい日本語の返信を作成してください。

【出力形式】
以下の形式で出力してください:

[返信]
（ここに日本語の返信を記載）

[翻訳]
（ここに上記の日本語返信を英語に翻訳した内容を記載）

【メッセージ】
{}"#,
                text
            )
        }
        Language::English => {
            // 返信は英語、翻訳は日本語
            format!(
                r#"Create a polite and professional English business reply to the following message.

【Output Format】
Please output in the following format:

[Reply]
(Write the English reply here)

[Translation]
(Translate the above English reply into Japanese)

【Message】
{}"#,
                text
            )
        }
    }
}

/// 返信レスポンスをパースして返信と翻訳を分離
fn parse_reply_response(response: &str) -> (String, String) {
    let response = response.trim();

    // [返信] と [翻訳] または [Reply] と [Translation] を探す
    let reply_markers = ["[返信]", "[Reply]"];
    let translation_markers = ["[翻訳]", "[Translation]", "[説明]", "[Explanation]"]; // 後方互換性のため説明マーカーも残す

    let mut reply = String::new();
    let mut translation = String::new();

    // 返信部分を抽出
    for marker in &reply_markers {
        if let Some(start) = response.find(marker) {
            let after_marker = &response[start + marker.len()..];
            // 次のセクションまで、または終わりまで取得
            let end = translation_markers
                .iter()
                .filter_map(|m| after_marker.find(m))
                .min()
                .unwrap_or(after_marker.len());
            reply = after_marker[..end].trim().to_string();
            break;
        }
    }

    // 翻訳部分を抽出
    for marker in &translation_markers {
        if let Some(start) = response.find(marker) {
            translation = response[start + marker.len()..].trim().to_string();
            break;
        }
    }

    // マーカーが見つからない場合は、全体を返信として使用
    if reply.is_empty() && translation.is_empty() {
        return (response.to_string(), String::new());
    }

    (reply, translation)
}

/// Ollamaで要約を実行
pub async fn summarize_with_ollama(
    text: &str,
    language: Language,
    endpoint: &str,
    model: &str,
) -> Result<SummarizeResult, TranslationError> {
    let start = Instant::now();
    let client = get_http_client();

    // プロンプト構築
    let prompt = build_summarize_prompt(text, language);
    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));

    // モデル種別を判定してAPIパラメータ構築
    let model_type = detect_model_type(model);
    let options = build_api_options(model_type);

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "stream": false,
        "options": options,
        "keep_alive": "10m"
    });

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                TranslationError::Timeout
            } else if e.is_connect() {
                TranslationError::ConnectionFailed(
                    "Ollamaが起動していません。Ollamaを起動してください。".to_string(),
                )
            } else {
                TranslationError::ConnectionFailed(e.to_string())
            }
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(TranslationError::ApiError(format!(
            "ステータス {}: {}",
            status, error_text
        )));
    }

    let chat_response: OllamaChatResponse = response
        .json()
        .await
        .map_err(|e| TranslationError::ApiError(format!("レスポンスのパースに失敗: {}", e)))?;

    let duration_ms = start.elapsed().as_millis() as u64;

    // クリーニング
    let summary = clean_translation_result(&chat_response.message.content, text);

    Ok(SummarizeResult {
        summary: summary.clone(),
        original_length: text.chars().count(),
        summary_length: summary.chars().count(),
        duration_ms,
    })
}

/// Ollamaで返信を生成
///
/// language: 返信を作成する言語（翻訳先言語）
/// source_language: 説明を作成する言語（翻訳元言語）
pub async fn generate_reply_with_ollama(
    text: &str,
    language: Language,
    source_language: Language,
    endpoint: &str,
    model: &str,
) -> Result<ReplyResult, TranslationError> {
    let start = Instant::now();
    let client = get_http_client();

    // プロンプト構築（返信言語と説明言語を指定）
    let prompt = build_reply_prompt(text, language, source_language);
    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));

    // モデル種別を判定してAPIパラメータ構築
    let model_type = detect_model_type(model);
    let options = build_api_options(model_type);

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "stream": false,
        "options": options,
        "keep_alive": "10m"
    });

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                TranslationError::Timeout
            } else if e.is_connect() {
                TranslationError::ConnectionFailed(
                    "Ollamaが起動していません。Ollamaを起動してください。".to_string(),
                )
            } else {
                TranslationError::ConnectionFailed(e.to_string())
            }
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(TranslationError::ApiError(format!(
            "ステータス {}: {}",
            status, error_text
        )));
    }

    let chat_response: OllamaChatResponse = response
        .json()
        .await
        .map_err(|e| TranslationError::ApiError(format!("レスポンスのパースに失敗: {}", e)))?;

    let duration_ms = start.elapsed().as_millis() as u64;

    // レスポンスをパースして返信と翻訳を分離
    let (reply, translation) = parse_reply_response(&chat_response.message.content);

    Ok(ReplyResult {
        reply,
        explanation: translation, // 翻訳を explanation フィールドに格納
        language,
        duration_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_name() {
        assert_eq!(Language::Japanese.name(), "Japanese");
        assert_eq!(Language::English.name(), "English");
    }

    #[test]
    fn test_detect_model_type_plamo() {
        assert_eq!(
            detect_model_type("mitmul/plamo-2-translate:Q4_K_M"),
            ModelType::PlamoTranslate
        );
        assert_eq!(
            detect_model_type("mitmul/plamo-2-translate:Q2_K_S"),
            ModelType::PlamoTranslate
        );
    }

    #[test]
    fn test_detect_model_type_general() {
        assert_eq!(detect_model_type("qwen2.5:3b"), ModelType::GeneralPurpose);
        assert_eq!(detect_model_type("llama2:7b"), ModelType::GeneralPurpose);
    }

    #[test]
    fn test_build_plamo_prompt_ja_to_en() {
        let prompt = build_plamo_prompt("こんにちは", Language::Japanese, Language::English);
        assert!(prompt.contains("こんにちは"));
        assert!(prompt.contains("English"));
    }

    #[test]
    fn test_build_plamo_prompt_en_to_ja() {
        let prompt = build_plamo_prompt("Hello", Language::English, Language::Japanese);
        assert!(prompt.contains("Hello"));
        assert!(prompt.contains("日本語"));
    }

    #[test]
    fn test_build_general_prompt_ja_to_en() {
        let prompt = build_general_prompt("こんにちは", Language::Japanese, Language::English);
        assert!(prompt.contains("こんにちは"));
        assert!(prompt.contains("Translate"));
        assert!(prompt.contains("English"));
    }

    #[test]
    fn test_build_general_prompt_en_to_ja() {
        let prompt = build_general_prompt("Hello", Language::English, Language::Japanese);
        assert!(prompt.contains("Hello"));
        assert!(prompt.contains("翻訳"));
    }

    #[test]
    fn test_build_translation_prompt_qwen() {
        let prompt =
            build_translation_prompt("Hello", Language::English, Language::Japanese, "qwen2.5:3b");
        assert!(prompt.contains("Hello"));
        assert!(prompt.contains("翻訳"));
    }

    #[test]
    fn test_build_translation_prompt_plamo() {
        let prompt = build_translation_prompt(
            "こんにちは",
            Language::Japanese,
            Language::English,
            "mitmul/plamo-2-translate:Q4_K_M",
        );
        assert!(prompt.contains("こんにちは"));
        assert!(prompt.contains("English"));
    }

    #[test]
    fn test_clean_translation_result_prefix_removal() {
        assert_eq!(
            clean_translation_result("Translation: Hello world", "こんにちは"),
            "Hello world"
        );
        assert_eq!(
            clean_translation_result("翻訳: こんにちは世界", "Hello"),
            "こんにちは世界"
        );
    }

    #[test]
    fn test_clean_translation_result_quote_removal() {
        assert_eq!(
            clean_translation_result("\"Hello world\"", "こんにちは"),
            "Hello world"
        );
        assert_eq!(
            clean_translation_result("「こんにちは世界」", "Hello"),
            "こんにちは世界"
        );
    }

    #[test]
    fn test_clean_translation_result_original_text_removal() {
        let result = clean_translation_result("こんにちは\n\nHello", "こんにちは");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_build_api_options_plamo() {
        let options = build_api_options(ModelType::PlamoTranslate);
        assert_eq!(options["temperature"], 0.1);
        assert_eq!(options["repeat_penalty"], 1.4);
        assert_eq!(options["num_predict"], 4096);
    }

    #[test]
    fn test_build_api_options_general() {
        let options = build_api_options(ModelType::GeneralPurpose);
        assert_eq!(options["temperature"], 0.2);
        assert_eq!(options["repeat_penalty"], 1.1);
        assert_eq!(options["num_predict"], 4096);
    }

    #[test]
    fn test_translation_error_display() {
        let err = TranslationError::Timeout;
        assert_eq!(err.to_string(), "翻訳リクエストがタイムアウトしました");

        let err = TranslationError::ConnectionFailed("接続エラー".to_string());
        assert_eq!(err.to_string(), "接続に失敗しました: 接続エラー");
    }

    #[test]
    fn test_translation_error_serialization() {
        let err = TranslationError::Timeout;
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"翻訳リクエストがタイムアウトしました\"");
    }

    #[test]
    fn test_translation_result_serialization() {
        let result = TranslationResult {
            translated_text: "こんにちは".to_string(),
            source_lang: Language::English,
            target_lang: Language::Japanese,
            duration_ms: 500,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"translatedText\""));
        assert!(json.contains("\"sourceLang\""));
        assert!(json.contains("\"targetLang\""));
        assert!(json.contains("\"durationMs\""));
    }

    #[test]
    fn test_provider_status_serialization() {
        let status = ProviderStatus::Available;
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"available\""));

        let status = ProviderStatus::Unavailable {
            reason: "接続エラー".to_string(),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"unavailable\""));
        assert!(json.contains("\"reason\":\"接続エラー\""));
    }

    #[test]
    fn test_language_serialization() {
        let lang = Language::Japanese;
        let json = serde_json::to_string(&lang).unwrap();
        assert_eq!(json, "\"japanese\"");

        let lang = Language::English;
        let json = serde_json::to_string(&lang).unwrap();
        assert_eq!(json, "\"english\"");
    }

    #[test]
    fn test_language_deserialization() {
        let lang: Language = serde_json::from_str("\"japanese\"").unwrap();
        assert_eq!(lang, Language::Japanese);

        let lang: Language = serde_json::from_str("\"english\"").unwrap();
        assert_eq!(lang, Language::English);
    }

    #[test]
    fn test_build_summarize_prompt_japanese() {
        let prompt = build_summarize_prompt("テストテキスト", Language::Japanese);
        assert!(prompt.contains("要約"));
        assert!(prompt.contains("テストテキスト"));
        assert!(prompt.contains("簡潔"));
    }

    #[test]
    fn test_build_summarize_prompt_english() {
        let prompt = build_summarize_prompt("Test text", Language::English);
        assert!(prompt.contains("Summarize"));
        assert!(prompt.contains("Test text"));
        assert!(prompt.contains("concisely"));
    }

    #[test]
    fn test_build_reply_prompt_japanese() {
        let prompt = build_reply_prompt("こんにちは", Language::Japanese);
        assert!(prompt.contains("返信"));
        assert!(prompt.contains("こんにちは"));
        assert!(prompt.contains("ビジネス"));
        assert!(prompt.contains("丁寧"));
    }

    #[test]
    fn test_build_reply_prompt_english() {
        let prompt = build_reply_prompt("Hello", Language::English);
        assert!(prompt.contains("reply"));
        assert!(prompt.contains("Hello"));
        assert!(prompt.contains("polite"));
        assert!(prompt.contains("professional"));
    }

    #[test]
    fn test_summarize_result_serialization() {
        let result = SummarizeResult {
            summary: "要約テキスト".to_string(),
            original_length: 100,
            summary_length: 20,
            duration_ms: 500,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"summary\""));
        assert!(json.contains("\"originalLength\""));
        assert!(json.contains("\"summaryLength\""));
        assert!(json.contains("\"durationMs\""));
    }

    #[test]
    fn test_reply_result_serialization() {
        let result = ReplyResult {
            reply: "返信テキスト".to_string(),
            language: Language::Japanese,
            duration_ms: 500,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"reply\""));
        assert!(json.contains("\"language\""));
        assert!(json.contains("\"durationMs\""));
        assert!(json.contains("\"japanese\""));
    }
}
