# Requirements Document

## Introduction

本ドキュメントは、Nani風AI翻訳デスクトップアプリの機能要件を定義します。このアプリケーションは、macOSユーザーがグローバルショートカットキーを使用して、任意のアプリケーションで選択したテキストを即座に翻訳できるデスクトップツールです。

**対象ユーザー**: macOSを使用する個人ユーザー（開発者、翻訳者、一般ユーザー）
**技術スタック**: Tauri v2 + React + TypeScript、Ollama（ローカルLLM）/ Claude API（リモートLLM）
**対応言語ペア（MVP）**: 日本語 ↔ 英語
**対応言語ペア（将来）**: 日本語 ↔ 韓国語、英語 ↔ 韓国語

## Requirements

### Requirement 1: グローバルショートカット機能

**Objective:** As a ユーザー, I want グローバルショートカットキーで翻訳を起動できること, so that どのアプリケーションを使用中でも素早く翻訳を実行できる

#### Acceptance Criteria

1. When ユーザーがデフォルトショートカット（Cmd+Shift+T）を押下した時, the 翻訳アプリ shall 選択テキストの取得処理を開始する
2. While 翻訳アプリがバックグラウンドで動作中, the 翻訳アプリ shall グローバルショートカットキーの入力を常に監視する
3. If ショートカットキーが他のアプリケーションと競合している場合, the 翻訳アプリ shall 競合を検出しユーザーに通知する
4. Where カスタムショートカットが設定されている場合, the 翻訳アプリ shall ユーザー定義のショートカットキーで翻訳を起動する
5. The 翻訳アプリ shall macOSのアクセシビリティ権限が許可されているか確認する

### Requirement 2: 選択テキスト取得機能

**Objective:** As a ユーザー, I want 選択中のテキストを自動的に取得してほしい, so that 手動でコピー&ペーストする手間を省ける

#### Acceptance Criteria

1. When グローバルショートカットが押下された時, the 翻訳アプリ shall システムにCmd+Cを送信して選択テキストをクリップボードにコピーする
2. When クリップボードにテキストがコピーされた時, the 翻訳アプリ shall クリップボードからテキストを読み取る
3. If クリップボードが空または非テキストデータの場合, the 翻訳アプリ shall 「翻訳するテキストが選択されていません」とユーザーに通知する
4. The 翻訳アプリ shall 元のクリップボード内容を翻訳処理後に復元する

### Requirement 3: 言語自動検出機能

**Objective:** As a ユーザー, I want 翻訳元言語を自動検出してほしい, so that 手動で言語を選択する手間を省ける

#### Acceptance Criteria

1. When テキストが取得された時, the 言語検出モジュール shall 入力テキストの言語を日本語または英語から判定する
2. When 日本語テキストが検出された時, the 翻訳アプリ shall 英語への翻訳を実行する
3. When 英語テキストが検出された時, the 翻訳アプリ shall 日本語への翻訳を実行する
4. If 言語検出の信頼度が低い場合, the 翻訳アプリ shall デフォルトの翻訳方向（日→英）を使用する
5. The 言語検出モジュール shall 10文字未満の短いテキストでも言語を推測する

### Requirement 4: 翻訳処理機能

**Objective:** As a ユーザー, I want テキストを高品質に翻訳してほしい, so that 正確な翻訳結果を得られる

#### Acceptance Criteria

1. When 翻訳リクエストが発生した時, the 翻訳アプリ shall 設定されたLLMプロバイダー（OllamaまたはClaude API）に翻訳リクエストを送信する
2. When Ollamaが選択されている時, the 翻訳アプリ shall localhost:11434のOllama APIにリクエストを送信する
3. When Claude APIが選択されている時, the 翻訳アプリ shall api.anthropic.comにHTTPSでリクエストを送信する
4. If LLMからの応答がタイムアウトした場合（10秒）, the 翻訳アプリ shall タイムアウトエラーをユーザーに通知する
5. If LLM接続に失敗した場合, the 翻訳アプリ shall 接続エラーメッセージを表示し、設定の確認を促す
6. The 翻訳アプリ shall 翻訳結果のみを返し、LLMの説明文や前置きを除去する

### Requirement 5: ポップアップ表示機能

**Objective:** As a ユーザー, I want 翻訳結果を小さなポップアップで確認したい, so that 作業中の画面を大きく遮らない

