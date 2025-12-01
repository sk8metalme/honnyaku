/**
 * 言語タイプ定義（フロントエンド用）
 */
export type Language = 'ja' | 'en';

/**
 * 言語タイプ定義（バックエンドAPI用）
 */
export type BackendLanguage = 'japanese' | 'english';

/**
 * 言語検出結果
 */
export interface DetectionResult {
  language: Language;
  /** 信頼度スコア (0.0 - 1.0) */
  confidence: number;
}

/**
 * アプリケーション設定
 */
export interface AppSettings {
  /** グローバルショートカット */
  shortcut: string;
  /** Ollamaモデル名 */
  ollamaModel: string;
  /** Ollamaエンドポイント */
  ollamaEndpoint: string;
}

/**
 * プロバイダー接続状態
 */
export type ProviderStatus =
  | { status: 'available' }
  | { status: 'unavailable'; reason: string };

/**
 * 翻訳結果（バックエンドから返される）
 */
export interface TranslationResult {
  translatedText: string;
  sourceLang: BackendLanguage;
  targetLang: BackendLanguage;
  durationMs: number;
}

/**
 * デフォルト設定値
 */
export const DEFAULT_SETTINGS: AppSettings = {
  shortcut: 'CommandOrControl+J',
  ollamaModel: 'qwen2.5:3b',
  ollamaEndpoint: 'http://localhost:11434',
};

/**
 * フロントエンド言語をバックエンド言語に変換
 */
export function toBackendLanguage(lang: Language): BackendLanguage {
  return lang === 'ja' ? 'japanese' : 'english';
}

/**
 * バックエンド言語をフロントエンド言語に変換
 */
export function toFrontendLanguage(lang: BackendLanguage): Language {
  return lang === 'japanese' ? 'ja' : 'en';
}
