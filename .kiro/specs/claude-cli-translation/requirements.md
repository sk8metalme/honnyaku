# Requirements Document

## Project Description (Input)
翻訳機能にClaude CLI (claude -p) のプロンプトキャッシング方式を追加する。既存のAPI呼び出しベースの翻訳機能はそのまま維持し、新たな翻訳方式としてclaude -pコマンドを使ったオプションを追加することで、ユーザーが用途に応じて選択できるようにする

## Introduction

Honnyaku翻訳アプリケーションに、新たな翻訳方式としてClaude CLI（`claude -p`）によるプロンプトキャッシング方式を追加します。既存のOllama（ローカルLLM）およびClaude API方式を維持しつつ、第3の選択肢としてClaude CLIを提供することで、ユーザーは用途・環境に応じて最適な翻訳方式を選択できるようになります。

プロンプトキャッシングにより、繰り返し翻訳時のコスト削減と応答速度向上が期待できます。

## Requirements

### Requirement 1: Claude CLI翻訳エンジンの実装

**Objective:** アプリケーション開発者として、`claude -p`コマンドを使った翻訳機能を実装し、ユーザーがプロンプトキャッシングの恩恵を受けられるようにしたい

#### Acceptance Criteria

1. When ユーザーがClaude CLI翻訳方式を選択している場合、the Honnyakuアプリ shall `claude -p`コマンドを実行して翻訳を行う
2. When 翻訳リクエストが発行される場合、the Honnyakuアプリ shall 翻訳プロンプトをClaude CLIに渡し、標準出力から翻訳結果を取得する
3. The Honnyakuアプリ shall Claude CLIの実行結果（stdout, stderr, exit code）を適切にハンドリングする
4. If Claude CLIコマンドが存在しない、またはパスが通っていない場合、then the Honnyakuアプリ shall ユーザーに分かりやすいエラーメッセージを表示する
5. If Claude CLIの実行がタイムアウト（30秒超過）した場合、then the Honnyakuアプリ shall プロセスを終了し、タイムアウトエラーを表示する
6. The Honnyakuアプリ shall 翻訳実行時のClaude CLIコマンドライン引数を適切に構成する（プロンプトキャッシングオプション含む）

### Requirement 2: 翻訳方式選択機能の拡張

**Objective:** ユーザーとして、既存のOllama/Claude APIに加えてClaude CLI方式を選択できるようにし、用途に応じて翻訳方式を切り替えたい

#### Acceptance Criteria

1. The Honnyakuアプリ shall 設定画面で3つの翻訳方式（Ollama、Claude API、Claude CLI）から選択可能なUIを提供する
2. When ユーザーが翻訳方式を変更して保存した場合、the Honnyakuアプリ shall 選択された方式を`tauri-plugin-store`に永続化する
3. When アプリケーション起動時、the Honnyakuアプリ shall 保存された翻訳方式設定を読み込み、デフォルト翻訳エンジンとして設定する
4. The Honnyakuアプリ shall 各翻訳方式の説明（プライバシー、速度、コストなどの特徴）を設定UIに表示する
5. When Claude CLI方式が選択されているがコマンドが利用不可の場合、the Honnyakuアプリ shall 設定画面で警告メッセージを表示する

### Requirement 3: バックエンド統合（Rust側実装）

**Objective:** バックエンド開発者として、Rustレイヤーで`claude -p`コマンドを安全に実行し、フロントエンドにIPC経由で結果を返せるようにしたい

#### Acceptance Criteria

1. The Honnyakuアプリ shall `src-tauri/src/commands/`に新しいTauriコマンド`translate_with_claude_cli`を実装する
2. When `translate_with_claude_cli`が呼び出された場合、the Honnyakuアプリ shall Rustの`std::process::Command`を使って`claude -p`を非同期実行する
3. The Honnyakuアプリ shall 翻訳対象テキストと翻訳方向（source/target language）をコマンド引数として受け取る
4. The Honnyakuアプリ shall Claude CLIの標準出力をパースし、翻訳結果のみを抽出してフロントエンドに返す
5. If Claude CLIがエラーステータス（exit code != 0）で終了した場合、then the Honnyakuアプリ shall stderrの内容を含むエラーレスポンスを返す
6. The Honnyakuアプリ shall Claude CLI実行中の子プロセスを適切に管理し、リソースリークを防ぐ

### Requirement 4: フロントエンド統合（TypeScript側実装）

**Objective:** フロントエンド開発者として、既存の翻訳フローにClaude CLI方式を統合し、UIレイヤーからシームレスに呼び出せるようにしたい

#### Acceptance Criteria

