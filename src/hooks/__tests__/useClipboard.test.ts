/**
 * useClipboard Hook Tests
 */

import { renderHook, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useClipboard } from '../useClipboard';
import type { ClipboardContent } from '../useClipboard';

// Tauriのinvokeをモック
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';
const mockInvoke = vi.mocked(invoke);

describe('useClipboard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('readText', () => {
    it('クリップボードからテキストを読み取れること', async () => {
      const mockContent: ClipboardContent = {
        text: 'Hello, World!',
        success: true,
      };
      mockInvoke.mockResolvedValueOnce(mockContent);

      const { result } = renderHook(() => useClipboard());

      let content: ClipboardContent | undefined;
      await act(async () => {
        content = await result.current.readText();
      });

      expect(mockInvoke).toHaveBeenCalledWith('read_clipboard');
      expect(content).toEqual(mockContent);
    });

    it('読み取り失敗時に空のコンテンツを返すこと', async () => {
      mockInvoke.mockRejectedValueOnce(new Error('読み取りエラー'));

      const { result } = renderHook(() => useClipboard());

      let content: ClipboardContent | undefined;
      await act(async () => {
        content = await result.current.readText();
      });

      expect(content).toEqual({
        text: '',
        success: false,
      });
    });

    it('空のクリップボードを正しく処理すること', async () => {
      const mockContent: ClipboardContent = {
        text: '',
        success: false,
      };
      mockInvoke.mockResolvedValueOnce(mockContent);

      const { result } = renderHook(() => useClipboard());

      let content: ClipboardContent | undefined;
      await act(async () => {
        content = await result.current.readText();
      });

      expect(content).toEqual(mockContent);
    });
  });

  describe('writeText', () => {
    it('クリップボードにテキストを書き込めること', async () => {
      mockInvoke.mockResolvedValueOnce(undefined);

      const { result } = renderHook(() => useClipboard());

      await act(async () => {
        await result.current.writeText('書き込むテキスト');
      });

      expect(mockInvoke).toHaveBeenCalledWith('write_clipboard', {
        text: '書き込むテキスト',
      });
    });

    it('書き込み失敗時にエラーをスローすること', async () => {
      const error = new Error('書き込みエラー');
      mockInvoke.mockRejectedValueOnce(error);

      const { result } = renderHook(() => useClipboard());

      await expect(
        act(async () => {
          await result.current.writeText('テスト');
        })
      ).rejects.toThrow('書き込みエラー');
    });

    it('日本語テキストを正しく書き込めること', async () => {
      mockInvoke.mockResolvedValueOnce(undefined);

      const { result } = renderHook(() => useClipboard());

      await act(async () => {
        await result.current.writeText('こんにちは、世界！');
      });

      expect(mockInvoke).toHaveBeenCalledWith('write_clipboard', {
        text: 'こんにちは、世界！',
      });
    });
  });

  describe('getSelectedText', () => {
    it('選択テキストを取得できること', async () => {
      const mockContent: ClipboardContent = {
        text: '選択されたテキスト',
        success: true,
      };
      mockInvoke.mockResolvedValueOnce(mockContent);

      const { result } = renderHook(() => useClipboard());

      let content: ClipboardContent | undefined;
      await act(async () => {
        content = await result.current.getSelectedText();
      });

      expect(mockInvoke).toHaveBeenCalledWith('get_selected_text');
      expect(content).toEqual(mockContent);
    });

    it('選択テキスト取得失敗時に空のコンテンツを返すこと', async () => {
      mockInvoke.mockRejectedValueOnce(new Error('権限エラー'));

      const { result } = renderHook(() => useClipboard());

      let content: ClipboardContent | undefined;
      await act(async () => {
        content = await result.current.getSelectedText();
      });

      expect(content).toEqual({
        text: '',
        success: false,
      });
    });

    it('選択なしの場合に空のコンテンツを返すこと', async () => {
      const mockContent: ClipboardContent = {
        text: '',
        success: false,
      };
      mockInvoke.mockResolvedValueOnce(mockContent);

      const { result } = renderHook(() => useClipboard());

      let content: ClipboardContent | undefined;
      await act(async () => {
        content = await result.current.getSelectedText();
      });

      expect(content).toEqual(mockContent);
    });
  });

  describe('Hookの安定性', () => {
    it('関数参照が安定していること', () => {
      const { result, rerender } = renderHook(() => useClipboard());

      const firstReadText = result.current.readText;
      const firstWriteText = result.current.writeText;
      const firstGetSelectedText = result.current.getSelectedText;

      rerender();

      expect(result.current.readText).toBe(firstReadText);
      expect(result.current.writeText).toBe(firstWriteText);
      expect(result.current.getSelectedText).toBe(firstGetSelectedText);
    });
  });
});
