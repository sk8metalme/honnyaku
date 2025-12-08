# タスク分割: claude-cli-translation

## 実装タスク

### Phase 2: TDD実装（Implementation）

**総タスク数**: 3メジャータスク、9サブタスク
**推定工数**: 3-7日（gap-analysis.md見積もりに基づく）

---

## 1. バックエンド実装

Claude CLI実行とRust側の翻訳エンジンを実装します。

### - [x] 1.1 (P) Claude CLI実行サービスの実装

Claude CLIをRustから非同期実行し、JSON出力をパースして翻訳結果を取得する機能を実装します。

- `tokio::process::Command`を使用した非同期プロセス実行
- `--output-format json`を指定したコマンド構成（`--system-prompt`含む）
- JSON出力のパース処理（`serde_json`使用）、`output`フィールドの抽出
- 30秒タイムアウトの実装（`tokio::time::timeout`）
- Exit code検証とstderrエラーハンドリング
- `kill_on_drop(true)`によるプロセスリソース管理
- 単体テスト: 正常系（翻訳成功）、異常系（タイムアウト、Exit code != 0、JSONパースエラー）
- _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 3.2, 3.3, 3.4, 3.5, 3.6, 6.1, 6.2, 6.3, 6.4_
- _Contracts: claude_cli.rs Service Interface_

### - [x] 1.2 Tauri IPCコマンドの実装

フロントエンドからClaude CLI翻訳を呼び出すためのTauriコマンドを実装します。

- `translate_with_claude_cli` Tauriコマンド定義（`#[tauri::command]`）
- `AppSettings`から`claude_cli_path`を取得し、Claude CLI実行サービスに渡す
- `TranslationResult`を返却（既存Ollama翻訳と同じ型）
- エラーレスポンスの変換（`TranslationError` → Tauri IPC エラー）
- `lib.rs`の`invoke_handler`に`translate_with_claude_cli`を登録
- 単体テスト: 正常系（設定パス使用、デフォルトパス使用）、異常系（パス不正、CLI実行エラー）
- _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_
- _Contracts: translate_with_claude_cli API Contract_

### - [x] 1.3 (P) 設定モデルの拡張

Provider選択とClaude CLIパス設定を`AppSettings`に追加します。

- `AppSettings`構造体に`provider`フィールド追加（`String`型、デフォルト値`"ollama"`）
- `AppSettings`構造体に`claude_cli_path`フィールド追加（`Option<String>`型）
- `#[serde(default)]`による後方互換性の確保
- 既存の設定ファイルからのデシリアライズ検証（デフォルト値が正しく適用されることを確認）
- 単体テスト: デフォルト値検証、シリアライズ/デシリアライズ、既存設定との互換性
- _Requirements: 2.2, 2.3, 5.2, 5.3, 5.5, 8.4, 8.5_
- _Contracts: AppSettings State Management_

---

## 2. フロントエンド実装

TypeScript側のクライアントとUI統合を実装します。

### - [x] 2.1 (P) Claude CLIクライアントの実装

Tauri IPCを呼び出してClaude CLI翻訳を実行するTypeScriptクライアントを実装します。

- `translateWithClaudeCLI`関数の実装（`invoke('translate_with_claude_cli', ...)`）
- 引数: `text`, `sourceLang`, `targetLang`
- 戻り値: `Promise<TranslationResult>`
- エラーハンドリング（Tauri IPCエラーをキャッチし、`Error`をthrow）
- 型定義（`Language`, `TranslationResult`）の使用
- 単体テスト（モック使用）: 正常系（翻訳成功）、異常系（IPC通信エラー）
- _Requirements: 4.1, 4.2, 4.6_
- _Contracts: claude-cli.ts Service Interface_

### - [x] 2.2 翻訳フックのProvider分岐ロジック追加

`useTranslation` hookにProvider選択機能を追加し、Ollama/Claude CLI翻訳を切り替えます。

- `settings.provider`の確認ロジック追加
- `provider === 'claude-cli'`の場合は`translateWithClaudeCLI`を呼び出し
- `provider === 'ollama'`または未設定の場合は既存Ollama翻訳ロジック実行（変更なし）
- エラーハンドリング: Claude CLI翻訳失敗時は`error`状態を設定
- 既存の`isLoading`、`translatedText`、`error`状態管理を維持
- 既存テストの実行とパス確認（Ollama翻訳フローが変更されていないことを確認）
- 新規テスト: Provider分岐ロジック（Ollama選択時、Claude CLI選択時）
- _Requirements: 2.1, 2.3, 4.3, 4.4, 4.5, 8.1, 8.2_
- _Contracts: useTranslation State Management_

### - [x] 2.3 (P) 設定UIのProvider選択機能追加

`SettingsPanel`にProvider選択UIとClaude CLIパス入力フィールドを追加します。

