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
    /// 例: 日→英翻訳の場合は英語の返信
    pub reply: String,
    /// 返信の翻訳（翻訳元言語）
    /// 例: 日→英翻訳の場合は日本語の返信（上記replyを翻訳したもの）
    pub explanation: String,
    /// 返信の言語（翻訳先言語）
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
    #[error("このモデルは要約・返信機能に対応していません。7B以上のモデルを使用してください（現在: {0}）")]
    ModelTooSmall(String),
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

/// モデル名からパラメータサイズ（B単位）を抽出
/// 例: "qwen2.5:3b" -> Some(3), "qwen2.5:7b" -> Some(7), "unknown" -> None
fn extract_model_size(model: &str) -> Option<u32> {
    let model_lower = model.to_lowercase();

    // "3b", "7b", "14b" などのパターンを検索
    // コロンの後か、ハイフンの後に続く数字+bのパターン
    let patterns = [
        // "model:3b" パターン
        (r":", r"b"),
        // "model-3b" パターン
        (r"-", r"b"),
        // "model_3b" パターン
        (r"_", r"b"),
    ];

    for (prefix, _suffix) in patterns {
        if let Some(pos) = model_lower.find(prefix) {
            let after_prefix = &model_lower[pos + prefix.len()..];
            // 数字を抽出
            let mut num_str = String::new();
            for ch in after_prefix.chars() {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                } else if ch == 'b' && !num_str.is_empty() {
                    // 数字の後にbが続く場合は成功
                    if let Ok(size) = num_str.parse::<u32>() {
                        return Some(size);
                    }
                    break;
                } else if !num_str.is_empty() {
                    // 数字の後に他の文字が続く場合は失敗
                    break;
                }
            }
        }
    }

    None
}

/// 要約・返信機能に必要な最小モデルサイズ（B単位）
const MIN_MODEL_SIZE_FOR_ADVANCED_FEATURES: u32 = 7;

