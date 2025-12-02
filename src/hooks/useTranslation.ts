/**
 * 翻訳Hook
 *
 * 言語検出から翻訳結果取得までのフローを管理する
 */

import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { detectLanguage } from '@/lib/language-detect';
import type { TranslationResult, Language } from '@/types';
import { toBackendLanguage } from '@/types';

/** 信頼度の閾値（これ以下はデフォルト方向を使用） */
const CONFIDENCE_THRESHOLD = 0.5;

/** デフォルトの翻訳元言語 */
const DEFAULT_SOURCE_LANG: Language = 'ja';

/** デフォルトの翻訳先言語 */
const DEFAULT_TARGET_LANG: Language = 'en';

/**
 * useTranslation Hookの戻り値
 */
export interface UseTranslationReturn {
  /** 翻訳処理中かどうか */
  isLoading: boolean;
  /** 翻訳元テキスト */
  originalText: string;
  /** 翻訳結果（翻訳前はnull） */
  translatedText: string | null;
  /** エラーメッセージ（エラーがない場合はnull） */
  error: string | null;
  /** テキストを翻訳する */
  translate: (text: string) => Promise<void>;
  /** 状態をリセットする */
  reset: () => void;
}

/**
 * 翻訳Hook
 *
 * 言語検出を実行し、適切な翻訳方向を決定してバックエンドの翻訳サービスを呼び出す
 *
 * @example
 * ```tsx
 * const { isLoading, translatedText, error, translate, reset } = useTranslation();
 *
 * const handleTranslate = async () => {
 *   await translate('Hello, world!');
 * };
 * ```
 */
export function useTranslation(): UseTranslationReturn {
  const [isLoading, setIsLoading] = useState(false);
  const [originalText, setOriginalText] = useState('');
  const [translatedText, setTranslatedText] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const translate = useCallback(async (text: string) => {
    // 空テキストのバリデーション
    const trimmedText = text.trim();
    if (!trimmedText) {
      setError('翻訳するテキストが入力されていません');
      return;
    }

    setIsLoading(true);
    setOriginalText(text);
    setTranslatedText(null);
    setError(null);

    try {
      // 言語検出
      const detection = detectLanguage(trimmedText);

      // 翻訳方向を決定
      let sourceLang: Language;
      let targetLang: Language;

      if (detection.confidence >= CONFIDENCE_THRESHOLD) {
        // 信頼度が高い場合は検出結果を使用
        sourceLang = detection.language;
        targetLang = detection.language === 'ja' ? 'en' : 'ja';
      } else {
        // 信頼度が低い場合はデフォルト（日→英）を使用
        sourceLang = DEFAULT_SOURCE_LANG;
        targetLang = DEFAULT_TARGET_LANG;
      }

      // バックエンドで翻訳を実行
      const result = await invoke<TranslationResult>('translate', {
        text: trimmedText,
        sourceLang: toBackendLanguage(sourceLang),
        targetLang: toBackendLanguage(targetLang),
      });

      setTranslatedText(result.translatedText);
    } catch (err) {
      // エラーメッセージを設定
      const errorMessage =
        err instanceof Error ? err.message : String(err);
      setError(errorMessage);
    } finally {
      setIsLoading(false);
    }
  }, []);

  const reset = useCallback(() => {
    setIsLoading(false);
    setOriginalText('');
    setTranslatedText(null);
    setError(null);
  }, []);

  return {
    isLoading,
    originalText,
    translatedText,
    error,
    translate,
    reset,
  };
}