#### Acceptance Criteria

1. When 翻訳が完了した時, the ポップアップウィンドウ shall マウスカーソル位置の近くに表示される
2. While ポップアップが表示中, the ポップアップ shall 常に他のウィンドウの前面に表示される（always on top）
3. When ユーザーがポップアップ外をクリックした時, the ポップアップ shall 自動的に閉じる
4. When Escキーが押された時, the ポップアップ shall 即座に閉じる
5. If 翻訳処理中の場合, the ポップアップ shall ローディングインジケーターを表示する
6. The ポップアップ shall タイトルバーなしの角丸デザインで表示される
7. The ポップアップ shall macOSのDockに表示されない

### Requirement 6: クリップボードコピー機能

**Objective:** As a ユーザー, I want 翻訳結果をワンクリックでコピーしたい, so that 他のアプリケーションにすぐ貼り付けられる

#### Acceptance Criteria

1. When ユーザーがコピーボタンをクリックした時, the 翻訳アプリ shall 翻訳結果をクリップボードにコピーする
2. When コピーが成功した時, the 翻訳アプリ shall コピー完了のビジュアルフィードバック（チェックマーク等）を表示する
3. When ユーザーがCmd+Cを押下した時（ポップアップフォーカス中）, the 翻訳アプリ shall 翻訳結果をクリップボードにコピーする
4. The 翻訳アプリ shall コピー後もポップアップを自動で閉じない（ユーザー操作を待つ）

### Requirement 7: LLMプロバイダー切り替え機能

**Objective:** As a ユーザー, I want OllamaとClaude APIを切り替えたい, so that 用途に応じて最適なLLMを選択できる

#### Acceptance Criteria

1. When ユーザーが設定画面でLLMプロバイダーを変更した時, the 翻訳アプリ shall 次回の翻訳から選択されたプロバイダーを使用する
2. Where Ollamaが選択されている場合, the 翻訳アプリ shall Ollamaの接続状態を確認し、利用可能かどうかを表示する
3. Where Claude APIが選択されている場合, the 翻訳アプリ shall APIキーが設定されているか確認する
4. If Ollamaが起動していない場合, the 翻訳アプリ shall 「Ollamaが起動していません」と警告を表示する
5. The 翻訳アプリ shall デフォルトでOllama（ローカル）を使用する

### Requirement 8: 設定管理機能

**Objective:** As a ユーザー, I want アプリの設定をカスタマイズしたい, so that 自分の好みに合わせて使用できる

#### Acceptance Criteria

1. When ユーザーが設定を変更した時, the 翻訳アプリ shall 設定をローカルストレージに永続化する
2. When アプリが起動した時, the 翻訳アプリ shall 保存された設定を読み込む
3. The 設定画面 shall ショートカットキーのカスタマイズを提供する
4. The 設定画面 shall LLMプロバイダーの選択を提供する
5. The 設定画面 shall Claude APIキーの入力フィールドを提供する
6. The 設定画面 shall Ollamaで使用するモデル名の設定を提供する
7. If Claude APIキーが入力された場合, the 翻訳アプリ shall APIキーをmacOS Keychainに安全に保存する

### Requirement 9: 非機能要件

**Objective:** As a ユーザー, I want 高速でセキュア、かつ軽量なアプリを使用したい, so that ストレスなく安心して利用できる

#### Acceptance Criteria

1. The 翻訳アプリ shall ショートカット押下から翻訳結果表示まで3秒以内（Ollama使用時、ローカルネットワーク）に完了する
2. The 翻訳アプリ shall アイドル時のメモリ使用量を100MB以下に維持する
3. The 翻訳アプリ shall Claude APIとの通信にHTTPSを使用する
4. The 翻訳アプリ shall Claude APIキーをプレーンテキストでファイルに保存しない
5. While オフライン状態の場合, the 翻訳アプリ shall Ollama（ローカル）のみを利用可能とし、Claude APIエラーを適切に処理する
6. The 翻訳アプリ shall macOS 12.0（Monterey）以上で動作する
7. The 翻訳アプリ shall 初回起動時にアクセシビリティ権限のリクエストダイアログを表示する
8. The 翻訳アプリ shall アプリサイズを50MB以下に維持する
