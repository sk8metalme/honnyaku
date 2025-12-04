/**
 * useTranslationFlow Hook Tests
 */

import { renderHook, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useTranslationFlow } from '../useTranslationFlow';
import type { ClipboardContent } from '../useClipboard';
import type { TranslationResult } from '@/types';

// Tauriのinvokeをモック
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Tauriのイベントリスナーをモック
const mockListeners = new Map<string, (() => void)[]>();
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn((event: string, callback: () => void) => {
    const listeners = mockListeners.get(event) || [];
    listeners.push(callback);
    mockListeners.set(event, listeners);
    return Promise.resolve(() => {
      const currentListeners = mockListeners.get(event) || [];
      mockListeners.set(
        event,
        currentListeners.filter((l) => l !== callback)
      );
    });
  }),
}));

// Tauriのウィンドウ APIをモック
vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: vi.fn(() => ({
    outerSize: vi.fn().mockResolvedValue({ width: 400, height: 604 }),
    setPosition: vi.fn().mockResolvedValue(undefined),
    setAlwaysOnTop: vi.fn().mockResolvedValue(undefined),
    setFocus: vi.fn().mockResolvedValue(undefined),
  })),
  availableMonitors: vi.fn().mockResolvedValue([
    {
      name: 'Mock Monitor',
      position: { x: 0, y: 0 },
      size: { width: 1920, height: 1080 },
      scaleFactor: 2,
    },
  ]),
}));

// Tauriの DPI APIをモック
vi.mock('@tauri-apps/api/dpi', () => ({
  LogicalPosition: vi.fn((x: number, y: number) => ({ x, y })),
}));

// 言語検出をモック
vi.mock('@/lib/language-detect', () => ({
  detectLanguage: vi.fn((text: string) => {
    // 日本語文字を含む場合は日本語と判定
    const hasJapanese = /[\u3040-\u309f\u30a0-\u30ff\u4e00-\u9fff]/.test(text);
    return {
      language: hasJapanese ? 'ja' : 'en',
      confidence: 0.9,
    };
  }),
}));

import { invoke } from '@tauri-apps/api/core';
const mockInvoke = vi.mocked(invoke);

// ショートカットイベントを発火するヘルパー
function triggerShortcut() {
  const listeners = mockListeners.get('shortcut-triggered') || [];
  listeners.forEach((listener) => {
    listener();
  });
}

