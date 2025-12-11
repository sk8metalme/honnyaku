/**
 * 権限管理Hook
 *
 * macOSのアクセシビリティ権限の確認とリクエストを管理する
 */

import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * 権限状態
 */
export interface PermissionStatus {
  /** アクセシビリティ権限が付与されているか */
  accessibilityGranted: boolean;
  /** 権限リクエストが必要か */
  needsPermissionRequest: boolean;
}

/**
 * usePermissions Hookの戻り値
 */
export interface UsePermissionsReturn {
  /** アクセシビリティ権限が付与されているか */
  isAccessibilityGranted: boolean;
  /** 権限確認中かどうか */
  isChecking: boolean;
  /** エラーメッセージ */
  error: string | null;
  /** 権限をリクエストする */
  requestAccessibility: () => Promise<void>;
  /** 権限状態を再確認する */
  checkAccessibility: () => Promise<void>;
}

/**
 * 権限管理Hook
 *
 * @returns 権限の状態と操作関数
 *
 * @example
 * ```tsx
 * function App() {
 *   const {
 *     isAccessibilityGranted,
 *     requestAccessibility,
 *   } = usePermissions();
 *
 *   if (!isAccessibilityGranted) {
 *     return (
 *       <button onClick={requestAccessibility}>
 *         アクセシビリティ権限を許可
 *       </button>
 *     );
 *   }
 *
 *   return <div>権限が付与されています</div>;
 * }
 * ```
 */
export function usePermissions(): UsePermissionsReturn {
  const [isAccessibilityGranted, setIsAccessibilityGranted] = useState(false);
  const [isChecking, setIsChecking] = useState(true);
  const [error, setError] = useState<string | null>(null);

  /**
   * アクセシビリティ権限を確認
   */
  const checkAccessibility = useCallback(async () => {
    setIsChecking(true);
    setError(null);

    try {
      const granted = await invoke<boolean>('is_accessibility_granted');
      setIsAccessibilityGranted(granted);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      console.error('Failed to check accessibility permission:', err);
    } finally {
      setIsChecking(false);
    }
  }, []);

  /**
   * アクセシビリティ権限をリクエスト
   */
  const requestAccessibility = useCallback(async () => {
    setIsChecking(true);
    setError(null);

    try {
      const status = await invoke<PermissionStatus>(
        'request_accessibility_permission_prompt'
      );
      setIsAccessibilityGranted(status.accessibilityGranted);

      // 権限が付与されていない場合、ポーリングで再チェック
      if (!status.accessibilityGranted) {
        let retries = 0;
        const maxRetries = 10; // 最大10秒間（1秒×10回）
        const pollingInterval = 1000; // 1秒ごと

        const pollingTimer = setInterval(async () => {
          try {
            const granted = await invoke<boolean>('is_accessibility_granted');
            if (granted) {
              setIsAccessibilityGranted(true);
              clearInterval(pollingTimer);
              setIsChecking(false);
            } else {
              retries++;
              if (retries >= maxRetries) {
                clearInterval(pollingTimer);
                setIsChecking(false);
              }
            }
          } catch (pollErr) {
            console.error('Polling error:', pollErr);
            clearInterval(pollingTimer);
            setIsChecking(false);
          }
        }, pollingInterval);
      } else {
        setIsChecking(false);
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      console.error('Failed to request accessibility permission:', err);
      setIsChecking(false);
    }
  }, []);

  // 初回マウント時に権限を確認
  useEffect(() => {
    void checkAccessibility();
  }, [checkAccessibility]);

  return {
    isAccessibilityGranted,
    isChecking,
    error,
    requestAccessibility,
    checkAccessibility,
  };
}
