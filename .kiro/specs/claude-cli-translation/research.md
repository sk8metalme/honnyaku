# Research & Design Decisions

## Summary

- **Feature**: `claude-cli-translation`
- **Discovery Scope**: Extension（既存システムへの拡張）
- **Key Findings**:
  - `claude -p`はプロンプトキャッシング専用フラグではなく、**Print Mode（非対話型実行モード）**のフラグ
  - Claude Codeは公式のCLIツールで、`claude -p "query"`形式でヘッドレス実行が可能
  - プロンプトキャッシングはClaude API機能であり、カスタムシステムプロンプトで最適化可能

## Research Log

### Claude CLI (`claude -p`) の仕様調査

**Context**: 要件では「`claude -p`のプロンプトキャッシング方式」と記載されていたが、正確な仕様を確認する必要があった

**Sources Consulted**:
- [Claude Code CLI Reference](https://code.claude.com/docs/en/cli-usage)
- [Claude Code Overview](https://code.claude.com/docs/en/overview)
- [Claude Code Cheatsheet (Shipyard)](https://shipyard.build/blog/claude-code-cheat-sheet/)
- [Anthropic Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)

**Findings**:

1. **`-p`フラグの正体**:
   - **Print Mode（プリントモード）**: 非対話型でクエリを1回実行して終了
   - スクリプトや自動化に最適化されたヘッドレスモード
   - `claude -p "query"`形式で使用

2. **基本的な使用方法**:
   ```bash
   # 直接クエリ実行
   claude -p "explain this project"

   # パイプ入力対応
   echo "Hello, how are you?" | claude -p "Translate to French"
   cat file.txt | claude -p "Summarize this text"
   ```

3. **翻訳タスクでの活用**:
   - テキストをパイプで渡し、翻訳指示を与えることで翻訳結果を取得
   - 非対話型なのでスクリプトやバックグラウンド処理に適している
   - 出力フォーマット指定可能（`--output-format json`）

4. **重要なオプション**:
   - `--output-format`: `text`（デフォルト）、`json`、`stream-json`
   - `--system-prompt`: システムプロンプトを置き換え
   - `--append-system-prompt`: デフォルトプロンプトに追加
   - `--max-turns`: エージェント反復回数の制限
   - `--fallback-model`: オーバーロード時の代替モデル

**Implications**:
- 翻訳タスクに適した非対話型実行が可能
- パイプ入力対応により、既存のクリップボードテキストをシームレスに処理できる
- システムプロンプトのカスタマイズにより、翻訳品質とキャッシング効率を最適化可能

### プロンプトキャッシングの実装方法

**Context**: Claude APIのプロンプトキャッシング機能をClaude CLI経由でどう活用するか

**Sources Consulted**:
- [Prompt caching - Claude Docs](https://platform.claude.com/docs/en/build-with-claude/prompt-caching)
- [Prompt caching with Claude (Anthropic Blog)](https://www.claude.com/blog/prompt-caching)
- [Unlocking Efficiency: A Practical Guide to Claude Prompt Caching (Medium)](https://medium.com/@mcraddock/unlocking-efficiency-a-practical-guide-to-claude-prompt-caching-3185805c0eef)

**Findings**:

1. **プロンプトキャッシングの仕組み**:
   - Claude APIレベルの機能で、プロンプトのプレフィックス部分をキャッシュ
   - キャッシュの有効期限: 5分（使用時に更新）
   - コスト削減: キャッシュヒット時は入力トークンの10%のコスト、書き込み時は25%増

2. **キャッシング最適化の方法**:
   - **システムプロンプト**（固定部分）: 翻訳ルール、言語仕様、スタイルガイド
   - **可変部分**（ユーザー入力）: 翻訳対象テキスト
   - `--system-prompt`または`--append-system-prompt`でシステムプロンプトを設定

3. **実装戦略**:
   ```bash
   # 固定の翻訳システムプロンプトを使用
   claude -p --system-prompt "You are a professional translator. Translate the following text to [target language] while preserving meaning, tone, and context." "Text to translate"
   ```

**Implications**:
- Claude Codeがバックエンドでプロンプトキャッシングを自動的に活用する可能性が高い
- カスタムシステムプロンプトを使用することで、キャッシング効率を明示的に向上できる
- 設計では「システムプロンプト管理」を考慮する必要がある

### 既存の翻訳実装パターン分析

**Context**: 既存のOllama翻訳実装を理解し、Claude CLI統合の拡張ポイントを特定

**Sources Consulted**:
- ギャップ分析ドキュメント（gap-analysis.md）
- 既存コードベース調査結果

**Findings**:

1. **既存アーキテクチャ**:
   - **フロントエンド**: `useTranslation` hook → Tauri IPC `translate`コマンド
   - **バックエンド**: `lib.rs`の`translate`コマンド → `translation::translate_with_ollama`
   - **設定管理**: `AppSettings`（tauri-plugin-store）- Ollama専用設定のみ

2. **拡張ポイント**:
   - **新規Tauriコマンド**: `translate_with_claude_cli`をlib.rsに追加
   - **新規サービス**: `src-tauri/src/services/claude_cli.rs`で`std::process::Command`実行
   - **フロントエンドクライアント**: `src/lib/claude-cli.ts`でTauri IPC呼び出し
   - **設定拡張**: `AppSettings`に`provider`, `claude_cli_path`フィールド追加

3. **統合パターン**:
   - `useTranslation` hook内でprovider分岐を追加
   - 既存のOllama/未実装のClaude APIと同じインターフェースを維持
   - `TranslationResult`型を共通化

**Implications**:
- 既存パターンを踏襲することで実装コストを削減
- provider抽象化により、将来的なLLM追加が容易
- 既存テストへの影響を最小化

### Rust `std::process::Command`でのCLI実行

**Context**: RustからClaude CLIを安全に実行する方法

**Sources Consulted**:
- Rust公式ドキュメント（std::process::Command）
- 既存コードベースのパターン（非同期実行）

**Findings**:

1. **非同期実行パターン**:
   ```rust
   use tokio::process::Command;

   let output = Command::new("claude")
       .arg("-p")
       .arg("translate query")
       .stdin(Stdio::piped())
       .stdout(Stdio::piped())
       .stderr(Stdio::piped())
       .kill_on_drop(true)
       .spawn()?;
   ```

2. **タイムアウト処理**:
   - `tokio::time::timeout`で30秒タイムアウトを設定
   - タイムアウト時は子プロセスを終了（`kill_on_drop`）

3. **出力パース**:
   - 標準出力（stdout）から翻訳結果を取得
   - 標準エラー（stderr）でエラーメッセージを処理
   - Exit code 0以外はエラーとして扱う

4. **リソース管理**:
   - `kill_on_drop(true)`で子プロセスの自動クリーンアップ
   - ストリーム処理でメモリ効率を維持

**Implications**:
- 既存の非同期パターン（`tokio`）と整合性が取れる
- タイムアウトとリソース管理が標準的な方法で実現可能
- エラーハンドリングはRustの`Result`型で型安全に実装

## Architecture Pattern Evaluation

### Provider Abstraction Pattern

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Strategy Pattern | provider選択を`useTranslation` hook内で分岐 | シンプル、既存パターンに沿う | hook内のロジックが複雑化 | 推奨: 小規模な拡張に適している |
| Service Layer Abstraction | `TranslationService` traitで抽象化 | 高い拡張性、テスト容易性 | 過度な抽象化、実装コスト増 | 将来的にproviderが5+になる場合に検討 |
| Hybrid (Recommended) | フロントエンドはStrategy、バックエンドは個別サービス | バランスが良い、段階的な実装が可能 | 一貫性の維持が必要 | gap-analysisで推奨されたアプローチ |

**Selected**: Hybrid Approach（推奨）

## Design Decisions

### Decision: Claude CLI実行方式

**Context**: Rustバックエンドから`claude -p`コマンドを実行する方法

**Alternatives Considered**:
1. **Option A**: `std::process::Command`で直接実行
   - シンプルで標準的
   - 同期実行のため、パフォーマンス懸念
2. **Option B**: `tokio::process::Command`で非同期実行
   - 既存パターンと一貫性
   - 非同期処理でパフォーマンス向上
3. **Option C**: Claude APIを直接呼び出し
   - CLI不要、依存関係削減
   - APIキー管理が必要、ユーザー要件と異なる

**Selected Approach**: Option B（`tokio::process::Command`）

**Rationale**:
- 既存のOllama実装が非同期パターンを使用している
- Tauriのランタイムが`tokio`を使用しているため、自然な統合
- ユーザー要件である「`claude -p`方式」に合致

**Trade-offs**:
- **Benefits**: 非同期処理による応答性向上、既存パターンとの一貫性
- **Compromises**: CLI依存（インストール必要）、子プロセス管理のオーバーヘッド

**Follow-up**: 実装時にタイムアウト処理とエラーハンドリングを十分にテスト

### Decision: プロンプトキャッシング実装

**Context**: プロンプトキャッシングを効果的に活用する方法

**Alternatives Considered**:
1. **Option A**: デフォルトのClaude Codeプロンプトに依存
   - 実装不要
   - キャッシング効率が不明確
2. **Option B**: カスタムシステムプロンプトで翻訳ルールを固定化
   - キャッシング効率向上
   - システムプロンプト管理が必要
3. **Option C**: 動的にシステムプロンプトを構築
   - 柔軟性が高い
   - 複雑性が増加

**Selected Approach**: Option B（カスタムシステムプロンプト）

**Rationale**:
- システムプロンプト（翻訳ルール）は固定で、キャッシュ可能
- 可変部分（翻訳対象テキスト）のみをユーザー入力として渡す
- プロンプトキャッシングの恩恵を最大化

**Trade-offs**:
- **Benefits**: コスト削減、応答速度向上
- **Compromises**: システムプロンプトのメンテナンスが必要

**Follow-up**: 翻訳品質を検証し、システムプロンプトを最適化

### Decision: Provider選択UIの配置

**Context**: ユーザーが翻訳方式を選択するUIをどこに配置するか

**Alternatives Considered**:
1. **Option A**: `SettingsPanel`に既存のOllama設定と並べて配置
   - シンプル、既存パターンに沿う
   - SettingsPanelが複雑化
2. **Option B**: 新しい`ProviderSelector`コンポーネントを作成
   - 責任分離が明確
   - ファイル数増加
3. **Option C**: ポップアップ内にprovider選択を埋め込み
   - 翻訳実行時に選択
   - UXが煩雑になる可能性

**Selected Approach**: Option A（SettingsPanelに配置）

**Rationale**:
- 設定は一度設定すれば保存されるため、毎回選択する必要はない
- 既存のOllama設定と同じ場所に配置することで、ユーザーの学習コストを削減
- provider選択はラジオボタンで3つの選択肢を提供

**Trade-offs**:
- **Benefits**: シンプル、既存UIパターンを踏襲
- **Compromises**: SettingsPanelのUIが若干複雑化（許容範囲内）

**Follow-up**: UIモックアップで視覚的に確認

## Risks & Mitigations

- **Risk 1**: Claude CLIがインストールされていない環境でエラー
  - **Mitigation**: 設定画面でClaude CLIの検出とインストール案内を表示、カスタムパス設定を提供
- **Risk 2**: `claude -p`コマンドのタイムアウト（ネットワーク遅延等）
  - **Mitigation**: 30秒タイムアウトを設定し、わかりやすいエラーメッセージを表示
- **Risk 3**: プロンプトキャッシングの効果が不明確
  - **Mitigation**: 実装後にベンチマークを実施し、コスト削減効果を測定
- **Risk 4**: 既存Ollama翻訳への影響
  - **Mitigation**: 既存テストを全て実行し、回帰がないことを確認。provider選択のデフォルト値をOllamaに設定
- **Risk 5**: Claude Code出力フォーマットの変動
  - **Mitigation**: `--output-format json`を使用して構造化された出力を取得、パースロジックを堅牢に実装

## References

### Claude Code公式ドキュメント
- [Claude Code CLI Reference](https://code.claude.com/docs/en/cli-usage) — Print modeの詳細仕様
- [Claude Code Overview](https://code.claude.com/docs/en/overview) — インストールと基本的な使い方
- [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices) — 公式ベストプラクティス

### プロンプトキャッシング
- [Prompt caching - Claude Docs](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) — 公式プロンプトキャッシングドキュメント
- [Unlocking Efficiency: Claude Prompt Caching Guide](https://medium.com/@mcraddock/unlocking-efficiency-a-practical-guide-to-claude-prompt-caching-3185805c0eef) — 実践ガイド

### コミュニティリソース
- [Claude Code Cheatsheet (Shipyard)](https://shipyard.build/blog/claude-code-cheat-sheet/) — CLI使用例とベストプラクティス
- [GitHub - anthropics/claude-code](https://github.com/anthropics/claude-code) — 公式GitHubリポジトリ
