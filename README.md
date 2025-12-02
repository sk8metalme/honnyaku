# Honnyaku

AI翻訳デスクトップアプリ - ローカルLLM（Ollama）を使用したテキスト翻訳

## 機能

- グローバルショートカット（デフォルト: `Cmd+J`）で選択テキストを翻訳
- 日本語 ↔ 英語の自動言語検出
- Ollama（ローカルLLM）でプライベートな翻訳
- 翻訳結果のワンクリックコピー

## 必要条件

- macOS（アクセシビリティ権限が必要）
- Node.js 18+
- Rust（Tauri開発用）
- Ollama

## セットアップ

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

## 使い方

1. アプリを起動
2. 翻訳したいテキストを選択
3. `Cmd+J`を押す
4. 翻訳結果がポップアップで表示
5. クリックで結果をコピー

## 設定

設定画面（歯車アイコン）から以下を変更可能：

- **ショートカットキー**: グローバルショートカットのカスタマイズ
- **Ollamaエンドポイント**: Ollamaサーバーのアドレス
- **Ollamaモデル**: 使用するモデルの選択

## 技術スタック

- **フロントエンド**: React + TypeScript + Tailwind CSS
- **バックエンド**: Tauri v2 (Rust)
- **LLM**: Ollama（ローカル）

## トラブルシューティング

### 翻訳が失敗する

1. Ollamaが起動しているか確認: `ollama serve`
2. モデルがインストールされているか確認: `ollama list`
3. 設定画面で「接続をテスト」を実行

### ショートカットが効かない

1. システム環境設定 > プライバシーとセキュリティ > アクセシビリティ
2. Honnyakuアプリにチェックを入れる
3. アプリを再起動

## ライセンス

MIT
