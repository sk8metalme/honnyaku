/**
 * Honnyaku - AI Translation Desktop App
 *
 * メインアプリケーションコンポーネント
 */

import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { TranslationPopup } from '@/components/TranslationPopup';
import { SettingsPanel } from '@/components/SettingsPanel';
import { useTranslationFlow } from '@/hooks/useTranslationFlow';
import { useShortcut } from '@/hooks/useShortcut';
import { usePermissions } from '@/hooks/usePermissions';
import { useSettings } from '@/hooks/useSettings';
import { useClipboard } from '@/hooks/useClipboard';

function App() {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  // Hooks
  const { settings, isLoading: settingsLoading } = useSettings();
  const { isAccessibilityGranted, checkAccessibility } = usePermissions();
  const { registerShortcut, isRegistered } = useShortcut();
  const { writeText } = useClipboard();

  // 翻訳フロー
  const {
    state,
    originalText,
    translatedText,
    error,
    reset,
  } = useTranslationFlow({
    onTranslationComplete: () => {
      // 翻訳完了時の処理（必要に応じてサウンド再生など）
    },
    onError: (err) => {
      console.error('Translation error:', err);
    },
  });

  // 初期化: ショートカット登録
  useEffect(() => {
    if (settings && !isRegistered && !settingsLoading) {
      registerShortcut(settings.shortcut).catch((err) => {
        console.error('Failed to register shortcut:', err);
      });
    }
  }, [settings, isRegistered, settingsLoading, registerShortcut]);

  // 初期化: アクセシビリティ権限確認
  useEffect(() => {
    checkAccessibility();
  }, [checkAccessibility]);

  // 初期化: Ollamaモデルをプリロード（初回翻訳を高速化）
  useEffect(() => {
    if (settings && !settingsLoading) {
      invoke('preload_ollama_model').catch((err) => {
        console.log('Model preload skipped:', err);
      });
    }
  }, [settings, settingsLoading]);

  // Escキーでポップアップを閉じる
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        if (state !== 'idle') {
          reset();
        } else if (isSettingsOpen) {
          setIsSettingsOpen(false);
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [state, isSettingsOpen, reset]);

  // 翻訳結果をコピー
  const handleCopy = useCallback(async (text: string) => {
    try {
      await writeText(text);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  }, [writeText]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
      {/* メインコンテンツ */}
      <div className="flex flex-col items-center justify-center min-h-screen p-4">
        <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-xl p-8 w-full max-w-md">
          {/* ヘッダー */}
          <div className="flex items-center justify-between mb-6">
            <h1 className="text-2xl font-bold text-gray-800 dark:text-white">
              Honnyaku
            </h1>
            <button
              onClick={() => setIsSettingsOpen(true)}
              className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              aria-label="設定"
            >
              <svg
                className="w-6 h-6 text-gray-600 dark:text-gray-300"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                />
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                />
              </svg>
            </button>
          </div>

          {/* 説明 */}
          <p className="text-center text-gray-600 dark:text-gray-300 mb-6">
            AI翻訳デスクトップアプリ
          </p>

          {/* ステータス */}
          <div className="space-y-3">
            {/* アクセシビリティ権限 */}
            <div className="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
              <span className={`w-2 h-2 rounded-full ${
                isAccessibilityGranted ? 'bg-green-500' : 'bg-red-500'
              }`} />
              <span className="text-sm text-gray-600 dark:text-gray-400">
                アクセシビリティ権限: {isAccessibilityGranted ? '許可済み' : '未許可'}
              </span>
            </div>

            {/* ショートカット */}
            <div className="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
              <span className={`w-2 h-2 rounded-full ${
                isRegistered ? 'bg-green-500' : 'bg-yellow-500'
              }`} />
              <span className="text-sm text-gray-600 dark:text-gray-400">
                ショートカット: {settings?.shortcut || '未設定'}
                {isRegistered && ' (登録済み)'}
              </span>
            </div>

            {/* Ollamaモデル */}
            <div className="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
              <span className="w-2 h-2 rounded-full bg-blue-500" />
              <span className="text-sm text-gray-600 dark:text-gray-400">
                モデル: {settings?.ollamaModel || 'qwen2.5:3b'}
              </span>
            </div>
          </div>

          {/* 使い方 */}
          <div className="mt-6 p-4 bg-blue-50 dark:bg-blue-900/30 rounded-lg">
            <h3 className="text-sm font-semibold text-blue-800 dark:text-blue-200 mb-2">
              使い方
            </h3>
            <ol className="text-sm text-blue-700 dark:text-blue-300 space-y-1 list-decimal list-inside">
              <li>翻訳したいテキストを選択</li>
              <li>
                <kbd className="px-1.5 py-0.5 bg-blue-100 dark:bg-blue-800 rounded text-xs">
                  {settings?.shortcut || 'Cmd+Shift+T'}
                </kbd>
                {' '}を押す
              </li>
              <li>翻訳結果がポップアップで表示</li>
            </ol>
          </div>

          {/* フッター */}
          <div className="mt-6 text-center">
            <button
              onClick={() => setIsSettingsOpen(true)}
              className="text-sm text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300"
            >
              設定を開く
            </button>
          </div>
        </div>
      </div>

      {/* 翻訳ポップアップ */}
      <TranslationPopup
        state={state}
        originalText={originalText}
        translatedText={translatedText}
        error={error}
        onClose={reset}
        onCopy={handleCopy}
      />

      {/* 設定パネル */}
      <SettingsPanel
        isOpen={isSettingsOpen}
        onClose={() => setIsSettingsOpen(false)}
      />
    </div>
  );
}

export default App;
