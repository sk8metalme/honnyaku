/**
 * 翻訳フロー統合Hook
 *
 * ショートカットイベント → クリップボード取得 → 翻訳実行までのフローを管理する
 */

import { useState, useEffect, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow, availableMonitors } from '@tauri-apps/api/window';
import { LogicalPosition } from '@tauri-apps/api/dpi';
import { detectLanguage } from '@/lib/language-detect';
import type { Language, TranslationResult } from '@/types';
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
  /** 翻訳時間（ミリ秒） */
  durationMs: number | null;
  /** ショートカットが有効かどうか */
  isShortcutEnabled: boolean;
  /** 手動で翻訳フローを開始する */
  startFlow: () => Promise<void>;
  /** 状態をリセットする */
  reset: () => Promise<void>;
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
  onTranslationComplete?: (result: {
    original: string;
    translated: string;
  }) => void;
  onError?: (error: TranslationFlowError) => void;
}): UseTranslationFlowReturn {
  const { autoStart = true, onTranslationComplete, onError } = options ?? {};

  const [state, setState] = useState<TranslationFlowState>('idle');
  const [originalText, setOriginalText] = useState('');
  const [translatedText, setTranslatedText] = useState<string | null>(null);
  const [error, setError] = useState<TranslationFlowError | null>(null);
  const [durationMs, setDurationMs] = useState<number | null>(null);
  const [isShortcutEnabled, setIsShortcutEnabled] = useState(true);

  // コールバックの参照を保持
  const onTranslationCompleteRef = useRef(onTranslationComplete);
  const onErrorRef = useRef(onError);

  // 実行中フラグ（同期的に重複実行を防ぐ）
  const isExecutingRef = useRef(false);

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

        setDurationMs(result.durationMs);
        return result.translatedText;
      } catch (err) {
        console.error('Translation failed:', err);
        return null;
      }
    },
    []
  );

  /**
   * ウィンドウをカーソル位置の近くに移動し、前面に表示する
   */
  const moveWindowToCursor = useCallback(async () => {
    try {
      // カーソル位置を取得 (macOS座標系: 左下が原点)
      const [cursorX, cursorY] = await invoke<[number, number]>(
        'get_cursor_position'
      );

      const window = getCurrentWindow();

      // 全てのモニターを取得
      const monitors = await availableMonitors();
      if (monitors.length === 0) return;

      // 各モニターのスケールファクターとLogical座標を計算
      const logicalMonitors = monitors.map((m) => {
        const scaleFactor = m.scaleFactor;
        const logicalPos = {
          x: m.position.x / scaleFactor,
          y: m.position.y / scaleFactor,
        };
        const logicalSize = {
          width: m.size.width / scaleFactor,
          height: m.size.height / scaleFactor,
        };
        return { ...m, logicalPos, logicalSize, scaleFactor };
      });

      // カーソルがどのモニターにあるかを判定（Logical座標で）
      let targetMonitor = logicalMonitors[0];

      // macOSのカーソルY座標をTauri座標系に変換（全画面の高さを使用 - Logical座標）
      const allMonitorsMaxY = Math.max(
        ...logicalMonitors.map((m) => m.logicalPos.y + m.logicalSize.height)
      );
      const tauriCursorY = allMonitorsMaxY - cursorY;

      for (const monitor of logicalMonitors) {
        const monitorLeft = monitor.logicalPos.x;
        const monitorTop = monitor.logicalPos.y;
        const monitorRight = monitorLeft + monitor.logicalSize.width;
        const monitorBottom = monitorTop + monitor.logicalSize.height;

        if (
          cursorX >= monitorLeft &&
          cursorX < monitorRight &&
          tauriCursorY >= monitorTop &&
          tauriCursorY < monitorBottom
        ) {
          targetMonitor = monitor;
          break;
        }
      }

      // ウィンドウサイズを取得（Logical）
      const windowSize = await window.outerSize();
      const windowWidth = windowSize.width / targetMonitor.scaleFactor;
      const windowHeight = windowSize.height / targetMonitor.scaleFactor;

      // ウィンドウをカーソルの右下に配置（オフセット: +20px, +20px）
      const offsetX = 20;
      const offsetY = 20;

      const tauriX = cursorX + offsetX;
      const tauriY = tauriCursorY + offsetY;

      // ウィンドウが画面外に出ないように調整（Logical座標）
      const monitorRight =
        targetMonitor.logicalPos.x + targetMonitor.logicalSize.width;
      const monitorBottom =
        targetMonitor.logicalPos.y + targetMonitor.logicalSize.height;

      const finalX = Math.max(
        targetMonitor.logicalPos.x,
        Math.min(tauriX, monitorRight - windowWidth - 10)
      );
      const finalY = Math.max(
        targetMonitor.logicalPos.y,
        Math.min(tauriY, monitorBottom - windowHeight - 10)
      );

      // ウィンドウ位置を設定（Logical座標）
      await window.setPosition(new LogicalPosition(finalX, finalY));

      // ウィンドウを前面に表示
      await window.setAlwaysOnTop(true);
      await window.setFocus();
    } catch (err) {
      console.error('Failed to move window to cursor:', err);
      // エラーが発生してもフローは継続
    }
  }, []);

  /**
   * 翻訳フローを開始する
   */
  const startFlow = useCallback(async () => {
    console.log(
      '[翻訳フロー] 開始リクエスト - 実行中:',
      isExecutingRef.current
    );

    // 既に実行中なら無視（同期的に重複実行を防ぐ）
    if (isExecutingRef.current) {
      console.log('[翻訳フロー] 既に実行中のためスキップ');
      return;
    }

    // 実行中フラグを立てる
    isExecutingRef.current = true;
    console.log('[翻訳フロー] 開始');

    try {
      // リセット
      setError(null);
      setOriginalText('');
      setTranslatedText(null);

      // Step 1: 選択テキストを取得（フォーカスが変わる前に実行）
      setState('getting-selection');

      const selectedText = await getSelectedText();

      if (!selectedText) {
        handleError('no-selection', 'テキストが選択されていません');
        return;
      }

      setOriginalText(selectedText);

      // Step 2: ウィンドウをカーソル位置に移動（テキスト取得後）
      await moveWindowToCursor();

      // Step 3: 翻訳を実行
      setState('translating');

      const result = await translateText(selectedText);

      if (!result) {
        handleError('translation-failed', '翻訳に失敗しました');
        return;
      }

      // 完了
      console.log('[翻訳フロー] 完了:', { textLength: result.length });
      setTranslatedText(result);
      setState('completed');
      onTranslationCompleteRef.current?.({
        original: selectedText,
        translated: result,
      });
    } finally {
      // 実行中フラグを解除
      isExecutingRef.current = false;
      console.log('[翻訳フロー] フラグ解除');
    }
  }, [getSelectedText, translateText, handleError, moveWindowToCursor]);

  // startFlowの参照を保持（useEffectでの重複登録を防ぐ）
  const startFlowRef = useRef(startFlow);
  useEffect(() => {
    startFlowRef.current = startFlow;
  }, [startFlow]);

  /**
   * 状態をリセットする
   */
  const reset = useCallback(async () => {
    // 実行中フラグを解除
    isExecutingRef.current = false;

    setState('idle');
    setOriginalText('');
    setTranslatedText(null);
    setError(null);
    setDurationMs(null);

    // ウィンドウを通常状態に戻す（常に前面表示を解除）
    try {
      const window = getCurrentWindow();
      await window.setAlwaysOnTop(false);
    } catch (err) {
      console.error('Failed to reset window state:', err);
    }
  }, []);

  // ショートカットイベントのリスナーを設定
  useEffect(() => {
    if (!autoStart) return;

    console.log('[リスナー] 登録開始');
    let unlisten: UnlistenFn | null = null;

    async function setupListener() {
      try {
        unlisten = await listen('shortcut-triggered', () => {
          console.log('[リスナー] ショートカットイベント受信');
          if (isShortcutEnabled) {
            void startFlowRef.current();
          }
        });
        console.log('[リスナー] 登録完了');
      } catch (err) {
        console.error('Failed to setup shortcut listener:', err);
      }
    }

    void setupListener();

    return () => {
      console.log('[リスナー] クリーンアップ');
      unlisten?.();
    };
  }, [autoStart, isShortcutEnabled]);

  return {
    state,
    originalText,
    translatedText,
    error,
    durationMs,
    isShortcutEnabled,
    startFlow,
    reset,
    setShortcutEnabled: setIsShortcutEnabled,
  };
}
