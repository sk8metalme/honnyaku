/**
 * 設定管理用Contextプロバイダー
 *
 * アプリ全体で設定を共有するためのContext
 */

import { createContext, useContext, type ReactNode } from 'react';
import { useSettings, type UseSettingsReturn } from '@/hooks/useSettings';

const SettingsContext = createContext<UseSettingsReturn | null>(null);

/**
 * 設定Context Providerコンポーネント
 */
export function SettingsProvider({ children }: { children: ReactNode }) {
  const settingsValue = useSettings();

  return (
    <SettingsContext.Provider value={settingsValue}>
      {children}
    </SettingsContext.Provider>
  );
}

/**
 * 設定Contextを使用するカスタムフック
 *
 * @returns 設定の状態と操作関数
 * @throws Context外で使用された場合エラー
 */
export function useSettingsContext(): UseSettingsReturn {
  const context = useContext(SettingsContext);
  if (!context) {
    throw new Error('useSettingsContext must be used within SettingsProvider');
  }
  return context;
}
