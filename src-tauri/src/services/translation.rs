//! 翻訳サービス
//!
//! Ollama APIを使用したテキスト翻訳機能を提供

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
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

/// 翻訳用プロンプトを構築
fn build_translation_prompt(text: &str, source_lang: Language, target_lang: Language) -> String {
    format!(
        r#"You are a professional translator. Translate the following text from {} to {}.

Rules:
- Output ONLY the translation, nothing else
- Preserve the original tone and style
- Keep technical terms accurate
- Maintain formatting (line breaks, punctuation)

Text to translate:
{}"#,
        source_lang.name(),
        target_lang.name(),
        text
    )
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

/// Ollamaで翻訳を実行
pub async fn translate_with_ollama(
    text: &str,
    source_lang: Language,
    target_lang: Language,
    endpoint: &str,
    model: &str,
) -> Result<TranslationResult, TranslationError> {
    let start = Instant::now();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| TranslationError::ConnectionFailed(e.to_string()))?;

    let prompt = build_translation_prompt(text, source_lang, target_lang);
    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "stream": false
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

    Ok(TranslationResult {
        translated_text: chat_response.message.content.trim().to_string(),
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
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": "hi"
            }
        ],
        "stream": false
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_name() {
        assert_eq!(Language::Japanese.name(), "Japanese");
        assert_eq!(Language::English.name(), "English");
    }

    #[test]
    fn test_build_translation_prompt() {
        let prompt = build_translation_prompt("Hello", Language::English, Language::Japanese);
        assert!(prompt.contains("English"));
        assert!(prompt.contains("Japanese"));
        assert!(prompt.contains("Hello"));
        assert!(prompt.contains("ONLY the translation"));
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
}