/// モデルが要約・返信機能に対応しているか検証
fn validate_model_for_advanced_features(model: &str) -> Result<(), TranslationError> {
    match extract_model_size(model) {
        Some(size) if size >= MIN_MODEL_SIZE_FOR_ADVANCED_FEATURES => Ok(()),
        Some(size) => Err(TranslationError::ModelTooSmall(
            format!("{}B（最小要件: {}B以上）", size, MIN_MODEL_SIZE_FOR_ADVANCED_FEATURES)
        )),
        None => {
            // サイズが抽出できない場合は警告せずに続行
            // （カスタムモデルや特殊な命名規則に対応）
            Ok(())
        }
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

/// 要約用プロンプトを構築（極限までシンプル化）
fn build_summarize_prompt(text: &str, language: Language) -> String {
    match language {
        Language::Japanese => {
            format!(
                "以下の日本語テキストを3文以内で日本語で要約してください。要約のみを出力してください。\n\n{}",
                text
            )
        }
        Language::English => {
            format!(
                "Summarize the following English text in 3 sentences or less in English. Output only the summary.\n\n{}",
                text
            )
        }
    }
}

/// 返信用プロンプトを構築（極限までシンプル化）
///
/// language: 返信を作成する言語
/// _source_language: 未使用（後方互換性のため保持）
fn build_reply_prompt(text: &str, language: Language, _source_language: Language) -> String {
    match language {
        Language::Japanese => {
            format!(
                "以下の日本語メッセージに対して、丁寧なビジネスメールの返信を日本語で書いてください。返信のみを出力してください。\n\n{}",
                text
            )
        }
        Language::English => {
            format!(
                "Write a polite business email reply to the following English message in English. Output only the reply.\n\n{}",
                text
            )
        }
    }
}

/// 返信レスポンスをパースして返信と翻訳を分離
/// 注: 2段階処理実装後は未使用。テストのために保持。
#[allow(dead_code)]
fn parse_reply_response(response: &str) -> (String, String) {
    let response = response.trim();

    // 新形式のマーカー（REPLY:、TRANSLATION:）を優先的に探す
    let reply_markers = ["REPLY:", "Reply:", "reply:", "[返信]", "[Reply]"];
    let translation_markers = ["TRANSLATION:", "Translation:", "translation:", "[翻訳]", "[Translation]", "[説明]", "[Explanation]"];

    let mut reply = String::new();
    let mut translation = String::new();

    // 返信部分を抽出
    for marker in &reply_markers {
        if let Some(start) = response.find(marker) {
            let after_marker = &response[start + marker.len()..];
            // 次のセクション（TRANSLATION:など）までを取得
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

    // マーカーが見つからない場合
    if reply.is_empty() && translation.is_empty() {
        eprintln!("[WARNING] 返信レスポンスのパースに失敗しました。マーカーが見つかりませんでした。");
        eprintln!("[WARNING] レスポンス全文: {}", response);

        // 行ごとに分割して、最初の2行を返信と翻訳として扱う
        let lines: Vec<&str> = response.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.len() >= 2 {
            eprintln!("[INFO] フォールバック: 最初の2行を返信と翻訳として使用");
            return (lines[0].trim().to_string(), lines[1].trim().to_string());
        } else if lines.len() == 1 {
            eprintln!("[INFO] フォールバック: 最初の1行のみを返信として使用");
            return (lines[0].trim().to_string(), String::new());
        } else {
            return (response.to_string(), String::new());
        }
    }

    // 返信または翻訳が空の場合もログに記録
    if reply.is_empty() {
        eprintln!("[WARNING] 返信部分が空です");
    }
    if translation.is_empty() {
        eprintln!("[WARNING] 翻訳部分が空です");
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
    // モデルサイズ検証
    validate_model_for_advanced_features(model)?;

    let start = Instant::now();
    let client = get_http_client();

    // デバッグログ
    eprintln!("[要約] デバッグ情報:");
    eprintln!("  text length: {}", text.len());
    eprintln!("  language: {:?}", language);

    // プロンプト構築
    let prompt = build_summarize_prompt(text, language);
    let preview = prompt.chars().take(200).collect::<String>();
    eprintln!("  prompt preview: {}", preview);
    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));

    // モデル種別を判定してAPIパラメータ構築
    let model_type = detect_model_type(model);
    let options = build_api_options(model_type);

    // システムメッセージ（言語固定の指示）
    let system_message = match language {
        Language::Japanese => "あなたは日本語の要約専門家です。必ず日本語でのみ応答してください。絶対に英語に翻訳しないでください。",
        Language::English => "You are an English summarization expert. You MUST respond in English only. DO NOT translate to Japanese.",
    };

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": system_message
            },
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

    // デバッグログ: レスポンス
    eprintln!("[要約] レスポンス:");
    eprintln!("  content: {}", &chat_response.message.content);

    // クリーニング
    let summary = clean_translation_result(&chat_response.message.content, text);
    eprintln!("[要約] クリーニング後:");
    eprintln!("  summary: {}", &summary);

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
    // モデルサイズ検証
    validate_model_for_advanced_features(model)?;

    let start = Instant::now();
    let client = get_http_client();

    // デバッグログ
    eprintln!("[返信生成] デバッグ情報:");
    eprintln!("  text length: {}", text.len());
    eprintln!("  language: {:?}", language);
    eprintln!("  source_language: {:?}", source_language);

    // プロンプト構築（返信言語と説明言語を指定）
    let prompt = build_reply_prompt(text, language, source_language);
    let preview = prompt.chars().take(200).collect::<String>();
    eprintln!("  prompt preview: {}", preview);
    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));

    // モデル種別を判定してAPIパラメータ構築
    let model_type = detect_model_type(model);
    let options = build_api_options(model_type);

    // システムメッセージ（単一言語の返信のみ）
    let system_message = match language {
        Language::Japanese => "あなたはビジネスメールの返信作成専門家です。必ず日本語でのみ返信を作成してください。",
        Language::English => "You are a business email reply expert. You MUST write the reply in English only.",
    };

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": system_message
            },
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

    // デバッグログ: レスポンス
    eprintln!("[返信生成] レスポンス:");
    eprintln!("  content: {}", &chat_response.message.content);

    // クリーニング（簡易版）
    let reply = clean_translation_result(&chat_response.message.content, text);

    eprintln!("[返信生成] クリーニング後:");
    eprintln!("  reply: {}", &reply);

    Ok(ReplyResult {
        reply: reply.clone(),
        explanation: reply, // 2段階処理では翻訳はフロントエンドで実施するため、同じ内容を格納
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
    fn test_extract_model_size() {
        // コロンパターン
        assert_eq!(extract_model_size("qwen2.5:3b"), Some(3));
        assert_eq!(extract_model_size("qwen2.5:7b"), Some(7));
        assert_eq!(extract_model_size("llama2:14b"), Some(14));
        assert_eq!(extract_model_size("model:32b"), Some(32));

        // ハイフンパターン
        assert_eq!(extract_model_size("model-3b"), Some(3));
        assert_eq!(extract_model_size("model-7b"), Some(7));

        // アンダースコアパターン
        assert_eq!(extract_model_size("model_3b"), Some(3));
        assert_eq!(extract_model_size("model_7b"), Some(7));

        // サイズが抽出できないケース
        assert_eq!(extract_model_size("unknown"), None);
        assert_eq!(extract_model_size("model"), None);
        assert_eq!(extract_model_size("model:latest"), None);
    }

    #[test]
    fn test_validate_model_for_advanced_features() {
        // 7B以上は成功
        assert!(validate_model_for_advanced_features("qwen2.5:7b").is_ok());
        assert!(validate_model_for_advanced_features("qwen2.5:14b").is_ok());
        assert!(validate_model_for_advanced_features("qwen2.5:32b").is_ok());

        // 3B以下は失敗
        assert!(validate_model_for_advanced_features("qwen2.5:3b").is_err());
        assert!(validate_model_for_advanced_features("model:1b").is_err());

        // サイズが抽出できない場合は成功（カスタムモデル対応）
        assert!(validate_model_for_advanced_features("custom-model").is_ok());
        assert!(validate_model_for_advanced_features("unknown").is_ok());
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
        assert!(prompt.contains("3文"));
    }

    #[test]
    fn test_build_summarize_prompt_english() {
        let prompt = build_summarize_prompt("Test text", Language::English);
        assert!(prompt.contains("Summarize"));
        assert!(prompt.contains("Test text"));
        assert!(prompt.contains("3 sentences"));
    }

    #[test]
    fn test_build_reply_prompt_japanese() {
        let prompt = build_reply_prompt("こんにちは", Language::Japanese, Language::English);
        assert!(prompt.contains("返信"));
        assert!(prompt.contains("こんにちは"));
        assert!(prompt.contains("丁寧"));
    }

    #[test]
    fn test_build_reply_prompt_english() {
        let prompt = build_reply_prompt("Hello", Language::English, Language::Japanese);
        assert!(prompt.contains("reply"));
        assert!(prompt.contains("Hello"));
        assert!(prompt.contains("polite"));
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
            explanation: "Reply text".to_string(),
            language: Language::Japanese,
            duration_ms: 500,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"reply\""));
        assert!(json.contains("\"explanation\""));
        assert!(json.contains("\"language\""));
        assert!(json.contains("\"durationMs\""));
        assert!(json.contains("\"japanese\""));
    }

    #[test]
    fn test_parse_reply_response_with_markers() {
        let response = r#"[返信]
こちらは日本語の返信です。

[翻訳]
This is the English translation."#;

        let (reply, translation) = parse_reply_response(response);
        assert_eq!(reply, "こちらは日本語の返信です。");
        assert_eq!(translation, "This is the English translation.");
    }

    #[test]
    fn test_parse_reply_response_english_markers() {
        let response = r#"[Reply]
This is an English reply.

[Translation]
これは日本語の翻訳です。"#;

        let (reply, translation) = parse_reply_response(response);
        assert_eq!(reply, "This is an English reply.");
        assert_eq!(translation, "これは日本語の翻訳です。");
    }

    #[test]
    fn test_parse_reply_response_no_markers() {
        let response = "This is a reply without markers.";

        let (reply, translation) = parse_reply_response(response);
        assert_eq!(reply, "This is a reply without markers.");
        assert_eq!(translation, "");
    }

    #[test]
    fn test_parse_reply_response_new_format_japanese() {
        let response = r#"REPLY: ご連絡ありがとうございます。承知いたしました。
TRANSLATION: Thank you for your message. I understand."#;

        let (reply, translation) = parse_reply_response(response);
        assert_eq!(reply, "ご連絡ありがとうございます。承知いたしました。");
        assert_eq!(translation, "Thank you for your message. I understand.");
    }

    #[test]
    fn test_parse_reply_response_new_format_english() {
        let response = r#"REPLY: Thank you for your message. I understand.
TRANSLATION: ご連絡ありがとうございます。承知いたしました。"#;

        let (reply, translation) = parse_reply_response(response);
        assert_eq!(reply, "Thank you for your message. I understand.");
        assert_eq!(translation, "ご連絡ありがとうございます。承知いたしました。");
    }

    #[test]
    fn test_parse_reply_response_fallback_two_lines() {
        let response = "First line as reply\nSecond line as translation";

        let (reply, translation) = parse_reply_response(response);
        assert_eq!(reply, "First line as reply");
        assert_eq!(translation, "Second line as translation");
    }
}
