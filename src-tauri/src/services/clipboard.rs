//! クリップボード管理サービス
//!
//! クリップボードの読み取り・書き込みと、元のクリップボード内容の保存・復元を提供

use serde::Serialize;
use thiserror::Error;

/// クリップボードエラー
#[derive(Debug, Error)]
pub enum ClipboardError {
    #[error("クリップボードの読み取りに失敗しました: {0}")]
    ReadFailed(String),
    #[error("クリップボードへの書き込みに失敗しました: {0}")]
    WriteFailed(String),
    #[error("クリップボードが空です")]
    Empty,
    #[error("クリップボードにテキストがありません")]
    NoText,
}

impl Serialize for ClipboardError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// クリップボード読み取り結果
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardContent {
    /// テキスト内容
    pub text: String,
    /// 読み取り成功かどうか
    pub success: bool,
}

impl ClipboardContent {
    /// 空のクリップボードコンテンツを作成
    pub fn empty() -> Self {
        Self {
            text: String::new(),
            success: false,
        }
    }

    /// テキストからクリップボードコンテンツを作成
    pub fn from_text(text: String) -> Self {
        Self {
            text,
            success: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_error_display() {
        let err = ClipboardError::ReadFailed("テスト".to_string());
        assert_eq!(err.to_string(), "クリップボードの読み取りに失敗しました: テスト");

        let err = ClipboardError::WriteFailed("テスト".to_string());
        assert_eq!(err.to_string(), "クリップボードへの書き込みに失敗しました: テスト");

        let err = ClipboardError::Empty;
        assert_eq!(err.to_string(), "クリップボードが空です");

        let err = ClipboardError::NoText;
        assert_eq!(err.to_string(), "クリップボードにテキストがありません");
    }

    #[test]
    fn test_clipboard_error_serialization() {
        let err = ClipboardError::Empty;
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"クリップボードが空です\"");
    }

    #[test]
    fn test_clipboard_content_empty() {
        let content = ClipboardContent::empty();
        assert!(content.text.is_empty());
        assert!(!content.success);
    }

    #[test]
    fn test_clipboard_content_from_text() {
        let content = ClipboardContent::from_text("Hello".to_string());
        assert_eq!(content.text, "Hello");
        assert!(content.success);
    }

    #[test]
    fn test_clipboard_content_serialization() {
        let content = ClipboardContent::from_text("テスト".to_string());
        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"text\":\"テスト\""));
        assert!(json.contains("\"success\":true"));
    }
}
