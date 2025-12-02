/**
 * 設定管理Hook
 *
 * Tauri IPC経由でアプリケーション設定の読み込み・保存を管理する。
 * tauri-plugin-storeを使用してJSON形式で設定を永続化。
 */

import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { AppSettings, ProviderStatus } from '@/types';
import { DEFAULT_SETTINGS } from '@/types';

/**
 * 設定管理Hookの戻り値
 */
export interface UseSettingsReturn {
  /** 現在の設定 */
  settings: AppSettings;
  /** 設定を読み込み中かどうか */
  isLoading: boolean;
  /** エラーメッセージ */
  error: string | null;
  /** 設定を更新 */
  updateSettings: (newSettings: Partial<AppSettings>) => Promise<void>;
  /** 設定をリセット */
  resetSettings: () => Promise<void>;
  /** 設定を再読み込み */
  reloadSettings: () => Promise<void>;
  /** Ollamaの接続状態を確認 */
  checkProviderStatus: () => Promise<ProviderStatus>;
}

/**
 * 設定管理Hook
 *
 * @returns 設定の状態と操作関数
 *
 * @example
 * ```tsx
 * function SettingsPanel() {
 *   const { settings, isLoading, updateSettings } = useSettings();
 *
 *   if (isLoading) return <div>Loading...</div>;
 *
 *   return (
 *     <input
 *       value={settings.shortcut}
 *       onChange={(e) => updateSettings({ shortcut: e.target.value })}
 *     />
 *   );
 * }
 * ```
 */
export function useSettings(): UseSettingsReturn {
  const [settings, setSettings] = useState<AppSettings>(DEFAULT_SETTINGS);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // 設定の読み込み
  useEffect(() => {
    let mounted = true;

    async function loadSettings() {
      try {
        setIsLoading(true);
        setError(null);

        const loadedSettings = await invoke<AppSettings>('get_settings');

        if (mounted) {
          setSettings(loadedSettings);
        }
      } catch (err) {
        if (mounted) {
          console.error('Failed to load settings:', err);
          setError('設定の読み込みに失敗しました');
        }
      } finally {
        if (mounted) {
          setIsLoading(false);
        }
      }
    }

    loadSettings();

    return () => {
      mounted = false;
    };
  }, []);

  /**
   * 設定を更新
   */
  const updateSettings = useCallback(
    async (newSettings: Partial<AppSettings>) => {
      try {
        setError(null);

        const updatedSettings = { ...settings, ...newSettings };

        await invoke('save_settings', { settings: updatedSettings });

        // ローカル状態を更新
        setSettings(updatedSettings);
      } catch (err) {
        console.error('Failed to update settings:', err);
        setError('設定の保存に失敗しました');
        throw err;
      }
    },
    [settings]
  );

  /**
   * 設定をデフォルトにリセット
   */
  const resetSettings = useCallback(async () => {
    try {
      setError(null);

      const defaultSettings = await invoke<AppSettings>('reset_settings');
      setSettings(defaultSettings);
    } catch (err) {
      console.error('Failed to reset settings:', err);
      setError('設定のリセットに失敗しました');
      throw err;
    }
  }, []);

  /**
   * 設定を再読み込み
   */
  const reloadSettings = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);

      const loadedSettings = await invoke<AppSettings>('get_settings');
      setSettings(loadedSettings);
    } catch (err) {
      console.error('Failed to reload settings:', err);
      setError('設定の再読み込みに失敗しました');
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Ollamaの接続状態を確認
   */
  const checkProviderStatus = useCallback(async (): Promise<ProviderStatus> => {
    try {
      const status = await invoke<ProviderStatus>('check_provider_status');
      return status;
    } catch (err) {
      console.error('Failed to check provider status:', err);
      return { status: 'unavailable', reason: '接続状態の確認に失敗しました' };
    }
  }, []);

  return {
    settings,
    isLoading,
    error,
    updateSettings,
    resetSettings,
    reloadSettings,
    checkProviderStatus,
  };
}
