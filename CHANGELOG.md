# Changelog

このプロジェクトのすべての注目すべき変更は、このファイルに記録されます。

フォーマットは [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいており、
このプロジェクトは [Semantic Versioning](https://semver.org/lang/ja/) に準拠しています。

## [Unreleased]

## [0.4.0] - 2025-12-06

### 追加
- **翻訳要約・返信機能**
  - 翻訳ポップアップに要約・返信ボタンと結果表示エリアを追加
  - 要約・返信機能のUX改善
  - 小さいモデルでの要約・返信機能を制限

### 修正
- ポップアップコンテンツのスクロール対応
- 新規翻訳時に要約・返信結果をリセット
- 返信案の説明を翻訳に変更

## [0.3.0] - 2025-12-04

### 追加
- **翻訳速度向上と翻訳時間表示機能**
  - 翻訳処理の最適化
  - 翻訳完了時間の表示

### 修正
- CI/CDエラー修正（テストとLinting）
- コードフォーマットの統一（Rustfmt、Prettier）

## [0.2.0] - 2025-12-03

### 追加
- **PLaMo-2-Translateモデル対応**
  - Preferred Networks社のPLaMo-2-Translateモデルをサポート
  - UI状態同期の改善

- **CI/CDパイプライン構築**
  - GitHub Actions CI/CDパイプライン構築
  - PR時の自動テスト実行（フロントエンド/バックエンド/Linter）
  - タグpush時の自動リリースビルドとGitHub Release作成
  - Intel Mac対応のリリースワークフロー改善

- **ドキュメント改善**
  - READMEにデモGIF追加
  - ライセンス整備

## [0.1.0] - 2025-12-02

### 追加
- **グローバルショートカット翻訳機能**
  - デフォルトショートカット `Cmd+J` で選択テキストを即座に翻訳
  - カスタマイズ可能なショートカットキー設定

- **自動言語検出**
  - 日本語 ↔ 英語の自動言語検出と翻訳方向決定
  - 高精度な言語検出アルゴリズム

- **プライバシー重視のローカルLLM対応**
  - Ollama（ローカルLLM）を使用した完全オフライン翻訳
  - 翻訳データが外部サーバーに送信されない設計

- **軽量設計**
  - アプリサイズ: 4.8MB
  - メモリ使用量: 100MB以下
  - 高速起動とレスポンス

- **便利な機能**
  - ワンクリックコピー機能
  - ポップアップ表示（作業を邪魔しない設計）
  - Ollamaエンドポイント/モデル/ショートカットのカスタマイズ
  - Ollamaモデルプリロード機能（初回翻訳高速化）

- **包括的なテストカバレッジ**
  - フロントエンド単体テスト: 69テスト
  - バックエンド単体テスト: 26テスト
  - E2E統合テスト: 10テストケース
  - テストカバレッジ95%以上

- **充実したドキュメント**
  - README.md: プロジェクト概要と導入ガイド
  - CONTRIBUTING.md: 貢献者向け開発ガイド
  - docs/USER_MANUAL.md: 詳細なユーザーマニュアル

- **開発環境サポート**
  - Kiro Spec-Driven Development フレームワーク統合
  - Claude Code エージェント設定（Designer/Developer/Manager/Tester）
  - テスト仕様書（E2E/Integration/Performance/Security/Unit）

### 技術スタック
- フロントエンド: React 19 + TypeScript 5 + Tailwind CSS 4
- バックエンド: Tauri v2 (Rust 1.77+)
- LLM: Ollama（ローカルLLM）
- テスト: Vitest + Cargo Test + WebdriverIO
- ビルドツール: Vite 7

### 対応プラットフォーム
- macOS 12.0 (Monterey) 以上
- Apple Silicon (arm64) ネイティブサポート

[Unreleased]: https://github.com/sk8metalme/honnyaku/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/sk8metalme/honnyaku/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/sk8metalme/honnyaku/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/sk8metalme/honnyaku/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/sk8metalme/honnyaku/releases/tag/v0.1.0
