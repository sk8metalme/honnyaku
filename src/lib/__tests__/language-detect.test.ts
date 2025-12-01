/**
 * 言語検出ライブラリのテスト
 */

import { describe, it, expect } from 'vitest';
import {
  detectLanguage,
  getTargetLanguage,
  getLanguageDisplayName,
} from '../language-detect';

describe('detectLanguage', () => {
  describe('日本語テキストの検出', () => {
    it('ひらがなのみのテキストを日本語と判定する', () => {
      const result = detectLanguage('こんにちは');
      expect(result.language).toBe('ja');
      expect(result.confidence).toBeGreaterThan(0.5);
    });

    it('カタカナのみのテキストを日本語と判定する', () => {
      const result = detectLanguage('コンニチハ');
      expect(result.language).toBe('ja');
      expect(result.confidence).toBeGreaterThan(0.5);
    });

    it('漢字のみのテキストを日本語と判定する', () => {
      const result = detectLanguage('翻訳機能');
      expect(result.language).toBe('ja');
      expect(result.confidence).toBeGreaterThan(0.5);
    });

    it('混合テキスト（日本語+英語）を日本語と判定する', () => {
      const result = detectLanguage(
        'このAPIはとても使いやすいです。'
      );
      expect(result.language).toBe('ja');
      expect(result.confidence).toBeGreaterThan(0.5);
    });

    it('長い日本語テキストを高い信頼度で日本語と判定する', () => {
      const result = detectLanguage(
        '本日は晴天なり。天気予報によると、明日も晴れるそうです。'
      );
      expect(result.language).toBe('ja');
      expect(result.confidence).toBeGreaterThan(0.7);
    });
  });

  describe('英語テキストの検出', () => {
    it('英語のみのテキストを英語と判定する', () => {
      const result = detectLanguage('Hello, World!');
      expect(result.language).toBe('en');
      expect(result.confidence).toBeGreaterThan(0.5);
    });

    it('長い英語テキストを高い信頼度で英語と判定する', () => {
      const result = detectLanguage(
        'The quick brown fox jumps over the lazy dog. This is a common pangram.'
      );
      expect(result.language).toBe('en');
      expect(result.confidence).toBeGreaterThan(0.7);
    });

    it('技術的な英語テキストを英語と判定する', () => {
      const result = detectLanguage(
        'function calculateSum(a, b) { return a + b; }'
      );
      expect(result.language).toBe('en');
      expect(result.confidence).toBeGreaterThan(0.5);
    });
  });

  describe('短いテキストの処理', () => {
    it('短い日本語（1文字）を日本語と判定する', () => {
      const result = detectLanguage('あ');
      expect(result.language).toBe('ja');
    });

    it('短い英語（1単語）を英語と判定する', () => {
      const result = detectLanguage('Hello');
      expect(result.language).toBe('en');
    });

    it('短いテキストは信頼度が低め', () => {
      const shortJa = detectLanguage('あ');
      const shortEn = detectLanguage('Hi');
      // 短いテキストは信頼度が1.0未満
      expect(shortJa.confidence).toBeLessThan(1.0);
      expect(shortEn.confidence).toBeLessThan(1.0);
    });
  });

  describe('境界値テスト', () => {
    it('空文字列はデフォルトで日本語、信頼度0と判定する', () => {
      const result = detectLanguage('');
      expect(result.language).toBe('ja');
      expect(result.confidence).toBe(0);
    });

    it('空白のみはデフォルトで日本語、信頼度0と判定する', () => {
      const result = detectLanguage('   ');
      expect(result.language).toBe('ja');
      expect(result.confidence).toBe(0);
    });

    it('数字のみのテキストを英語と判定する', () => {
      const result = detectLanguage('12345');
      expect(result.language).toBe('en');
    });

    it('記号のみのテキストを英語と判定する', () => {
      const result = detectLanguage('!@#$%');
      expect(result.language).toBe('en');
    });

    it('日本語句読点を含むテキストを日本語と判定する', () => {
      const result = detectLanguage('こんにちは。');
      expect(result.language).toBe('ja');
    });
  });

  describe('10文字境界テスト', () => {
    it('9文字の日本語テキストを日本語と判定する（短文処理）', () => {
      const result = detectLanguage('こんにちは世界！'); // 8文字
      expect(result.language).toBe('ja');
    });

    it('9文字の英語テキストを英語と判定する（短文処理）', () => {
      const result = detectLanguage('Hello!!!'); // 8文字
      expect(result.language).toBe('en');
    });

    it('10文字以上の日本語テキストは通常処理', () => {
      const result = detectLanguage('こんにちは世界です！'); // 10文字
      expect(result.language).toBe('ja');
      expect(result.confidence).toBeGreaterThan(0.5);
    });
  });
});

describe('getTargetLanguage', () => {
  it('日本語検出時は英語を返す', () => {
    const detection = { language: 'ja' as const, confidence: 1.0 };
    expect(getTargetLanguage(detection)).toBe('en');
  });

  it('英語検出時は日本語を返す', () => {
    const detection = { language: 'en' as const, confidence: 1.0 };
    expect(getTargetLanguage(detection)).toBe('ja');
  });
});

describe('getLanguageDisplayName', () => {
  it('jaは「日本語」を返す', () => {
    expect(getLanguageDisplayName('ja')).toBe('日本語');
  });

  it('enは「英語」を返す', () => {
    expect(getLanguageDisplayName('en')).toBe('英語');
  });
});
