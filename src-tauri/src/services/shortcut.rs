//! ショートカット管理サービス
//!
//! グローバルショートカットの登録・解除・管理を提供

use serde::Serialize;
use thiserror::Error;

/// ショートカットエラー
#[derive(Debug, Error)]
pub enum ShortcutError {
    #[error("ショートカットの登録に失敗しました: {0}")]
    RegistrationFailed(String),
    #[error("ショートカットの解除に失敗しました: {0}")]
    UnregistrationFailed(String),
    #[error("無効なショートカット形式です: {0}")]
    InvalidFormat(String),
    #[error("ショートカットが競合しています: {0}")]
    Conflict(String),
}

impl Serialize for ShortcutError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// ショートカット登録状態
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutStatus {
    /// 現在登録されているショートカット
    pub current_shortcut: Option<String>,
    /// 登録されているかどうか
    pub is_registered: bool,
}

impl Default for ShortcutStatus {
    fn default() -> Self {
        Self {
            current_shortcut: None,
            is_registered: false,
        }
    }
}

/// ショートカット文字列を検証する
///
/// # Arguments
/// * `shortcut` - 検証するショートカット文字列（例: "CommandOrControl+Shift+T"）
///
/// # Returns
/// 有効な場合はOk(())、無効な場合はエラー
pub fn validate_shortcut(shortcut: &str) -> Result<(), ShortcutError> {
    if shortcut.is_empty() {
        return Err(ShortcutError::InvalidFormat(
            "ショートカットが空です".to_string(),
        ));
    }

    // 基本的な形式チェック
    let parts: Vec<&str> = shortcut.split('+').collect();
    if parts.is_empty() {
        return Err(ShortcutError::InvalidFormat(
            "無効な形式です".to_string(),
        ));
    }

    // 最後のパートがキーである必要がある
    let key = parts.last().unwrap();
    if key.is_empty() {
        return Err(ShortcutError::InvalidFormat(
            "キーが指定されていません".to_string(),
        ));
    }

    // 有効な修飾キー
    let valid_modifiers = [
        "Command",
        "Cmd",
        "Control",
        "Ctrl",
        "CommandOrControl",
        "CmdOrCtrl",
        "Alt",
        "Option",
        "Shift",
        "Super",
        "Meta",
    ];

    // 修飾キーの検証（最後のパート以外）
    for part in &parts[..parts.len() - 1] {
        let is_valid_modifier = valid_modifiers
            .iter()
            .any(|m| m.eq_ignore_ascii_case(part));
        if !is_valid_modifier {
            return Err(ShortcutError::InvalidFormat(format!(
                "無効な修飾キー: {}",
                part
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_status_default() {
        let status = ShortcutStatus::default();
        assert!(status.current_shortcut.is_none());
        assert!(!status.is_registered);
    }

    #[test]
    fn test_shortcut_error_display() {
        let err = ShortcutError::RegistrationFailed("test".to_string());
        assert_eq!(err.to_string(), "ショートカットの登録に失敗しました: test");

        let err = ShortcutError::UnregistrationFailed("test".to_string());
        assert_eq!(err.to_string(), "ショートカットの解除に失敗しました: test");

        let err = ShortcutError::InvalidFormat("test".to_string());
        assert_eq!(err.to_string(), "無効なショートカット形式です: test");

        let err = ShortcutError::Conflict("Cmd+C".to_string());
        assert_eq!(err.to_string(), "ショートカットが競合しています: Cmd+C");
    }

    #[test]
    fn test_shortcut_error_serialization() {
        let err = ShortcutError::RegistrationFailed("test error".to_string());
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"ショートカットの登録に失敗しました: test error\"");
    }

    #[test]
    fn test_validate_shortcut_valid() {
        assert!(validate_shortcut("CommandOrControl+Shift+T").is_ok());
        assert!(validate_shortcut("Cmd+Shift+T").is_ok());
        assert!(validate_shortcut("Control+Alt+X").is_ok());
        assert!(validate_shortcut("Shift+F1").is_ok());
        assert!(validate_shortcut("Command+A").is_ok());
    }

    #[test]
    fn test_validate_shortcut_empty() {
        let result = validate_shortcut("");
        assert!(result.is_err());
        if let Err(ShortcutError::InvalidFormat(msg)) = result {
            assert!(msg.contains("空"));
        }
    }

    #[test]
    fn test_validate_shortcut_invalid_modifier() {
        let result = validate_shortcut("InvalidModifier+T");
        assert!(result.is_err());
        if let Err(ShortcutError::InvalidFormat(msg)) = result {
            assert!(msg.contains("無効な修飾キー"));
        }
    }

    #[test]
    fn test_validate_shortcut_no_key() {
        let result = validate_shortcut("Command+");
        assert!(result.is_err());
        if let Err(ShortcutError::InvalidFormat(msg)) = result {
            assert!(msg.contains("キーが指定されていません"));
        }
    }
}
