import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act, waitFor } from '@testing-library/react';
import { useTranslation } from '../useTranslation';

// Tauri invokeをモック
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// language-detectをモック
vi.mock('@/lib/language-detect', () => ({
  detectLanguage: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';
import { detectLanguage } from '@/lib/language-detect';

describe('useTranslation', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('初期状態は正しく設定されている', () => {
    const { result } = renderHook(() => useTranslation());

    expect(result.current.isLoading).toBe(false);
    expect(result.current.originalText).toBe('');
    expect(result.current.translatedText).toBeNull();
    expect(result.current.error).toBeNull();
  });

  it('日本語テキストを英語に翻訳できる', async () => {
    vi.mocked(detectLanguage).mockReturnValue({
      language: 'ja',
      confidence: 0.9,
    });

    vi.mocked(invoke).mockResolvedValue({
      translatedText: 'Hello',
      sourceLang: 'japanese',
      targetLang: 'english',
      provider: 'ollama',
      durationMs: 500,
    });

    const { result } = renderHook(() => useTranslation());

    await act(async () => {
      await result.current.translate('こんにちは');
    });

    expect(result.current.originalText).toBe('こんにちは');
    expect(result.current.translatedText).toBe('Hello');
    expect(result.current.error).toBeNull();
    expect(result.current.isLoading).toBe(false);

    expect(invoke).toHaveBeenCalledWith('translate', {
      text: 'こんにちは',
      sourceLang: 'japanese',
      targetLang: 'english',
    });
  });

  it('英語テキストを日本語に翻訳できる', async () => {
    vi.mocked(detectLanguage).mockReturnValue({
      language: 'en',
      confidence: 0.95,
    });

    vi.mocked(invoke).mockResolvedValue({
      translatedText: 'こんにちは',
      sourceLang: 'english',
      targetLang: 'japanese',
      provider: 'ollama',
      durationMs: 450,
    });

    const { result } = renderHook(() => useTranslation());

    await act(async () => {
      await result.current.translate('Hello');
    });

    expect(result.current.originalText).toBe('Hello');
    expect(result.current.translatedText).toBe('こんにちは');
    expect(result.current.error).toBeNull();

    expect(invoke).toHaveBeenCalledWith('translate', {
      text: 'Hello',
      sourceLang: 'english',
      targetLang: 'japanese',
    });
  });

  it('低信頼度の言語検出ではデフォルト方向（日→英）を使用する', async () => {
    vi.mocked(detectLanguage).mockReturnValue({
      language: 'en',
      confidence: 0.3, // 低信頼度
    });

    vi.mocked(invoke).mockResolvedValue({
      translatedText: 'Test translation',
      sourceLang: 'japanese',
      targetLang: 'english',
      provider: 'ollama',
      durationMs: 300,
    });

    const { result } = renderHook(() => useTranslation());

    await act(async () => {
      await result.current.translate('テスト');
    });

    // 低信頼度の場合はデフォルト（日→英）が使用される
    expect(invoke).toHaveBeenCalledWith('translate', {
      text: 'テスト',
      sourceLang: 'japanese',
      targetLang: 'english',
    });
  });

  it('翻訳中はisLoadingがtrueになる', async () => {
    vi.mocked(detectLanguage).mockReturnValue({
      language: 'ja',
      confidence: 0.9,
    });

    let resolvePromise: (value: unknown) => void;
    const pendingPromise = new Promise((resolve) => {
      resolvePromise = resolve;
    });
    vi.mocked(invoke).mockReturnValue(pendingPromise);

    const { result } = renderHook(() => useTranslation());

    act(() => {
      void result.current.translate('テスト');
    });

    await waitFor(() => {
      expect(result.current.isLoading).toBe(true);
    });

    await act(async () => {
      resolvePromise!({
        translatedText: 'Test',
        sourceLang: 'japanese',
        targetLang: 'english',
        provider: 'ollama',
        durationMs: 100,
      });
    });

    expect(result.current.isLoading).toBe(false);
  });

  it('翻訳エラー時はerrorが設定される', async () => {
    vi.mocked(detectLanguage).mockReturnValue({
      language: 'ja',
      confidence: 0.9,
    });

    vi.mocked(invoke).mockRejectedValue('接続に失敗しました');

    const { result } = renderHook(() => useTranslation());

    await act(async () => {
      await result.current.translate('こんにちは');
    });

    expect(result.current.error).toBe('接続に失敗しました');
    expect(result.current.translatedText).toBeNull();
    expect(result.current.isLoading).toBe(false);
  });

  it('resetで状態を初期化できる', async () => {
    vi.mocked(detectLanguage).mockReturnValue({
      language: 'ja',
      confidence: 0.9,
    });

    vi.mocked(invoke).mockResolvedValue({
      translatedText: 'Hello',
      sourceLang: 'japanese',
      targetLang: 'english',
      provider: 'ollama',
      durationMs: 500,
    });

    const { result } = renderHook(() => useTranslation());

    await act(async () => {
      await result.current.translate('こんにちは');
    });

    expect(result.current.translatedText).toBe('Hello');

    act(() => {
      result.current.reset();
    });

    expect(result.current.isLoading).toBe(false);
    expect(result.current.originalText).toBe('');
    expect(result.current.translatedText).toBeNull();
    expect(result.current.error).toBeNull();
  });

  it('空のテキストでは翻訳を実行しない', async () => {
    const { result } = renderHook(() => useTranslation());

    await act(async () => {
      await result.current.translate('');
    });

    expect(invoke).not.toHaveBeenCalled();
    expect(result.current.error).toBe('翻訳するテキストが入力されていません');
  });

  it('空白のみのテキストでは翻訳を実行しない', async () => {
    const { result } = renderHook(() => useTranslation());

    await act(async () => {
      await result.current.translate('   ');
    });

    expect(invoke).not.toHaveBeenCalled();
    expect(result.current.error).toBe('翻訳するテキストが入力されていません');
  });
});
