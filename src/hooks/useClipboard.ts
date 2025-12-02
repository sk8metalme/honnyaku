/**
 * クリップボード管理Hook
 *
 * クリップボードの読み取り・書き込みと選択テキストの取得を管理する
 */

import { useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * クリップボードコンテンツ
 */
export interface ClipboardContent {
  /** テキスト内容 */
  text: string;
  /** 読み取り成功かどうか */
  success: boolean;
}

/**
 * useClipboard Hookの戻り値
 */
export interface UseClipboardReturn {
  /** クリップボードからテキストを読み取る */
  readText: () => Promise<ClipboardContent>;
  /** クリップボードにテキストを書き込む */
  writeText: (text: string) => Promise<void>;
  /** 選択テキストを取得する（Cmd+Cを送信） */
  getSelectedText: () => Promise<ClipboardContent>;
}

/**
 * クリップボード管理Hook
 *
 * @returns クリップボード操作関数
 *
 * @example
 * ```tsx
 * function App() {
 *   const { readText, writeText, getSelectedText } = useClipboard();
 *
 *   const handleCopy = async () => {
 *     await writeText('コピーするテキスト');
 *   };
 *
 *   const handleGetSelection = async () => {
 *     const content = await getSelectedText();
 *     if (content.success) {
 *       console.log('選択テキスト:', content.text);
 *     }
 *   };
 *
 *   return (
 *     <button onClick={handleGetSelection}>選択テキストを取得</button>
 *   );
 * }
 * ```
 */
export function useClipboard(): UseClipboardReturn {
  /**
   * クリップボードからテキストを読み取る
   */
  const readText = useCallback(async (): Promise<ClipboardContent> => {
    try {
      const content = await invoke<ClipboardContent>('read_clipboard');
      return content;
    } catch (err) {
      console.error('Failed to read clipboard:', err);
      return {
        text: '',
        success: false,
      };
    }
  }, []);

  /**
   * クリップボードにテキストを書き込む
   */
  const writeText = useCallback(async (text: string): Promise<void> => {
    try {
      await invoke('write_clipboard', { text });
    } catch (err) {
      console.error('Failed to write to clipboard:', err);
      throw err;
    }
  }, []);

  /**
   * 選択テキストを取得する
   *
   * アクセシビリティ権限が必要。
   * Cmd+Cを送信してクリップボードから読み取る。
   */
  const getSelectedText = useCallback(async (): Promise<ClipboardContent> => {
    try {
      const content = await invoke<ClipboardContent>('get_selected_text');
      return content;
    } catch (err) {
      console.error('Failed to get selected text:', err);
      return {
        text: '',
        success: false,
      };
    }
  }, []);

  return {
    readText,
    writeText,
    getSelectedText,
  };
}
