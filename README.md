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

## デモ

<p align="center">
  <img src="./docs/demo.gif" alt="Honnyaku Demo" width="800">
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

### 2.1 PLaMo-2-Translate（翻訳特化モデル）のインストール

[Preferred Networks社](https://www.preferred.jp/)が開発した、翻訳タスクに特化した高品質モデルです。

#### 特徴

- **パラメータ数**: 10B（高品質な翻訳を実現）
- **対応言語**: 日本語 ↔ 英語
- **ライセンス**: PLaMo community license（非商用利用は自由、商用利用は要申請）
- **Ollama対応**: GGUF形式で利用可能

#### Ollamaで直接インストール

```bash
# 推奨: Q4量子化版（バランス重視、約5.6GB）
ollama pull mitmul/plamo-2-translate:Q4_K_M

# または: Q2量子化版（最小サイズ、約3.5GB）
ollama pull mitmul/plamo-2-translate:Q2_K_S

# 動作確認
ollama run mitmul/plamo-2-translate:Q4_K_M
```

#### 量子化バリエーション

| 量子化 | サイズ | 品質 | 速度 | 推奨用途 |
|--------|--------|------|------|---------|
| Q4_K_M | 約5.6GB | 高品質 | 中速 | バランス重視（推奨） |
| Q2_K_S | 約3.5GB | 標準 | 高速 | 高速動作優先 |
| IQ2_M | 約3.0GB | 標準 | 最速 | 最小サイズ優先 |

#### ライセンス注意事項

**非商用利用**: 自由に使用可能
**商用利用**: [申請フォーム](https://forms.gle/mTL8tBLrMYXKNZD56)からPreferred Networks社への承認申請が必要

#### 参考リンク

- [PLaMo-2-Translate (HuggingFace)](https://huggingface.co/pfnet/plamo-2-translate)
- [PLaMo GGUF版 (Ollama)](https://ollama.com/mitmul/plamo-2-translate)
- [mmnga/plamo-2-translate-gguf](https://huggingface.co/mmnga/plamo-2-translate-gguf)

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
- **翻訳専用**: `mitmul/plamo-2-translate:Q4_K_M`（翻訳タスクに最適化、10B、約5.6GB、非商用利用推奨）

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

### 「壊れているため開けません」と表示される

ダウンロードしたDMGファイルを開こうとした際に「"honnyaku"は壊れているため開けません。ゴミ箱に入れる必要があります。」と表示される場合があります。

**原因**: アプリがコード署名されていないため、macOSのGatekeeper機能がブロックしています。

**解決方法**:

ターミナルで以下のコマンドを実行してから、再度DMGファイルを開いてください：

```bash
# ダウンロードしたDMGファイルの隔離属性を削除
xattr -d com.apple.quarantine ~/Downloads/honnyaku_*.dmg
```

または、すでにアプリをインストール済みの場合：

```bash
# インストール済みアプリの隔離属性を削除
xattr -d com.apple.quarantine /Applications/honnyaku.app
```

> **注意**: このコマンドはmacOSのセキュリティ機能を回避するものです。信頼できるソース（公式GitHub Releases）からダウンロードしたファイルに対してのみ実行してください。

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