- Provider選択ラジオボタン（Ollama、Claude CLI）の実装
- 各Provider選択肢の説明テキスト表示（プライバシー、速度、コスト特性）
- Claude CLI選択時のみClaude CLIパス入力フィールドを表示（条件付きレンダリング）
- パス入力フィールド: プレースホルダー（`/opt/homebrew/bin/claude`）
- パス検証: Bashコマンド`which claude`またはカスタムパスで実行可能性を確認
- Claude CLI未検出時の警告メッセージ表示
- 設定保存時に`provider`と`claude_cli_path`を`save_settings` IPCで送信
- UIテスト: Provider選択、パス入力、警告表示
- _Requirements: 2.1, 2.4, 2.5, 5.1, 5.4_
- _Contracts: SettingsPanel UI_

---

## 3. 統合とテスト

システム全体を統合し、エラーハンドリングとE2Eテストを実施します。

### - [x] 3.1 システム統合と接続テスト

フロントエンド・バックエンド・Claude CLIの統合動作を検証します。

- グローバルショートカット（Cmd+Shift+T）→ クリップボード取得 → Claude CLI翻訳 → ポップアップ表示のフロー確認
- Provider切り替え動作確認（Ollama ↔ Claude CLI）
- Claude CLIパス設定の永続化と復元確認
- アプリ再起動後の設定復元確認
- 既存Ollama翻訳フローが影響を受けていないことを確認（回帰テスト）
- 統合テスト: End-to-End翻訳フロー（Ollama、Claude CLI両方）
- _Requirements: 2.2, 2.3, 4.3, 8.1, 8.2, 8.3, 8.5_

### - [x] 3.2 エラーハンドリングの検証

Claude CLI翻訳の各種エラーケースを検証し、適切なメッセージが表示されることを確認します。

- Claude CLI未インストール時のエラーメッセージ検証
- タイムアウト（30秒）時のエラーメッセージ検証
- CLI実行エラー（Exit code != 0）時のエラーメッセージ検証
- パス検証エラー時の警告メッセージ検証
- エラー内容がコンソールログに出力されることを確認
- ユーザー向けエラーメッセージが`TranslationPopup`に表示されることを確認
- エラーハンドリングテスト: 各エラーケースでの動作確認
- _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

### - [x] 3.3 E2Eテストと性能検証

Claude CLI翻訳の性能とユーザー体験を検証します。

- 翻訳レスポンス時間の測定（目標: 平均10秒以内、通常テキスト~100語）
- タイムアウト処理の正確性検証（30秒で確実に終了）
- 子プロセスのリソースリーク防止検証（プロセス終了後のリソース状態確認）
- プロンプトキャッシング効果の測定（初回 vs 2回目以降のトークン消費、応答時間）
- E2Eテスト: ユーザーシナリオベースのテスト（複数回連続翻訳、エラーリカバリ等）
- _Requirements: 6.1, 6.2_
- _Note: プロンプトキャッシング効果は参考値として測定、MVP後の最適化で詳細分析_

---

## 要件カバレッジ

| 要件 | 対応タスク |
|------|-----------|
| 1.1, 1.2, 1.3, 1.4, 1.5, 1.6 | 1.1 |
| 2.1, 2.4, 2.5 | 2.3 |
| 2.2, 2.3 | 1.3, 3.1 |
| 3.1, 3.2, 3.3, 3.4, 3.5 | 1.2 |
| 3.2, 3.3, 3.4, 3.5, 3.6 | 1.1 |
| 4.1, 4.2, 4.6 | 2.1 |
| 4.3, 4.4, 4.5 | 2.2 |
| 5.1, 5.4 | 2.3 |
| 5.2, 5.3, 5.5 | 1.3 |
| 6.1, 6.2, 6.3, 6.4 | 1.1, 3.3 |
| 7.1, 7.2, 7.3, 7.4, 7.5 | 3.2 |
| 8.1, 8.2, 8.3, 8.4, 8.5 | 1.3, 2.2, 3.1 |

**全8要件、36個の受け入れ基準をカバー**

---

## 実装順序の推奨

**並列実行可能タスク（(P)マーク）**:
- 1.1 (Backend: Claude CLI実行) と 2.1 (Frontend: Claude CLIクライアント) は並列実行可能
- 1.3 (Backend: AppSettings拡張) と 2.3 (Frontend: SettingsPanel拡張) は並列実行可能

**依存関係**:
1. **Phase 1**: 1.1, 1.3, 2.1, 2.3（並列実行可能）
2. **Phase 2**: 1.2（1.1, 1.3完了後）、2.2（2.1完了後）
3. **Phase 3**: 3.1（1.2, 2.2完了後）→ 3.2 → 3.3

**推奨実装順序**:
```
Week 1:
  Day 1-2: 1.1 (P) + 2.1 (P) 並列実装
  Day 3: 1.3 (P) + 2.3 (P) 並列実装

Week 2:
  Day 4: 1.2 実装
  Day 5: 2.2 実装

Week 3:
  Day 6: 3.1 統合テスト
  Day 7: 3.2 + 3.3 検証・最適化
```

---

**次のステップ**:
1. タスクを確認し、必要に応じて調整
2. 実装開始: `/kiro:spec-impl claude-cli-translation 1.1` または `/kiro:spec-impl claude-cli-translation 1.1,2.1` (並列実行)
