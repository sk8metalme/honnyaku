/**
 * ショートカット管理Hook
 *
 * グローバルショートカットの登録・解除・イベント監視を管理する
 */

import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

/**
 * ショートカット登録状態
 */
export interface ShortcutStatus {
  /** 現在登録されているショートカット */
  currentShortcut: string | null;
  /** 登録されているかどうか */
  isRegistered: boolean;
}

/**
 * useShortcut Hookの戻り値
 */
export interface UseShortcutReturn {
  /** ショートカットが登録されているか */
  isRegistered: boolean;
  /** 現在登録されているショートカット */
  currentShortcut: string | null;
  /** 処理中かどうか */
  isLoading: boolean;
  /** エラーメッセージ */
  error: string | null;
  /** ショートカットを登録する */
  registerShortcut: (shortcut: string) => Promise<void>;
  /** ショートカットを解除する */
  unregisterShortcut: (shortcut: string) => Promise<void>;
  /** 全てのショートカットを解除する */
  unregisterAll: () => Promise<void>;
  /** ショートカット文字列を検証する */
  validateShortcut: (shortcut: string) => Promise<boolean>;
  /** ショートカット状態を再取得する */
  refreshStatus: () => Promise<void>;
}

/**
 * ショートカット管理Hook
 *
 * @param onShortcutTriggered - ショートカットが押された時のコールバック
 * @returns ショートカットの状態と操作関数
 *
 * @example
 * ```tsx
 * function App() {
 *   const {
 *     isRegistered,
 *     registerShortcut,
 *     error,
 *   } = useShortcut(() => {
 *     console.log('ショートカットが押されました！');
 *   });
 *
 *   useEffect(() => {
 *     registerShortcut('CommandOrControl+Shift+T');
 *   }, []);
 *
 *   return <div>{isRegistered ? '登録済み' : '未登録'}</div>;
 * }
 * ```
 */
export function useShortcut(
  onShortcutTriggered?: () => void
): UseShortcutReturn {
  const [isRegistered, setIsRegistered] = useState(false);
  const [currentShortcut, setCurrentShortcut] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // ショートカットイベントのリスナーを設定
  useEffect(() => {
    let unlisten: UnlistenFn | null = null;

    async function setupListener() {
      try {
        unlisten = await listen('shortcut-triggered', () => {
          onShortcutTriggered?.();
        });
      } catch (err) {
        console.error('Failed to setup shortcut listener:', err);
      }
    }

    setupListener();

    return () => {
      unlisten?.();
    };
  }, [onShortcutTriggered]);

  /**
   * ショートカット状態を取得
   */
  const refreshStatus = useCallback(async () => {
    try {
      const status = await invoke<ShortcutStatus>('get_shortcut_status');
      setIsRegistered(status.isRegistered);
      setCurrentShortcut(status.currentShortcut);
    } catch (err) {
      console.error('Failed to get shortcut status:', err);
    }
  }, []);

  // 初回マウント時にステータスを取得
  useEffect(() => {
    refreshStatus();
  }, [refreshStatus]);

  /**
   * ショートカット文字列を検証
   */
  const validateShortcut = useCallback(
    async (shortcut: string): Promise<boolean> => {
      try {
        await invoke('validate_shortcut_format', { shortcutStr: shortcut });
        return true;
      } catch {
        return false;
      }
    },
    []
  );

  /**
   * ショートカットを登録
   */
  const registerShortcut = useCallback(async (shortcut: string) => {
    setIsLoading(true);
    setError(null);

    try {
      await invoke('register_shortcut', { shortcutStr: shortcut });
      setIsRegistered(true);
      setCurrentShortcut(shortcut);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * ショートカットを解除
   */
  const unregisterShortcut = useCallback(async (shortcut: string) => {
    setIsLoading(true);
    setError(null);

    try {
      await invoke('unregister_shortcut', { shortcutStr: shortcut });
      setIsRegistered(false);
      setCurrentShortcut(null);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * 全てのショートカットを解除
   */
  const unregisterAll = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      await invoke('unregister_all_shortcuts');
      setIsRegistered(false);
      setCurrentShortcut(null);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  return {
    isRegistered,
    currentShortcut,
    isLoading,
    error,
    registerShortcut,
    unregisterShortcut,
    unregisterAll,
    validateShortcut,
    refreshStatus,
  };
}
