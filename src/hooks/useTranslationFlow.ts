/**
 * 翻訳フロー統合Hook
 *
 * ショートカットイベント → クリップボード取得 → 翻訳実行までのフローを管理する
 */

import { useState, useEffect, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { detectLanguage } from '@/lib/language-detect';
import type { TranslationResult, Language } from '@/types';
import { toBackendLanguage } from '@/types';
import type { ClipboardContent } from './useClipboard';

/** 信頼度の閾値（これ以下はデフォルト方向を使用） */
const CONFIDENCE_THRESHOLD = 0.5;

/** デフォルトの翻訳元言語 */
const DEFAULT_SOURCE_LANG: Language = 'ja';

/** デフォルトの翻訳先言語 */
const DEFAULT_TARGET_LANG: Language = 'en';

/**
 * 翻訳フローの状態
 */
export type TranslationFlowState =
  | 'idle' // 待機中
  | 'getting-selection' // 選択テキスト取得中
  | 'translating' // 翻訳中
  | 'completed' // 完了
  | 'error'; // エラー

/**
 * 翻訳フローエラーの種類
 */
export type TranslationFlowErrorType =
  | 'permission-denied' // アクセシビリティ権限がない
  | 'no-selection' // テキストが選択されていない
  | 'translation-failed' // 翻訳に失敗
  | 'unknown'; // 不明なエラー

/**
 * 翻訳フローエラー
 */
export interface TranslationFlowError {
  type: TranslationFlowErrorType;
  message: string;
}

/**
 * useTranslationFlow Hookの戻り値
 */
export interface UseTranslationFlowReturn {
  /** 現在の状態 */
  state: TranslationFlowState;
  /** 翻訳元テキスト */
  originalText: string;
  /** 翻訳結果 */
  translatedText: string | null;
  /** エラー情報 */
  error: TranslationFlowError | null;
  /** ショートカットが有効かどうか */
  isShortcutEnabled: boolean;
  /** 手動で翻訳フローを開始する */
  startFlow: () => Promise<void>;
  /** 状態をリセットする */
  reset: () => void;
  /** ショートカットを有効/無効にする */
  setShortcutEnabled: (enabled: boolean) => void;
}

/**
 * 翻訳フロー統合Hook
 *
 * グローバルショートカットが押された時に、選択テキストを取得して自動的に翻訳を実行する
 *
 * @param options.autoStart - ショートカット押下時に自動で翻訳を開始するか（デフォルト: true）
 * @param options.onTranslationComplete - 翻訳完了時のコールバック
 * @param options.onError - エラー発生時のコールバック
 *
 * @example
 * ```tsx
 * function TranslationPopup() {
 *   const {
 *     state,
 *     originalText,
 *     translatedText,
 *     error,
 *     reset,
 *   } = useTranslationFlow({
 *     onTranslationComplete: () => {
 *       // ポップアップを表示
 *     },
 *     onError: (err) => {
 *       console.error('翻訳エラー:', err.message);
 *     },
 *   });
 *
 *   if (state === 'idle') return null;
 *   if (state === 'getting-selection' || state === 'translating') {
 *     return <LoadingIndicator />;
 *   }
 *   if (state === 'error') {
 *     return <ErrorMessage message={error?.message} />;
 *   }
 *
 *   return (
 *     <TranslationResult
 *       original={originalText}
 *       translated={translatedText}
 *       onClose={reset}
 *     />
 *   );
 * }
 * ```
 */
export function useTranslationFlow(options?: {
  autoStart?: boolean;
  onTranslationComplete?: (result: { original: string; translated: string }) => void;
  onError?: (error: TranslationFlowError) => void;
}): UseTranslationFlowReturn {
  const { autoStart = true, onTranslationComplete, onError } = options ?? {};

  const [state, setState] = useState<TranslationFlowState>('idle');
  const [originalText, setOriginalText] = useState('');
  const [translatedText, setTranslatedText] = useState<string | null>(null);
  const [error, setError] = useState<TranslationFlowError | null>(null);
  const [isShortcutEnabled, setIsShortcutEnabled] = useState(true);

  // コールバックの参照を保持
  const onTranslationCompleteRef = useRef(onTranslationComplete);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onTranslationCompleteRef.current = onTranslationComplete;
    onErrorRef.current = onError;
  }, [onTranslationComplete, onError]);

  /**
   * エラーを設定してエラーコールバックを呼び出す
   */
  const handleError = useCallback(
    (type: TranslationFlowErrorType, message: string) => {
      const err: TranslationFlowError = { type, message };
      setError(err);
      setState('error');
      onErrorRef.current?.(err);
    },
    []
  );

  /**
   * 選択テキストを取得する
   */
  const getSelectedText = useCallback(async (): Promise<string | null> => {
    try {
      const content = await invoke<ClipboardContent>('get_selected_text');

      if (!content.success || !content.text.trim()) {
        return null;
      }

      return content.text;
    } catch (err) {
      console.error('Failed to get selected text:', err);
      return null;
    }
  }, []);

  /**
   * テキストを翻訳する
   */
  const translateText = useCallback(
    async (text: string): Promise<string | null> => {
      try {
        // 言語検出
        const detection = detectLanguage(text);

        // 翻訳方向を決定
        let sourceLang: Language;
        let targetLang: Language;

        if (detection.confidence >= CONFIDENCE_THRESHOLD) {
          sourceLang = detection.language;
          targetLang = detection.language === 'ja' ? 'en' : 'ja';
        } else {
          sourceLang = DEFAULT_SOURCE_LANG;
          targetLang = DEFAULT_TARGET_LANG;
        }

        // バックエンドで翻訳を実行
        const result = await invoke<TranslationResult>('translate', {
          text,
          sourceLang: toBackendLanguage(sourceLang),
          targetLang: toBackendLanguage(targetLang),
        });

        return result.translatedText;
      } catch (err) {
        console.error('Translation failed:', err);
        return null;
      }
    },
    []
  );

  /**
   * 翻訳フローを開始する
   */
  const startFlow = useCallback(async () => {
    // リセット
    setError(null);
    setOriginalText('');
    setTranslatedText(null);

    // Step 1: 選択テキストを取得
    setState('getting-selection');

    const selectedText = await getSelectedText();

    if (!selectedText) {
      handleError('no-selection', 'テキストが選択されていません');
      return;
    }

    setOriginalText(selectedText);

    // Step 2: 翻訳を実行
    setState('translating');

    const result = await translateText(selectedText);

    if (!result) {
      handleError('translation-failed', '翻訳に失敗しました');
      return;
    }

    // 完了
    setTranslatedText(result);
    setState('completed');
    onTranslationCompleteRef.current?.({
      original: selectedText,
      translated: result,
    });
  }, [getSelectedText, translateText, handleError]);

  /**
   * 状態をリセットする
   */
  const reset = useCallback(() => {
    setState('idle');
    setOriginalText('');
    setTranslatedText(null);
    setError(null);
  }, []);

  // ショートカットイベントのリスナーを設定
  useEffect(() => {
    if (!autoStart) return;

    let unlisten: UnlistenFn | null = null;

    async function setupListener() {
      try {
        unlisten = await listen('shortcut-triggered', () => {
          if (isShortcutEnabled) {
            void startFlow();
          }
        });
      } catch (err) {
        console.error('Failed to setup shortcut listener:', err);
      }
    }

    void setupListener();

    return () => {
      unlisten?.();
    };
  }, [autoStart, isShortcutEnabled, startFlow]);

  return {
    state,
    originalText,
    translatedText,
    error,
    isShortcutEnabled,
    startFlow,
    reset,
    setShortcutEnabled: setIsShortcutEnabled,
  };
}
