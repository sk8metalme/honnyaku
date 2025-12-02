# コントリビューションガイド

Honnyakuへのコントリビューションをありがとうございます！このドキュメントでは、プロジェクトへの貢献方法を説明します。

## 目次

- [行動規範](#行動規範)
- [開発環境のセットアップ](#開発環境のセットアップ)
- [開発ワークフロー](#開発ワークフロー)
- [コーディング規約](#コーディング規約)
- [テスト](#テスト)
- [プルリクエストの提出](#プルリクエストの提出)

---

## 行動規範

このプロジェクトは、すべての参加者に対して敬意と礼儀を持って接することを期待しています。建設的で友好的なコミュニティを維持するため、以下を守ってください：

- 他の貢献者を尊重する
- 建設的なフィードバックを提供する
- 異なる意見や経験を受け入れる

---

## 開発環境のセットアップ

### 必要な環境

- **Node.js** 18 以上
- **Rust** 1.77 以上
- **macOS** 12.0 (Monterey) 以上
- **Ollama** （ローカルLLMランタイム）

### セットアップ手順

1. **リポジトリのクローン**

   ```bash
   git clone https://github.com/sk8metalme/honnyaku.git
   cd honnyaku
   ```

2. **依存関係のインストール**

   ```bash
   # フロントエンド依存関係
   npm install

   # Rust依存関係（自動的にインストールされます）
   ```

3. **Ollamaのセットアップ**

   ```bash
   # Ollamaをインストール
   brew install ollama

   # Ollamaを起動
   ollama serve

   # 翻訳モデルをインストール（別ターミナル）
   ollama pull qwen2.5:3b
   ```

4. **開発モードで起動**

   ```bash
   npm run tauri:dev
   ```

---

## 開発ワークフロー

### ブランチ戦略

- **main**: 本番環境相当（常にデプロイ可能な状態）
- **feature/xxx**: 新機能開発用
- **bugfix/xxx**: バグ修正用
- **docs/xxx**: ドキュメント更新用

### 開発フロー

1. **Issueの作成または選択**

   - 新機能やバグ修正を始める前に、関連するIssueを作成または既存のIssueを選択

2. **ブランチの作成**

   ```bash
   git checkout main
   git pull origin main
   git checkout -b feature/your-feature-name
   ```

3. **開発**

   - コードを書く
   - テストを追加/更新
   - コミットメッセージを適切に書く

4. **テストの実行**

   ```bash
   # フロントエンドテスト
   npm run test

   # バックエンドテスト
   cd src-tauri
   cargo test

   # E2Eテスト
   npm run test:e2e
   ```

5. **プルリクエストの作成**

   - mainブランチへのPRを作成
   - 変更内容を詳細に説明
   - 関連するIssueをリンク

---

## コーディング規約

### TypeScript/React

- **ESLint**: `npm run lint`で確認、`npm run lint:fix`で自動修正
- **Prettier**: `npm run format`でフォーマット
- **命名規則**:
  - コンポーネント: PascalCase（例: `TranslationPopup.tsx`）
  - フック: camelCaseで`use`プレフィックス（例: `useTranslation.ts`）
  - ユーティリティ: camelCase（例: `language-detect.ts`）

### Rust

- **Cargo fmt**: コードフォーマット
  ```bash
  cd src-tauri
  cargo fmt
  ```
- **Cargo clippy**: Linter
  ```bash
  cargo clippy
  ```
- **命名規則**:
  - 関数: snake_case
  - 構造体: PascalCase
  - 定数: UPPER_SNAKE_CASE

### コミットメッセージ

[Conventional Commits](https://www.conventionalcommits.org/)形式を推奨：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type:**
- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメント
- `style`: コードスタイル（フォーマット）
- `refactor`: リファクタリング
- `test`: テスト追加/修正
- `chore`: その他（ビルド、依存関係更新など）

**例:**
```
feat(translation): 韓国語対応を追加

日本語↔韓国語、英語↔韓国語の翻訳に対応
言語検出ロジックにハングル文字の検出を追加

Closes #42
```

---

## テスト

### テストの種類

1. **フロントエンド単体テスト** (Vitest)
   ```bash
   npm run test
   npm run test:watch  # ウォッチモード
   npm run test:coverage  # カバレッジ
   ```

2. **バックエンド単体テスト** (Cargo Test)
   ```bash
   cd src-tauri
   cargo test
   ```

3. **E2E統合テスト** (WebdriverIO)
   ```bash
   npm run test:e2e
   ```

### テストの追加

- 新機能には必ずテストを追加
- バグ修正には再現テストを追加
- テストカバレッジ95%以上を維持

### テストファイルの配置

- **フロントエンド**: `src/**/__tests__/*.test.ts`
- **バックエンド**: `src-tauri/src/services/*.rs`（`#[cfg(test)]`モジュール内）
- **E2E**: `e2e-tests/specs/*.spec.js`

---

## プルリクエストの提出

### PRチェックリスト

提出前に以下を確認してください：

- [ ] すべてのテストがパスする（`npm run test` + `cargo test`）
- [ ] Linterエラーがない（`npm run lint` + `cargo clippy`）
- [ ] コードがフォーマットされている（`npm run format` + `cargo fmt`）
- [ ] 変更内容がREADME.mdや関連ドキュメントに反映されている
- [ ] コミットメッセージがConventional Commits形式
- [ ] PR説明が詳細で、変更の理由と内容を説明している

### PR説明テンプレート

```markdown
## 概要

このPRは[機能/バグ修正/ドキュメント]を[追加/修正/削除]します。

## 変更内容

- 変更点1
- 変更点2
- 変更点3

## テスト

- [ ] フロントエンド単体テスト追加/更新
- [ ] バックエンド単体テスト追加/更新
- [ ] E2Eテスト追加/更新
- [ ] 手動テスト完了

## スクリーンショット（該当する場合）

[スクリーンショットを追加]

## 関連Issue

Closes #[Issue番号]
```

### レビュープロセス

1. PRを作成すると、自動的にCIが実行されます
2. メンテナーがレビューします
3. フィードバックに対応します
4. 承認後、mainブランチにマージされます

---

## アーキテクチャ

### プロジェクト構造

```
honnyaku/
├── src/                      # フロントエンド (React + TypeScript)
│   ├── components/          # UIコンポーネント
│   ├── hooks/               # カスタムReact Hooks
│   ├── lib/                 # ユーティリティ関数
│   └── types/               # TypeScript型定義
├── src-tauri/               # バックエンド (Rust)
│   └── src/
│       ├── services/        # ビジネスロジック
│       ├── lib.rs           # Tauriコマンド定義
│       └── main.rs          # エントリーポイント
├── e2e-tests/               # E2E統合テスト
└── .kiro/                   # Kiro仕様駆動開発
```

### 主要コンポーネント

- **TranslationPopup**: 翻訳結果表示ポップアップ
- **SettingsPanel**: 設定画面パネル
- **useTranslationFlow**: 翻訳フロー管理Hook
- **TranslationService** (Rust): Ollama翻訳サービス
- **ShortcutService** (Rust): グローバルショートカット管理

---

## よくある質問

### Q: 開発中にOllamaエラーが発生します

**A**: Ollamaが起動していることを確認してください：
```bash
ollama serve
```

### Q: E2Eテストが失敗します

**A**:
1. プロダクションビルドが存在するか確認: `npm run tauri:build`
2. tauri-driverがインストールされているか確認: `cargo install tauri-driver`
3. アクセシビリティ権限が許可されているか確認

### Q: TypeScriptの型エラーが解決しない

**A**: 型定義を再生成してください：
```bash
npm run typecheck
```

---

## サポート

質問やサポートが必要な場合：

- [GitHub Discussions](https://github.com/sk8metalme/honnyaku/discussions)で質問
- [Issues](https://github.com/sk8metalme/honnyaku/issues)でバグ報告

---

ありがとうございます！あなたの貢献がHonnyakuをより良いものにします。
