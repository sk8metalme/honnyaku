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
 * 翻訳プロバイダー
 */
export type TranslationProvider = 'ollama' | 'claude-cli';

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
  /** 翻訳プロバイダー */
  provider: TranslationProvider;
  /** Claude CLIの実行パス */
  claudeCliPath: string | null;
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
  provider: 'ollama',
  claudeCliPath: null,
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

/**
 * ストリーミングチャンクイベント
 */
export interface StreamChunk {
  chunk: string;
  accumulated: string;
  done: boolean;
}

/**
 * ストリーミング完了イベント
 */
export interface StreamComplete {
  translatedText: string;
  durationMs: number;
}

/**
 * 要約結果（バックエンドから返される）
 */
export interface SummarizeResult {
  /** 要約テキスト */
  summary: string;
  /** 元テキストの文字数 */
  originalLength: number;
  /** 要約テキストの文字数 */
  summaryLength: number;
  /** 処理時間（ミリ秒） */
  durationMs: number;
}

/**
 * 返信結果（バックエンドから返される）
 */
export interface ReplyResult {
  /** 返信テキスト（翻訳先言語）
   * 例: 日→英翻訳の場合は英語の返信 */
  reply: string;
  /** 返信の翻訳（翻訳元言語）
   * 例: 日→英翻訳の場合は日本語の返信（上記replyを翻訳したもの） */
  explanation: string;
  /** 返信の言語（翻訳先言語） */
  language: BackendLanguage;
  /** 処理時間（ミリ秒） */
  durationMs: number;
}
