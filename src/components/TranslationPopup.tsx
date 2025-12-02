/**
 * 翻訳ポップアップコンポーネント
 *
 * 翻訳結果を表示するポップアップUI
 * - ローディング状態、翻訳結果、エラー状態の表示切り替え
 * - タイトルバーなしの角丸デザイン
 */

import { useState, useCallback } from 'react';
import type {
  TranslationFlowState,
  TranslationFlowError,
} from '@/hooks/useTranslationFlow';

/**
 * TranslationPopupのProps
 */
export interface TranslationPopupProps {
  /** 現在の状態 */
  state: TranslationFlowState;
  /** 翻訳元テキスト */
  originalText: string;
  /** 翻訳結果 */
  translatedText: string | null;
  /** エラー情報 */
  error: TranslationFlowError | null;
  /** 閉じるボタンクリック時のコールバック */
  onClose: () => void;
  /** コピーボタンクリック時のコールバック */
  onCopy?: (text: string) => void;
}

/**
 * ローディングスピナー
 */
function LoadingSpinner() {
  return (
    <div className="flex items-center justify-center py-8">
      <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500" />
    </div>
  );
}

/**
 * コピーボタン
 */
function CopyButton({
  text,
  onCopy,
}: {
  text: string;
  onCopy?: (text: string) => void;
}) {
  const [copied, setCopied] = useState(false);

  const handleCopy = useCallback(async () => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(true);
      onCopy?.(text);
      setTimeout(() => {
        setCopied(false);
      }, 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  }, [text, onCopy]);

  return (
    <button
      onClick={() => {
        void handleCopy();
      }}
      className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
      title="コピー"
      aria-label="翻訳結果をコピー"
    >
      {copied ? (
        <svg
          className="w-5 h-5 text-green-500"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M5 13l4 4L19 7"
          />
        </svg>
      ) : (
        <svg
          className="w-5 h-5 text-gray-500 dark:text-gray-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
          />
        </svg>
      )}
    </button>
  );
}

/**
 * 翻訳ポップアップコンポーネント
 */
export function TranslationPopup({
  state,
  originalText,
  translatedText,
  error,
  onClose,
  onCopy,
}: TranslationPopupProps) {
  // idle状態では何も表示しない
  if (state === 'idle') {
    return null;
  }

  return (
    <div className="fixed inset-0 flex items-center justify-center z-50">
      {/* オーバーレイ */}
      <div
        className="absolute inset-0 bg-black/20 backdrop-blur-sm"
        onClick={onClose}
      />

      {/* ポップアップ本体 */}
      <div className="relative bg-white dark:bg-gray-800 rounded-2xl shadow-2xl w-full max-w-md mx-4 overflow-hidden">
        {/* ヘッダー */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-sm font-medium text-gray-600 dark:text-gray-300">
            {state === 'getting-selection' && '選択テキストを取得中...'}
            {state === 'translating' && '翻訳中...'}
            {state === 'completed' && '翻訳完了'}
            {state === 'error' && 'エラー'}
          </h2>
          <button
            onClick={onClose}
            className="p-1 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            aria-label="閉じる"
          >
            <svg
              className="w-5 h-5 text-gray-500"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        {/* コンテンツ */}
        <div className="p-4">
          {/* ローディング状態 */}
          {(state === 'getting-selection' || state === 'translating') && (
            <LoadingSpinner />
          )}

          {/* エラー状態 */}
          {state === 'error' && error && (
            <div className="py-4">
              <div className="flex items-center gap-2 text-red-500 mb-2">
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
                <span className="font-medium">エラーが発生しました</span>
              </div>
              <p className="text-gray-600 dark:text-gray-400 text-sm">
                {error.message}
              </p>
              {error.type === 'permission-denied' && (
                <p className="text-gray-500 dark:text-gray-500 text-xs mt-2">
                  システム環境設定からアクセシビリティ権限を許可してください
                </p>
              )}
              {error.type === 'no-selection' && (
                <p className="text-gray-500 dark:text-gray-500 text-xs mt-2">
                  翻訳したいテキストを選択してからショートカットを押してください
                </p>
              )}
            </div>
          )}

          {/* 翻訳完了状態 */}
          {state === 'completed' && translatedText && (
            <div className="space-y-4">
              {/* 元のテキスト */}
              {originalText && (
                <div>
                  <div className="text-xs text-gray-500 dark:text-gray-400 mb-1">
                    原文
                  </div>
                  <div className="text-sm text-gray-600 dark:text-gray-300 bg-gray-50 dark:bg-gray-900 rounded-lg p-3 max-h-24 overflow-y-auto">
                    {originalText}
                  </div>
                </div>
              )}

              {/* 翻訳結果 */}
              <div>
                <div className="flex items-center justify-between mb-1">
                  <span className="text-xs text-gray-500 dark:text-gray-400">
                    翻訳結果
                  </span>
                  <CopyButton text={translatedText} onCopy={onCopy} />
                </div>
                <div className="text-base text-gray-800 dark:text-white bg-blue-50 dark:bg-blue-900/30 rounded-lg p-3 max-h-48 overflow-y-auto">
                  {translatedText}
                </div>
              </div>
            </div>
          )}
        </div>

        {/* フッター */}
        {state === 'completed' && (
          <div className="px-4 py-3 border-t border-gray-200 dark:border-gray-700 text-center">
            <span className="text-xs text-gray-400 dark:text-gray-500">
              Esc キーで閉じる
            </span>
          </div>
        )}
      </div>
    </div>
  );
}
