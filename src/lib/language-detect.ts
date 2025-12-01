/**
 * 言語検出ライブラリ
 *
 * テキストの言語を日本語/英語で判定し、信頼度スコアを返す。
 * Unicode文字範囲を使用して日本語文字（ひらがな、カタカナ、漢字）を検出する。
 */

import type { Language, DetectionResult } from '@/types';

/**
 * Unicode文字範囲定義（参照用にエクスポート）
 */
export const UNICODE_RANGES = {
  // ひらがな: U+3040-U+309F
  hiragana: /[\u3040-\u309F]/g,
  // カタカナ: U+30A0-U+30FF
  katakana: /[\u30A0-\u30FF]/g,
  // CJK統合漢字: U+4E00-U+9FFF
  kanji: /[\u4E00-\u9FFF]/g,
  // 全角記号・句読点: U+3000-U+303F
  jpPunctuation: /[\u3000-\u303F]/g,
  // 半角カタカナ: U+FF65-U+FF9F
  halfwidthKatakana: /[\uFF65-\uFF9F]/g,
} as const;

/**
 * 日本語文字を検出する正規表現
 */
const JAPANESE_CHAR_REGEX = new RegExp(
  `[${'\u3040-\u309F'}${'\u30A0-\u30FF'}${'\u4E00-\u9FFF'}${'\u3000-\u303F'}${'\uFF65-\uFF9F'}]`,
  'g'
);

/**
 * テキストから日本語文字の割合を算出
 *
 * @param text - 検査対象のテキスト
 * @returns 日本語文字の割合 (0.0 - 1.0)
 */
function calculateJapaneseRatio(text: string): number {
  // 空白と改行を除去した文字列
  const cleanedText = text.replace(/[\s\n\r]/g, '');

  if (cleanedText.length === 0) {
    return 0;
  }

  // 日本語文字のマッチ
  const jpMatches = cleanedText.match(JAPANESE_CHAR_REGEX);
  const jpCount = jpMatches ? jpMatches.length : 0;

  return jpCount / cleanedText.length;
}

/**
 * 短いテキストに対する特別な日本語判定
 *
 * 10文字未満のテキストでは、1文字でも日本語があれば日本語と判定
 *
 * @param text - 検査対象のテキスト
 * @returns 日本語文字が含まれているかどうか
 */
function hasJapaneseCharacter(text: string): boolean {
  return JAPANESE_CHAR_REGEX.test(text);
}

/**
 * テキストの言語を検出する
 *
 * 日本語と英語を判定し、信頼度スコアを返す。
 * - ひらがな、カタカナ、漢字の存在で日本語を判定
 * - 信頼度 < 0.5 の場合はデフォルト（日→英）方向を使用
 *
 * @param text - 検出対象のテキスト
 * @returns 言語と信頼度スコアを含む検出結果
 *
 * @example
 * ```ts
 * const result = detectLanguage('Hello, World!');
 * // { language: 'en', confidence: 0.95 }
 *
 * const result2 = detectLanguage('こんにちは');
 * // { language: 'ja', confidence: 1.0 }
 * ```
 */
export function detectLanguage(text: string): DetectionResult {
  // 空テキストまたは空白のみの場合
  const trimmedText = text.trim();
  if (trimmedText.length === 0) {
    // デフォルトは日本語（日→英翻訳）
    return { language: 'ja', confidence: 0 };
  }

  // 短いテキスト（10文字未満）の特別処理
  if (trimmedText.length < 10) {
    if (hasJapaneseCharacter(trimmedText)) {
      // 日本語文字があれば日本語と判定
      const ratio = calculateJapaneseRatio(trimmedText);
      // 短いテキストでは信頼度を少し下げる
      const confidence = Math.min(0.8, 0.5 + ratio * 0.5);
      return { language: 'ja', confidence };
    } else {
      // 日本語文字がなければ英語と判定
      // 短いテキストでは信頼度を少し下げる
      return { language: 'en', confidence: 0.7 };
    }
  }

  // 通常のテキスト（10文字以上）
  const japaneseRatio = calculateJapaneseRatio(trimmedText);

  // 日本語文字の割合が10%以上なら日本語と判定
  if (japaneseRatio >= 0.1) {
    // 日本語の信頼度を算出
    // 割合が高いほど信頼度も高い
    const confidence = Math.min(1.0, 0.5 + japaneseRatio);
    return { language: 'ja', confidence };
  } else {
    // 日本語文字が少なければ英語と判定
    // 日本語割合が低いほど信頼度が高い
    const confidence = Math.min(1.0, 0.6 + (1 - japaneseRatio) * 0.4);
    return { language: 'en', confidence };
  }
}

/**
 * 言語検出結果から翻訳先言語を決定
 *
 * @param detection - 言語検出結果
 * @returns 翻訳先言語
 */
export function getTargetLanguage(detection: DetectionResult): Language {
  // 検出された言語の反対を翻訳先とする
  return detection.language === 'ja' ? 'en' : 'ja';
}

/**
 * 言語コードを表示名に変換
 *
 * @param lang - 言語コード
 * @returns 言語の表示名
 */
export function getLanguageDisplayName(lang: Language): string {
  return lang === 'ja' ? '日本語' : '英語';
}