1. The Honnyakuアプリ shall `src/lib/`に`claude-cli.ts`クライアントモジュールを作成する
2. When `claude-cli.ts`の`translate()`関数が呼び出された場合、the Honnyakuアプリ shall Tauri IPCコマンド`translate_with_claude_cli`を`invoke()`で実行する
3. The Honnyakuアプリ shall 既存の`useTranslation` hookでClaude CLI方式をサポートし、選択された翻訳方式に応じて適切なクライアント（ollama/claude/claude-cli）を呼び出す
4. When Claude CLI翻訳が実行中の場合、the Honnyakuアプリ shall ローディングインジケーターを表示する
5. If Claude CLI翻訳がエラーを返した場合、then the Honnyakuアプリ shall エラーメッセージをポップアップUIに表示する
6. The Honnyakuアプリ shall Claude CLI翻訳結果を既存のOllama/Claude API結果と同じフォーマットで表示する

### Requirement 5: Claude CLIパス設定機能

**Objective:** ユーザーとして、`claude`コマンドが標準パスにない場合でも、カスタムパスを設定してClaude CLI翻訳を利用できるようにしたい

#### Acceptance Criteria

1. The Honnyakuアプリ shall 設定画面でClaude CLIの実行パスを入力できるフィールドを提供する
2. When カスタムパスが設定されている場合、the Honnyakuアプリ shall システムPATH検索よりもカスタムパスを優先して使用する
3. When カスタムパスが未設定の場合、the Honnyakuアプリ shall システムPATH環境変数から`claude`コマンドを検索する
4. The Honnyakuアプリ shall 設定されたパスが有効な実行可能ファイルかを検証し、無効な場合は警告を表示する
5. The Honnyakuアプリ shall Claude CLIパス設定を`tauri-plugin-store`に保存し、アプリ再起動後も保持する

### Requirement 6: プロンプトキャッシング最適化

**Objective:** アプリケーション開発者として、プロンプトキャッシングを効果的に活用し、繰り返し翻訳時のコスト削減と応答速度向上を実現したい

#### Acceptance Criteria

1. The Honnyakuアプリ shall `claude -p`コマンド実行時に、翻訳システムプロンプト部分をキャッシュ可能な形式で構成する
2. When 同一セッション内で複数回翻訳を実行する場合、the Honnyakuアプリ shall システムプロンプトをキャッシュから再利用してトークン消費を削減する
3. The Honnyakuアプリ shall キャッシュ可能なプロンプト部分（システムプロンプト、翻訳ルール）と可変部分（翻訳対象テキスト）を明確に分離する
4. The Honnyakuアプリ shall Claude CLIのキャッシング機能を活用するために、適切なプロンプト構造（キャッシュブロック）を使用する

### Requirement 7: エラーハンドリングとユーザーフィードバック

**Objective:** ユーザーとして、Claude CLI翻訳でエラーが発生した際に、原因を理解し適切な対処ができるようにしたい

#### Acceptance Criteria

1. If Claude CLIコマンドが見つからない場合、then the Honnyakuアプリ shall 「Claude CLIがインストールされていません。インストール方法を確認してください」というメッセージを表示する
2. If Claude CLI実行中にネットワークエラーが発生した場合、then the Honnyakuアプリ shall 「Claude APIとの通信に失敗しました。ネットワーク接続を確認してください」というメッセージを表示する
3. If Claude CLIのレート制限に達した場合、then the Honnyakuアプリ shall 「APIレート制限に達しました。しばらく待ってから再試行してください」というメッセージを表示する
4. The Honnyakuアプリ shall エラー内容を開発者向けにコンソールログに出力する（デバッグ用）
5. When Claude CLI翻訳が失敗した場合、the Honnyakuアプリ shall 自動的に別の翻訳方式（既存のClaude API等）にフォールバックする機能を提供しない（ユーザーが手動で切り替える）

### Requirement 8: 既存機能との互換性維持

**Objective:** アプリケーション開発者として、Claude CLI機能追加が既存のOllama/Claude API翻訳機能に影響を与えないことを保証したい

#### Acceptance Criteria

1. When Ollama翻訳方式が選択されている場合、the Honnyakuアプリ shall 既存のOllama翻訳フローを変更なく実行する
2. When Claude API翻訳方式が選択されている場合、the Honnyakuアプリ shall 既存のClaude API翻訳フローを変更なく実行する
3. The Honnyakuアプリ shall Claude CLI機能追加後も、既存の自動テストが全てパスすることを保証する
4. The Honnyakuアプリ shall 各翻訳方式の設定（APIキー、エンドポイント等）を独立して管理し、相互に影響しないようにする
5. When アプリケーションアップデート時、the Honnyakuアプリ shall 既存ユーザーの翻訳方式設定（Ollama/Claude API）を保持し、デフォルトでClaude CLI方式に変更しない

