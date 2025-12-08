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

// claude-cliをモック
vi.mock('@/lib/claude-cli', () => ({
  translateWithClaudeCLI: vi.fn(),
}));

// useSettingsをモック
vi.mock('@/hooks/useSettings', () => ({
  useSettings: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';
import { detectLanguage } from '@/lib/language-detect';
import { translateWithClaudeCLI } from '@/lib/claude-cli';
import { useSettings } from '@/hooks/useSettings';

describe('useTranslation', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    // useSettingsのデフォルトモック（Ollama provider）
    vi.mocked(useSettings).mockReturnValue({
      settings: {
        shortcut: 'CommandOrControl+J',
        ollamaModel: 'qwen2.5:3b',
        ollamaEndpoint: 'http://localhost:11434',
        provider: 'ollama',
        claudeCliPath: null,
      },
      isLoading: false,
      error: null,
      updateSettings: vi.fn(),
      resetSettings: vi.fn(),
      reloadSettings: vi.fn(),
      checkProviderStatus: vi.fn(),
    });
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

  describe('Provider分岐ロジック', () => {
    it('provider="ollama"の場合、既存のOllama翻訳が実行される', async () => {
      // Ollama providerの設定（デフォルト）
      vi.mocked(useSettings).mockReturnValue({
        settings: {
          shortcut: 'CommandOrControl+J',
          ollamaModel: 'qwen2.5:3b',
          ollamaEndpoint: 'http://localhost:11434',
          provider: 'ollama',
          claudeCliPath: null,
        },
        isLoading: false,
        error: null,
        updateSettings: vi.fn(),
        resetSettings: vi.fn(),
        reloadSettings: vi.fn(),
        checkProviderStatus: vi.fn(),
      });

      vi.mocked(detectLanguage).mockReturnValue({
        language: 'ja',
        confidence: 0.9,
      });

      vi.mocked(invoke).mockResolvedValue({
        translatedText: 'Hello',
        sourceLang: 'japanese',
        targetLang: 'english',
        durationMs: 500,
      });

      const { result } = renderHook(() => useTranslation());

      await act(async () => {
        await result.current.translate('こんにちは');
      });

      // Ollama翻訳（invoke('translate')）が呼ばれることを確認
      expect(invoke).toHaveBeenCalledWith('translate', {
        text: 'こんにちは',
        sourceLang: 'japanese',
        targetLang: 'english',
      });

      // Claude CLI翻訳は呼ばれないことを確認
      expect(translateWithClaudeCLI).not.toHaveBeenCalled();

      expect(result.current.translatedText).toBe('Hello');
      expect(result.current.error).toBeNull();
    });

    it('provider="claude-cli"の場合、Claude CLI翻訳が実行される', async () => {
      // Claude CLI providerの設定
      vi.mocked(useSettings).mockReturnValue({
        settings: {
          shortcut: 'CommandOrControl+J',
          ollamaModel: 'qwen2.5:3b',
          ollamaEndpoint: 'http://localhost:11434',
          provider: 'claude-cli',
          claudeCliPath: '/opt/homebrew/bin/claude',
        },
        isLoading: false,
        error: null,
        updateSettings: vi.fn(),
        resetSettings: vi.fn(),
        reloadSettings: vi.fn(),
        checkProviderStatus: vi.fn(),
      });

      vi.mocked(detectLanguage).mockReturnValue({
        language: 'en',
        confidence: 0.95,
      });

      vi.mocked(translateWithClaudeCLI).mockResolvedValue({
        translatedText: 'こんにちは',
        sourceLang: 'english',
        targetLang: 'japanese',
        durationMs: 3000,
      });

      const { result } = renderHook(() => useTranslation());

      await act(async () => {
        await result.current.translate('Hello');
      });

      // Claude CLI翻訳が呼ばれることを確認
      expect(translateWithClaudeCLI).toHaveBeenCalledWith(
        'Hello',
        'english',
        'japanese'
      );

      // Ollama翻訳（invoke('translate')）は呼ばれないことを確認
      expect(invoke).not.toHaveBeenCalled();

      expect(result.current.translatedText).toBe('こんにちは');
      expect(result.current.error).toBeNull();
    });

    it('Claude CLI翻訳失敗時、エラーメッセージが設定される', async () => {
      // Claude CLI providerの設定
      vi.mocked(useSettings).mockReturnValue({
        settings: {
          shortcut: 'CommandOrControl+J',
          ollamaModel: 'qwen2.5:3b',
          ollamaEndpoint: 'http://localhost:11434',
          provider: 'claude-cli',
          claudeCliPath: null,
        },
        isLoading: false,
        error: null,
        updateSettings: vi.fn(),
        resetSettings: vi.fn(),
        reloadSettings: vi.fn(),
        checkProviderStatus: vi.fn(),
      });

      vi.mocked(detectLanguage).mockReturnValue({
        language: 'ja',
        confidence: 0.9,
      });

      vi.mocked(translateWithClaudeCLI).mockRejectedValue(
        new Error('Claude CLI翻訳に失敗しました: タイムアウト')
      );

      const { result } = renderHook(() => useTranslation());

      await act(async () => {
        await result.current.translate('こんにちは');
      });

      expect(result.current.error).toBe(
        'Claude CLI翻訳に失敗しました: タイムアウト'
      );
      expect(result.current.translatedText).toBeNull();
    });
  });
});
