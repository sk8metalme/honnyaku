# Honnyaku（翻訳）

<p align="center">
  <strong>macOS向けAI翻訳デスクトップアプリ</strong><br>
  ローカルLLM（Ollama）を使用した、プライバシー重視のテキスト翻訳ツール
</p>

<p align="center">
  <a href="https://github.com/sk8metalme/honnyaku/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/sk8metalme/honnyaku/ci.yml?branch=main&label=CI" alt="CI"></a>
  <a href="https://github.com/sk8metalme/honnyaku/releases"><img src="https://img.shields.io/github/v/release/sk8metalme/honnyaku" alt="Release"></a>
  <a href="https://github.com/sk8metalme/honnyaku/blob/main/LICENSE"><img src="https://img.shields.io/github/license/sk8metalme/honnyaku" alt="License"></a>
  <a href="https://github.com/sk8metalme/honnyaku"><img src="https://img.shields.io/badge/platform-macOS-lightgrey" alt="Platform"></a>
</p>

---

## ✨ 主要機能

### 🚀 グローバルショートカット翻訳
- **どこでも翻訳**: デフォルトショートカット `Cmd+J` で、任意のアプリケーションで選択したテキストを即座に翻訳
- **カスタマイズ可能**: ショートカットキーを自由に変更可能

### 🌐 自動言語検出
- **日本語 ↔ 英語**: テキストの言語を自動検出し、適切な方向へ翻訳
- **シームレスな体験**: 言語を手動で選択する必要なし

### 🔒 プライバシー重視
- **完全ローカル処理**: Ollama（ローカルLLM）を使用し、翻訳データが外部サーバーに送信されない
- **オフライン動作**: インターネット接続不要（Ollamaセットアップ後）

### ⚡ 高速レスポンス
- **軽量設計**: アプリサイズ4.8MB、メモリ使用量100MB以下
- **モデルプリロード**: 初回翻訳を高速化する事前読み込み機能

### 📋 便利な機能
- **ワンクリックコピー**: 翻訳結果をクリップボードに即座にコピー
- **ポップアップ表示**: 作業を邪魔しない小さなポップアップで結果を表示
- **設定のカスタマイズ**: Ollamaエンドポイント、モデル、ショートカットの変更

## 📋 必要条件

### エンドユーザー向け
- **macOS 12.0 (Monterey)** 以上
- **Ollama**: ローカルLLMランタイム
- **アクセシビリティ権限**: グローバルショートカット動作に必要

### 開発者向け
- Node.js 18+
- Rust 1.77+
- Tauri CLI

---

## 📥 インストール

### 方法1: リリース版のインストール（推奨）

