//! 設定管理サービス
//!
//! アプリケーション設定の永続化とデフォルト値管理を提供

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// グローバルショートカット
    pub shortcut: String,
    /// Ollamaモデル名
    pub ollama_model: String,
    /// Ollamaエンドポイント
    pub ollama_endpoint: String,
    /// 翻訳プロバイダー ("ollama" or "claude-cli")
    #[serde(default = "default_provider")]
    pub provider: String,
    /// Claude CLIの実行パス
    #[serde(default)]
    pub claude_cli_path: Option<String>,
}

/// providerフィールドのデフォルト値
fn default_provider() -> String {
    "ollama".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            shortcut: "CommandOrControl+J".to_string(),
            ollama_model: "qwen2.5:3b".to_string(),
            ollama_endpoint: "http://localhost:11434".to_string(),
            provider: default_provider(),
            claude_cli_path: None,
        }
    }
}

/// 設定エラー
#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("設定の読み込みに失敗しました: {0}")]
    LoadFailed(String),
    #[error("設定の保存に失敗しました: {0}")]
    SaveFailed(String),
    #[error("設定ストアが初期化されていません")]
    #[allow(dead_code)]
    StoreNotInitialized,
    #[allow(dead_code)]
    #[error("シリアライズエラー: {0}")]
    SerializationError(String),
}

impl Serialize for SettingsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert_eq!(settings.shortcut, "CommandOrControl+J");
        assert_eq!(settings.ollama_model, "qwen2.5:3b");
        assert_eq!(settings.ollama_endpoint, "http://localhost:11434");
        assert_eq!(settings.provider, "ollama");
        assert_eq!(settings.claude_cli_path, None);
    }

    #[test]
    fn test_settings_serialization() {
        let settings = AppSettings::default();
        let json = serde_json::to_string(&settings).unwrap();

        // camelCaseでシリアライズされていることを確認
        assert!(json.contains("\"ollamaModel\""));
        assert!(json.contains("\"ollamaEndpoint\""));
    }

    #[test]
    fn test_settings_deserialization() {
        let json = r#"{
            "shortcut": "CommandOrControl+Shift+X",
            "ollamaModel": "llama2",
            "ollamaEndpoint": "http://localhost:8080"
        }"#;

        let settings: AppSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.shortcut, "CommandOrControl+Shift+X");
        assert_eq!(settings.ollama_model, "llama2");
        assert_eq!(settings.ollama_endpoint, "http://localhost:8080");
    }

    #[test]
    fn test_settings_error_display() {
        let err = SettingsError::LoadFailed("ファイルが見つかりません".to_string());
        assert_eq!(
            err.to_string(),
            "設定の読み込みに失敗しました: ファイルが見つかりません"
        );

        let err = SettingsError::StoreNotInitialized;
        assert_eq!(err.to_string(), "設定ストアが初期化されていません");
    }

    #[test]
    fn test_provider_default_value() {
        // providerフィールドのデフォルト値検証
        let settings = AppSettings::default();
        assert_eq!(settings.provider, "ollama");
    }

    #[test]
    fn test_claude_cli_path_default_value() {
        // claude_cli_pathフィールドのデフォルト値検証
        let settings = AppSettings::default();
        assert_eq!(settings.claude_cli_path, None);
    }

    #[test]
    fn test_backward_compatibility_deserialization() {
        // 既存の設定ファイル（providerとclaude_cli_pathがない）からのデシリアライズ検証
        let json = r#"{
            "shortcut": "CommandOrControl+J",
            "ollamaModel": "qwen2.5:3b",
            "ollamaEndpoint": "http://localhost:11434"
        }"#;

        let settings: AppSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.provider, "ollama"); // デフォルト値が適用される
        assert_eq!(settings.claude_cli_path, None); // デフォルト値が適用される
    }

    #[test]
    fn test_new_settings_serialization() {
        // 新しいフィールドを含む設定のシリアライズ検証
        let settings = AppSettings {
            shortcut: "CommandOrControl+J".to_string(),
            ollama_model: "qwen2.5:3b".to_string(),
            ollama_endpoint: "http://localhost:11434".to_string(),
            provider: "claude-cli".to_string(),
            claude_cli_path: Some("/opt/homebrew/bin/claude".to_string()),
        };

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"provider\""));
        assert!(json.contains("\"claude-cli\""));
        assert!(json.contains("\"claudeCliPath\""));
        assert!(json.contains("/opt/homebrew/bin/claude"));
    }

    #[test]
    fn test_new_settings_deserialization() {
        // 新しいフィールドを含む設定のデシリアライズ検証
        let json = r#"{
            "shortcut": "CommandOrControl+J",
            "ollamaModel": "qwen2.5:3b",
            "ollamaEndpoint": "http://localhost:11434",
            "provider": "claude-cli",
            "claudeCliPath": "/usr/local/bin/claude"
        }"#;

        let settings: AppSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.provider, "claude-cli");
        assert_eq!(
            settings.claude_cli_path,
            Some("/usr/local/bin/claude".to_string())
        );
    }
}
