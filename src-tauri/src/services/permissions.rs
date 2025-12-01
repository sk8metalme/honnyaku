//! 権限管理サービス
//!
//! macOSのアクセシビリティ権限の確認と管理を提供

use serde::Serialize;

/// 権限状態
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    /// アクセシビリティ権限が付与されているか
    pub accessibility_granted: bool,
    /// 権限リクエストが必要か
    pub needs_permission_request: bool,
}

impl Default for PermissionStatus {
    fn default() -> Self {
        Self {
            accessibility_granted: false,
            needs_permission_request: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_status_default() {
        let status = PermissionStatus::default();
        assert!(!status.accessibility_granted);
        assert!(status.needs_permission_request);
    }

    #[test]
    fn test_permission_status_serialization() {
        let status = PermissionStatus {
            accessibility_granted: true,
            needs_permission_request: false,
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"accessibilityGranted\":true"));
        assert!(json.contains("\"needsPermissionRequest\":false"));
    }
}