1. **DMGファイルのダウンロード**

   [最新リリース](https://github.com/sk8metalme/honnyaku/releases)から`honnyaku_x.x.x_aarch64.dmg`をダウンロード

2. **アプリのインストール**

   - DMGファイルを開く
   - Honnyaku.appを「アプリケーション」フォルダにドラッグ&ドロップ

3. **初回起動**

   - アプリケーションフォルダからHonnyakuを起動
   - 「開発元を確認できないため開けません」と表示された場合：
     - システム環境設定 > プライバシーとセキュリティ
     - 「このまま開く」をクリック

4. **アクセシビリティ権限の許可**

   - 初回起動時にアクセシビリティ権限のダイアログが表示される
   - 「システム環境設定を開く」をクリック
   - Honnyakuにチェックを入れる

### 方法2: ソースからビルド

開発者向けのビルド手順は[CONTRIBUTING.md](./CONTRIBUTING.md)を参照してください。

---

## ⚙️ セットアップ

### 1. Ollamaのインストール

Ollamaをインストールして起動します：

```bash
# Homebrewでインストール
brew install ollama

# Ollamaを起動
ollama serve
```

または[公式サイト](https://ollama.ai)からダウンロード。

### 2. 翻訳モデルのインストール

用途に応じてqwen2.5モデルをインストールします：

```bash
# 推奨: 標準モデル（バランス重視）
ollama pull qwen2.5:3b

# オプション: 追加モデル
ollama pull qwen2.5:0.5b  # 最速（395MB）
ollama pull qwen2.5:1.5b  # 高速（986MB）
ollama pull qwen2.5:7b    # 高品質（4.7GB）
```

| モデル | サイズ | 特徴 |
|--------|--------|------|
| qwen2.5:0.5b | 395MB | 最速だが品質は低め |
| qwen2.5:1.5b | 986MB | 速度と品質のバランス |
| qwen2.5:3b | 1.9GB | 標準的な品質（デフォルト） |
| qwen2.5:7b | 4.7GB | 高品質だが遅め |

### 2.1 日本語特化モデル（ABEJA-Qwen2.5-7b-Japanese）のインストール

日本語の翻訳品質を向上させたい場合は、日本語に特化したABEJA-Qwen2.5-7b-Japaneseモデルをインストールできます。

#### GGUFファイルのダウンロード

```bash
# huggingface_hubをインストール
pip install huggingface_hub

# GGUFファイルをダウンロード（Q4_K_M推奨）
huggingface-cli download mmnga/ABEJA-Qwen2.5-7b-Japanese-v0.1-gguf \
  ABEJA-Qwen2.5-7b-Japanese-v0.1-Q4_K_M.gguf \
  --local-dir .
```

#### Modelfileの作成とOllamaへの登録

```bash
# Modelfileを作成
cat << 'EOF' > Modelfile
FROM ./ABEJA-Qwen2.5-7b-Japanese-v0.1-Q4_K_M.gguf
TEMPLATE """{{ if .System }}<|im_start|>system
{{ .System }}<|im_end|>
{{ end }}{{ if .Prompt }}<|im_start|>user
{{ .Prompt }}<|im_end|>
{{ end }}<|im_start|>assistant
{{ .Response }}<|im_end|>
"""
PARAMETER stop <|im_start|>
PARAMETER stop <|im_end|>
PARAMETER stop <|endoftext|>
EOF

# Ollamaにモデルを登録
ollama create abeja-qwen2.5-7b-jp -f Modelfile

# 動作確認
ollama run abeja-qwen2.5-7b-jp "こんにちは"
```

#### 量子化バリエーション

用途に応じて異なる量子化レベルを選択できます：

| ファイル名 | サイズ | 品質 | 速度 |
|-----------|--------|------|------|
| Q4_K_M.gguf | 約4.3GB | バランス重視（推奨） | 高速 |
| Q5_K_M.gguf | 約5.0GB | やや高品質 | 中速 |
| Q8_0.gguf | 約7.5GB | 高品質 | 低速 |

### 3. アプリのビルド

```bash
# 依存関係をインストール
npm install

# 開発モードで起動
npm run tauri:dev

# 本番ビルド
npm run tauri:build
```

## 🎯 使い方

### 基本的な使い方

1. **Honnyakuを起動**

   アプリケーションフォルダからHonnyakuを起動します。起動後、メニューバーまたはDockに常駐します。

2. **テキストを選択**

   任意のアプリケーション（ブラウザ、エディタ、PDFビューアーなど）で翻訳したいテキストを選択します。

3. **ショートカットキーを押す**

   デフォルト: `Cmd+J`（設定で変更可能）

4. **翻訳結果を確認**

   カーソル位置近くにポップアップが表示され、翻訳結果が表示されます。

5. **結果をコピー（オプション）**

   - ポップアップ内のコピーボタンをクリック
   - または`Cmd+C`で翻訳結果をクリップボードにコピー

6. **ポップアップを閉じる**

   - `Esc`キーを押す
   - ポップアップ外をクリック

### 設定のカスタマイズ

設定画面（歯車アイコン）から以下をカスタマイズできます：

#### ショートカットキー
- **デフォルト**: `CommandOrControl+J`（`Cmd+J`）
- **変更方法**: 設定画面で新しいショートカットを入力

#### Ollama設定
- **エンドポイント**: Ollamaサーバーのアドレス（デフォルト: `http://localhost:11434`）
- **モデル**: 使用するLLMモデル（デフォルト: `qwen2.5:3b`）
- **接続テスト**: 「接続をテスト」ボタンでOllamaの接続状態を確認

## 🛠️ 技術スタック

- **フロントエンド**: React 19 + TypeScript 5 + Tailwind CSS 4
- **バックエンド**: Tauri v2 (Rust 1.77+)
- **LLM**: Ollama（ローカルLLM）
- **テスト**: Vitest (フロントエンド) + Cargo Test (バックエンド) + WebdriverIO (E2E)
- **ビルドツール**: Vite 7

## ❓ FAQ

### Q: どの言語ペアがサポートされていますか？

**A**: 現在、日本語 ↔ 英語のみサポートしています。将来的に韓国語対応を予定しています。

### Q: インターネット接続は必要ですか？

**A**: Ollamaセットアップ後は、完全にオフラインで動作します。翻訳データが外部に送信されることはありません。

### Q: どのOllamaモデルを使用すべきですか？

**A**:
- **推奨**: `qwen2.5:3b`（バランス重視、1.9GB）
- **高速**: `qwen2.5:0.5b`（最速、395MB、品質は低め）
- **高品質**: `qwen2.5:7b`（高品質、4.7GB、遅め）
- **日本語特化**: `abeja-qwen2.5-7b-jp`（日本語に最適化、約4.3GB）

### Q: 翻訳速度を改善するには？

**A**: 以下の方法を試してください：
1. より小さいモデルを使用（`qwen2.5:0.5b`または`qwen2.5:1.5b`）
2. OllamaのGPU設定を確認
3. 他のアプリケーションを終了してメモリを確保

### Q: ショートカットが他のアプリと競合します

**A**: 設定画面からショートカットキーを変更できます。例: `Cmd+Shift+T`、`Cmd+Option+J`など

### Q: Windows/Linuxで使用できますか？

**A**: 現在はmacOSのみサポートしています。将来的にクロスプラットフォーム対応を検討中です。

## 🐛 トラブルシューティング

### 翻訳が失敗する

1. **Ollamaが起動しているか確認**
   ```bash
   # ターミナルで実行
   ollama serve
   ```

2. **モデルがインストールされているか確認**
   ```bash
   ollama list
   ```
   使用するモデル（例: `qwen2.5:3b`）がリストにあることを確認

3. **Ollama接続テスト**
   - 設定画面を開く
   - 「接続をテスト」ボタンをクリック
   - 接続状態を確認

4. **ログを確認**
   - アプリを開発モードで起動: `npm run tauri:dev`
   - コンソールでエラーメッセージを確認

### ショートカットが効かない

1. **アクセシビリティ権限を確認**
   - システム環境設定 > プライバシーとセキュリティ > アクセシビリティ
   - Honnyakuアプリにチェックが入っているか確認

2. **アプリを再起動**
   - Honnyakuを完全に終了
   - 再度起動

3. **ショートカット競合を確認**
   - システム環境設定 > キーボード > ショートカット
   - 同じショートカットが他のアプリで使用されていないか確認

### アプリが起動しない

1. **macOSバージョンを確認**
   - macOS 12.0 (Monterey) 以上が必要

2. **セキュリティ設定を確認**
   - システム環境設定 > プライバシーとセキュリティ
   - 「このまま開く」ボタンをクリック

3. **ログを確認**
   - コンソール.app を開く
   - "honnyaku" で検索してエラーログを確認

## 🤝 コントリビューション

コントリビューションを歓迎します！以下の方法で参加できます：

1. **バグ報告**: [Issues](https://github.com/sk8metalme/honnyaku/issues)で報告
2. **機能リクエスト**: [Discussions](https://github.com/sk8metalme/honnyaku/discussions)で提案
3. **プルリクエスト**: [CONTRIBUTING.md](./CONTRIBUTING.md)を参照

## 📄 ライセンス

MIT License - 詳細は[LICENSE](./LICENSE)ファイルを参照してください。

## 🙏 謝辞

- [Tauri](https://tauri.app/) - クロスプラットフォームデスクトップアプリフレームワーク
- [Ollama](https://ollama.ai/) - ローカルLLMランタイム
- [Qwen](https://qwenlm.github.io/) - 高品質LLMモデル
