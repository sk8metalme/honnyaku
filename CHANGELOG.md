# Changelog

このプロジェクトのすべての注目すべき変更は、このファイルに記録されます。

フォーマットは [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいており、
このプロジェクトは [Semantic Versioning](https://semver.org/lang/ja/) に準拠しています。

## [Unreleased]

## [0.5.1] - 2025-12-11

### 重要な変更
- **Intel Macサポートの一時停止**
  - GitHub Actionsの課金制限により、Intel Mac (x86_64)ビルドを一時停止
  - v0.5.1以降はApple Silicon (M1/M2/M3/M4)のみをサポート
  - Intel Macユーザーは引き続きv0.5.0をご利用ください

### 修正
- **CI/CD**
  - GitHub Actions macOS-13ランナーイメージの廃止に対応
  - 無料ランナー(macos-latest)のみを使用してビルドを実行
  - リリースビルドの安定性を向上

### 技術的詳細
- macOS-13ランナーは2025年12月4日に完全廃止予定
- 有料ランナー(macos-14-large)は課金制限のため使用停止
- Apple Siliconのみのビルドに変更

## [0.5.0] - 2025-12-11

### 追加
- **翻訳品質向上機能**
  - Claude CLI翻訳のシステムプロンプトを最適化し、翻訳品質スコアを50/100から77.1/100（平均）に向上
  - 3セクション構成のプロンプト実装（役割定義・翻訳ルール・品質ガイドライン）
  - 技術文書特化型翻訳ルールを導入（プログラミング用語保持、コードスニペット非翻訳化、APIエンドポイント保持）
  - 10種類の基本技術用語の専門用語辞書を統合（API、framework、library、module、function、variable、interface、class、object、array）
  - プログラミング言語名・クラウドサービス名の自動保持（React、TypeScript、Rust、GitHub、Docker、Kubernetes等）
  - コンテキスト保持機能の強化（代名詞・接続詞の適切な翻訳、文間の論理的つながり保持）
  - 自然な表現変換ガイドライン（日本語では「です・ます調」優先、英語では能動態/受動態の適切な使い分け）
  - プロンプトキャッシング最適化（2000トークン以内に制限）

- **自動品質評価システム**
  - 30サンプルの統合テストを実装
  - Claude CLIによる自動品質評価機能
  - JSON形式の詳細レポート生成
  - カテゴリ別品質スコア測定
    - API Documentation: 91.4/100
    - Technical Blog: 84.6/100
    - Code Comment: 76.6/100
    - Error Message: 34.2/100（評価エラーあり - [Issue #10](https://github.com/sk8metalme/honnyaku/issues/10)で追跡中）

### 技術的詳細
- システムプロンプトは`build_system_prompt`関数で動的に生成
- 翻訳方向（英→日、日→英）に応じて異なるプロンプトを自動選択
- 既存のClaude CLI翻訳機能やOllama翻訳機能との完全な互換性を維持
- テストカバレッジ:
  - 単体テスト: 70 passed
  - フロントエンド統合テスト: 81 passed
  - 品質評価テスト: 30サンプル
  - 総合カバレッジ: 95%以上を維持

### 下位互換性
- 既存のAPIやインターフェースに変更なし
- ユーザー設定や翻訳フローに影響なし
- アプリケーション更新時に自動的に最適化されたプロンプトを使用

### 既知の問題
- エラーメッセージカテゴリの評価エラー（[Issue #10](https://github.com/sk8metalme/honnyaku/issues/10)）
- 一部サンプルでの評価タイムアウト

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

[Unreleased]: https://github.com/sk8metalme/honnyaku/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/sk8metalme/honnyaku/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/sk8metalme/honnyaku/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/sk8metalme/honnyaku/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/sk8metalme/honnyaku/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/sk8metalme/honnyaku/releases/tag/v0.1.0
