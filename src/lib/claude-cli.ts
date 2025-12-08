/**
 * Claude CLI翻訳クライアント
 *
 * Tauri IPCを通じてClaude CLI翻訳を実行する機能を提供
 */

import { invoke } from '@tauri-apps/api/core';
import type { BackendLanguage, TranslationResult } from '@/types';

/**
 * Claude CLIで翻訳を実行
 *
 * @param text - 翻訳するテキスト
 * @param sourceLang - 翻訳元言語
 * @param targetLang - 翻訳先言語
 * @returns 翻訳結果のPromise
 * @throws Tauri IPC通信エラーまたはCLI実行エラー
 */
export async function translateWithClaudeCLI(
  text: string,
  sourceLang: BackendLanguage,
  targetLang: BackendLanguage
): Promise<TranslationResult> {
  try {
    const result = await invoke<TranslationResult>('translate_with_claude_cli', {
      text,
      sourceLang,
      targetLang,
    });
    return result;
  } catch (error) {
    // Tauri IPCエラーを適切なエラーメッセージに変換
    if (error instanceof Error) {
      throw new Error(`Claude CLI翻訳に失敗しました: ${error.message}`);
    }
    throw new Error(`Claude CLI翻訳に失敗しました: ${String(error)}`);
  }
}
