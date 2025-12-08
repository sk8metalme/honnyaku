/**
 * Claude CLI翻訳クライアントのテスト
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { translateWithClaudeCLI } from '../claude-cli';
import type { TranslationResult } from '@/types';

// Tauri APIをモック
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// モックしたinvoke関数をインポート
import { invoke } from '@tauri-apps/api/core';
const mockInvoke = vi.mocked(invoke);

describe('translateWithClaudeCLI', () => {
  beforeEach(() => {
    // 各テスト前にモックをリセット
    vi.clearAllMocks();
  });

  describe('正常系', () => {
    it('翻訳が成功した場合、TranslationResultを返す', async () => {
      // モックレスポンスの設定
      const mockResult: TranslationResult = {
        translatedText: 'こんにちは、お元気ですか?',
        sourceLang: 'english',
        targetLang: 'japanese',
        durationMs: 5000,
      };
      mockInvoke.mockResolvedValue(mockResult);

      // テスト実行
      const result = await translateWithClaudeCLI(
        'Hello, how are you?',
        'english',
        'japanese'
      );

      // 検証
      expect(result).toEqual(mockResult);
      expect(mockInvoke).toHaveBeenCalledWith('translate_with_claude_cli', {
        text: 'Hello, how are you?',
        sourceLang: 'english',
        targetLang: 'japanese',
      });
      expect(mockInvoke).toHaveBeenCalledTimes(1);
    });

    it('日本語から英語への翻訳が成功する', async () => {
      // モックレスポンスの設定
      const mockResult: TranslationResult = {
        translatedText: 'Good morning',
        sourceLang: 'japanese',
        targetLang: 'english',
        durationMs: 4500,
      };
      mockInvoke.mockResolvedValue(mockResult);

      // テスト実行
      const result = await translateWithClaudeCLI(
        'おはようございます',
        'japanese',
        'english'
      );

      // 検証
      expect(result.translatedText).toBe('Good morning');
      expect(result.sourceLang).toBe('japanese');
      expect(result.targetLang).toBe('english');
    });

    it('翻訳時間が記録される', async () => {
      // モックレスポンスの設定
      const mockResult: TranslationResult = {
        translatedText: 'Translation result',
        sourceLang: 'japanese',
        targetLang: 'english',
        durationMs: 3000,
      };
      mockInvoke.mockResolvedValue(mockResult);

      // テスト実行
      const result = await translateWithClaudeCLI(
        'テスト',
        'japanese',
        'english'
      );

      // 検証
      expect(result.durationMs).toBeGreaterThan(0);
      expect(result.durationMs).toBe(3000);
    });
  });

  describe('異常系', () => {
    it('IPC通信エラー時にエラーをthrowする', async () => {
      // モックでエラーを発生させる
      const ipcError = new Error('IPC通信に失敗しました');
      mockInvoke.mockRejectedValue(ipcError);

      // テスト実行と検証
      await expect(
        translateWithClaudeCLI('Hello', 'english', 'japanese')
      ).rejects.toThrow('Claude CLI翻訳に失敗しました: IPC通信に失敗しました');

      expect(mockInvoke).toHaveBeenCalledTimes(1);
    });

    it('CLI実行エラー時に適切なエラーメッセージをthrowする', async () => {
      // モックでCLI実行エラーを発生させる
      const cliError = new Error('Claude CLIがエラーで終了しました');
      mockInvoke.mockRejectedValue(cliError);

      // テスト実行と検証
      await expect(
        translateWithClaudeCLI('Test text', 'english', 'japanese')
      ).rejects.toThrow('Claude CLI翻訳に失敗しました');
    });

    it('タイムアウトエラー時にエラーをthrowする', async () => {
      // モックでタイムアウトエラーを発生させる
      const timeoutError = new Error('翻訳リクエストがタイムアウトしました');
      mockInvoke.mockRejectedValue(timeoutError);

      // テスト実行と検証
      await expect(
        translateWithClaudeCLI('Long text...', 'english', 'japanese')
      ).rejects.toThrow('Claude CLI翻訳に失敗しました');
    });

    it('文字列エラーの場合も適切にハンドリングする', async () => {
      // モックで文字列エラーを発生させる（Errorオブジェクトではない）
      mockInvoke.mockRejectedValue('Unknown error');

      // テスト実行と検証
      await expect(
        translateWithClaudeCLI('Test', 'english', 'japanese')
      ).rejects.toThrow('Claude CLI翻訳に失敗しました: Unknown error');
    });
  });

  describe('境界値テスト', () => {
    it('空文字列でも翻訳リクエストが送信される', async () => {
      const mockResult: TranslationResult = {
        translatedText: '',
        sourceLang: 'english',
        targetLang: 'japanese',
        durationMs: 100,
      };
      mockInvoke.mockResolvedValue(mockResult);

      const result = await translateWithClaudeCLI('', 'english', 'japanese');

      expect(mockInvoke).toHaveBeenCalledWith('translate_with_claude_cli', {
        text: '',
        sourceLang: 'english',
        targetLang: 'japanese',
      });
      expect(result.translatedText).toBe('');
    });

    it('長文テキストでも正常に処理される', async () => {
      const longText = 'A'.repeat(10000);
      const mockResult: TranslationResult = {
        translatedText: 'A'.repeat(10000),
        sourceLang: 'english',
        targetLang: 'japanese',
        durationMs: 15000,
      };
      mockInvoke.mockResolvedValue(mockResult);

      const result = await translateWithClaudeCLI(
        longText,
        'english',
        'japanese'
      );

      expect(result.translatedText).toHaveLength(10000);
    });
  });
});
