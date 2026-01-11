# Requirements Document

## Project Description (Input)
翻訳後テキストの要約機能と、元の文章への返信作成機能を追加する。翻訳完了後のポップアップにボタンを追加し、翻訳先言語で要約・返信を生成する。

## Introduction
本機能は、既存の翻訳ポップアップに要約・返信作成機能を追加し、ユーザーの翻訳後の作業を効率化します。翻訳完了後、翻訳結果を直接要約したり、元の文章への返信を生成したりできることで、コピー&ペーストの手間を削減し、ワンストップで複数の文章処理を実行できます。

## Requirements

### Requirement 1: 要約ボタン表示
**Objective:** ユーザーとして、翻訳後すぐに要約を生成できるように、翻訳ポップアップに要約ボタンを表示したい

#### Acceptance Criteria
1. When 翻訳が完了し状態が`completed`になる, the 翻訳ポップアップ shall 「要約」ボタンを翻訳結果表示エリアの下に表示する
2. The 要約ボタン shall 紫色（`bg-purple-500`）で視覚的に識別可能な色を使用する
3. The 要約ボタン shall 翻訳結果が存在しない場合は非表示にする
4. While 要約処理が実行中である, the 要約ボタン shall 非活性化され、スピナーと「要約中...」テキストを表示する

### Requirement 2: 返信作成ボタン表示
**Objective:** ユーザーとして、翻訳後すぐに返信を作成できるように、翻訳ポップアップに返信作成ボタンを表示したい

#### Acceptance Criteria
1. When 翻訳が完了し状態が`completed`になる, the 翻訳ポップアップ shall 「返信作成」ボタンを翻訳結果表示エリアの下に表示する
2. The 返信作成ボタン shall 緑色（`bg-green-500`）で視覚的に識別可能な色を使用する
3. The 返信作成ボタン shall 要約ボタンの右隣に配置される
4. The 返信作成ボタン shall 翻訳結果が存在しない場合は非表示にする
5. While 返信生成処理が実行中である, the 返信作成ボタン shall 非活性化され、スピナーと「返信作成中...」テキストを表示する

### Requirement 3: 要約機能の実行
**Objective:** ユーザーとして、翻訳後テキストを簡潔に要約し、内容を素早く理解したい

#### Acceptance Criteria
1. When ユーザーが要約ボタンをクリックする, the Honnyakuアプリ shall 翻訳後テキストをOllamaに送信して要約を生成する
2. The Honnyakuアプリ shall 要約は翻訳先言語（日→英なら英語、英→日なら日本語）で生成する
3. The Honnyakuアプリ shall 要約生成中は`actionState`を`summarizing`に設定する
4. When 要約生成が完了する, the Honnyakuアプリ shall 要約結果を翻訳ポップアップ内に紫色背景（`bg-purple-50`）のエリアで表示する
5. The Honnyakuアプリ shall 要約結果にコピーボタンを配置し、ワンクリックでクリップボードにコピー可能にする
6. If Ollamaが起動していない または 要約生成に失敗する, then the Honnyakuアプリ shall エラーメッセージを表示し、`actionState`を`idle`に戻す

### Requirement 4: 返信作成機能の実行
**Objective:** ユーザーとして、元の文章に対するビジネス向けの丁寧な返信を自動生成し、返信作業を効率化したい

#### Acceptance Criteria
1. When ユーザーが返信作成ボタンをクリックする, the Honnyakuアプリ shall 翻訳後テキストをOllamaに送信して返信を生成する
2. The Honnyakuアプリ shall 返信は翻訳先言語（日→英なら英語、英→日なら日本語）で生成する
3. The Honnyakuアプリ shall 返信のトーンは常にビジネス向けの丁寧な文体とする
4. The Honnyakuアプリ shall 返信生成中は`actionState`を`generating-reply`に設定する
5. When 返信生成が完了する, the Honnyakuアプリ shall 返信結果を翻訳ポップアップ内に緑色背景（`bg-green-50`）のエリアで表示する
6. The Honnyakuアプリ shall 返信結果にコピーボタンを配置し、ワンクリックでクリップボードにコピー可能にする
7. If Ollamaが起動していない または 返信生成に失敗する, then the Honnyakuアプリ shall エラーメッセージを表示し、`actionState`を`idle`に戻す

### Requirement 5: バックエンドAPIの追加
**Objective:** フロントエンドから要約・返信生成を実行できるように、Tauri IPCコマンドを提供したい

#### Acceptance Criteria
1. The Honnyakuバックエンド shall `summarize`コマンドを提供し、テキストと言語を受け取り要約結果を返す
2. The Honnyakuバックエンド shall `generate_reply`コマンドを提供し、テキストと言語を受け取り返信結果を返す
3. The `summarize`コマンド shall 要約に特化したプロンプトをOllama APIに送信する
4. The `generate_reply`コマンド shall ビジネス向け丁寧な返信生成に特化したプロンプトをOllama APIに送信する
5. The 要約・返信コマンド shall 既存の`translate`コマンドと同じエラーハンドリング（接続失敗、タイムアウト、APIエラー）を実装する
6. The 要約・返信コマンド shall 処理時間（ミリ秒）を結果に含める

### Requirement 6: 状態管理とリセット
**Objective:** ユーザーが翻訳ポップアップを閉じた際、要約・返信の状態も正しくリセットされるようにしたい

#### Acceptance Criteria
1. When ユーザーがEscキーを押す または 閉じるボタンをクリックする, the Honnyakuアプリ shall 翻訳状態に加えて要約・返信の状態（`summaryText`, `replyText`, `actionError`, `actionState`）もリセットする
2. The Honnyakuアプリ shall リセット後は`actionState`を`idle`に設定する
3. The Honnyakuアプリ shall ポップアップを閉じた後、再度翻訳を実行した際は前回の要約・返信結果が表示されないようにする

### Requirement 7: 同時実行制御
**Objective:** ユーザーが要約と返信を同時に実行できないようにし、処理の競合を防止したい

#### Acceptance Criteria
1. While `actionState`が`summarizing` または `generating-reply`である, the Honnyakuアプリ shall 要約ボタンと返信作成ボタンの両方を非活性化する
2. The Honnyakuアプリ shall 要約処理中に返信ボタンがクリックされても処理を開始しない
3. The Honnyakuアプリ shall 返信生成処理中に要約ボタンがクリックされても処理を開始しない

### Requirement 8: UIレイアウトと視認性
**Objective:** ユーザーが要約・返信結果を翻訳結果と区別しやすいように、明確なUIデザインを提供したい

#### Acceptance Criteria
1. The 要約結果表示エリア shall 紫色背景（`bg-purple-50 dark:bg-purple-900/30`）で表示される
2. The 返信結果表示エリア shall 緑色背景（`bg-green-50 dark:bg-green-900/30`）で表示される
3. The 要約・返信結果表示エリア shall 翻訳結果の下に配置され、それぞれ最大高さ（`max-h-48`, `max-h-32`）を持ちスクロール可能とする
4. The 要約・返信結果 shall ラベル（「要約」「返信案」）とコピーボタンを含むヘッダーを持つ
5. The ポップアップ全体 shall 要約・返信結果が追加されても画面外にはみ出さない最大幅（`max-w-md`）を維持する