describe('useTranslationFlow', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockListeners.clear();
  });

  describe('初期状態', () => {
    it('初期状態がidleであること', () => {
      const { result } = renderHook(() => useTranslationFlow());

      expect(result.current.state).toBe('idle');
      expect(result.current.originalText).toBe('');
      expect(result.current.translatedText).toBeNull();
      expect(result.current.error).toBeNull();
      expect(result.current.isShortcutEnabled).toBe(true);
    });
  });

  describe('startFlow', () => {
    it('選択テキストがない場合にno-selectionエラーになること', async () => {
      // 空のクリップボード
      mockInvoke.mockResolvedValueOnce({
        text: '',
        success: false,
      } as ClipboardContent);

      const onError = vi.fn();
      const { result } = renderHook(() =>
        useTranslationFlow({ onError, autoStart: false })
      );

      await act(async () => {
        await result.current.startFlow();
      });

      expect(result.current.state).toBe('error');
      expect(result.current.error?.type).toBe('no-selection');
      expect(onError).toHaveBeenCalledWith(
        expect.objectContaining({ type: 'no-selection' })
      );
    });

    it('翻訳が成功した場合にcompletedになること', async () => {
      // 選択テキスト取得
      mockInvoke.mockResolvedValueOnce({
        text: 'Hello, world!',
        success: true,
      } as ClipboardContent);

      // カーソル位置取得
      mockInvoke.mockResolvedValueOnce([100, 200]);

      // 翻訳結果
      mockInvoke.mockResolvedValueOnce({
        translatedText: 'こんにちは、世界！',
        sourceLang: 'english',
        targetLang: 'japanese',
        provider: 'ollama',
        durationMs: 100,
      } as TranslationResult);

      const onTranslationComplete = vi.fn();
      const { result } = renderHook(() =>
        useTranslationFlow({ onTranslationComplete, autoStart: false })
      );

      await act(async () => {
        await result.current.startFlow();
      });

      expect(result.current.state).toBe('completed');
      expect(result.current.originalText).toBe('Hello, world!');
      expect(result.current.translatedText).toBe('こんにちは、世界！');
      expect(result.current.error).toBeNull();
      expect(onTranslationComplete).toHaveBeenCalledWith({
        original: 'Hello, world!',
        translated: 'こんにちは、世界！',
      });
    });

    it('翻訳が失敗した場合にtranslation-failedエラーになること', async () => {
      // 選択テキスト取得
      mockInvoke.mockResolvedValueOnce({
        text: 'Hello, world!',
        success: true,
      } as ClipboardContent);

      // 翻訳失敗
      mockInvoke.mockRejectedValueOnce(new Error('API error'));

      const onError = vi.fn();
      const { result } = renderHook(() =>
        useTranslationFlow({ onError, autoStart: false })
      );

      await act(async () => {
        await result.current.startFlow();
      });

      expect(result.current.state).toBe('error');
      expect(result.current.error?.type).toBe('translation-failed');
      expect(onError).toHaveBeenCalled();
    });
  });

  describe('ショートカットイベント', () => {
    it('ショートカットイベントで翻訳フローが開始されること', async () => {
      mockInvoke.mockResolvedValueOnce({
        text: 'テスト',
        success: true,
      } as ClipboardContent);

      // カーソル位置取得
      mockInvoke.mockResolvedValueOnce([100, 200]);

      mockInvoke.mockResolvedValueOnce({
        translatedText: 'Test',
        sourceLang: 'japanese',
        targetLang: 'english',
        provider: 'ollama',
        durationMs: 100,
      } as TranslationResult);

      const { result } = renderHook(() => useTranslationFlow());

      // ショートカットを発火
      await act(async () => {
        triggerShortcut();
        // イベント処理を待つ
        await new Promise((resolve) => setTimeout(resolve, 100));
      });

      await waitFor(() => {
        expect(result.current.state).toBe('completed');
      });

      expect(result.current.originalText).toBe('テスト');
      expect(result.current.translatedText).toBe('Test');
    });

    it('isShortcutEnabledがfalseの場合はフローが開始されないこと', async () => {
      const { result } = renderHook(() => useTranslationFlow());

      // ショートカットを無効化
      await act(async () => {
        result.current.setShortcutEnabled(false);
      });

      expect(result.current.isShortcutEnabled).toBe(false);

      // 無効化後に再レンダリングを待つ
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 50));
      });

      // ショートカットを発火 - 無効化されているので何も起こらないはず
      await act(async () => {
        triggerShortcut();
        await new Promise((resolve) => setTimeout(resolve, 50));
      });

      // フローが開始されていないことを確認
      expect(result.current.state).toBe('idle');
    });

    it('autoStart=falseの場合はショートカットイベントで開始されないこと', async () => {
      const { result } = renderHook(() =>
        useTranslationFlow({ autoStart: false })
      );

      act(() => {
        triggerShortcut();
      });

      // フローが開始されていないことを確認
      expect(result.current.state).toBe('idle');
    });
  });

  describe('reset', () => {
    it('resetで状態がidleに戻ること', async () => {
      mockInvoke.mockResolvedValueOnce({
        text: 'Hello',
        success: true,
      } as ClipboardContent);

      // カーソル位置取得
      mockInvoke.mockResolvedValueOnce([100, 200]);

      mockInvoke.mockResolvedValueOnce({
        translatedText: 'こんにちは',
        sourceLang: 'english',
        targetLang: 'japanese',
        provider: 'ollama',
        durationMs: 100,
      } as TranslationResult);

      const { result } = renderHook(() =>
        useTranslationFlow({ autoStart: false })
      );

      // 翻訳を実行
      await act(async () => {
        await result.current.startFlow();
      });

      expect(result.current.state).toBe('completed');

      // リセット
      await act(async () => {
        await result.current.reset();
      });

      expect(result.current.state).toBe('idle');
      expect(result.current.originalText).toBe('');
      expect(result.current.translatedText).toBeNull();
      expect(result.current.error).toBeNull();
    });
  });

  describe('日本語テキストの翻訳', () => {
    it('日本語テキストが英語に翻訳されること', async () => {
      mockInvoke.mockResolvedValueOnce({
        text: 'こんにちは',
        success: true,
      } as ClipboardContent);

      // カーソル位置取得
      mockInvoke.mockResolvedValueOnce([100, 200]);

      mockInvoke.mockResolvedValueOnce({
        translatedText: 'Hello',
        sourceLang: 'japanese',
        targetLang: 'english',
        provider: 'ollama',
        durationMs: 100,
      } as TranslationResult);

      const { result } = renderHook(() =>
        useTranslationFlow({ autoStart: false })
      );

      await act(async () => {
        await result.current.startFlow();
      });

      expect(result.current.translatedText).toBe('Hello');

      // 翻訳呼び出しの引数を確認
      expect(mockInvoke).toHaveBeenCalledWith('translate', {
        text: 'こんにちは',
        sourceLang: 'japanese',
        targetLang: 'english',
      });
    });
  });
});
