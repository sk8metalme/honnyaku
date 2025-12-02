import { expect } from '@wdio/globals';

describe('Honnyaku - Settings Panel', () => {
  it('should open settings panel when settings button is clicked', async () => {
    // 設定ボタンをクリック
    const settingsButton = await $('button[aria-label="設定"]');
    await settingsButton.click();

    // 設定パネルが表示されることを確認
    await browser.pause(500); // アニメーション待ち
    const settingsPanel = await $('div*=設定');
    await expect(settingsPanel).toBeDisplayed();
  });

  it('should display Ollama settings fields', async () => {
    // Ollamaモデル名の入力フィールドを確認
    const modelInput = await $('input[placeholder*="モデル"]');
    await expect(modelInput).toBeDisplayed();

    // Ollamaエンドポイントの入力フィールドを確認
    const endpointInput = await $('input[placeholder*="エンドポイント"]');
    await expect(endpointInput).toBeDisplayed();
  });

  it('should display shortcut settings field', async () => {
    // ショートカットキーの入力フィールドを確認
    const shortcutInput = await $('input[placeholder*="ショートカット"]');
    await expect(shortcutInput).toBeDisplayed();
  });

  it('should have save and close buttons', async () => {
    // 保存ボタンが存在することを確認
    const saveButton = await $('button*=保存');
    await expect(saveButton).toBeDisplayed();

    // 閉じるボタンまたはキャンセルボタンが存在することを確認
    const closeButton = await $('button[aria-label="閉じる"]');
    await expect(closeButton).toBeDisplayed();
  });

  it('should close settings panel when close button is clicked', async () => {
    // 閉じるボタンをクリック
    const closeButton = await $('button[aria-label="閉じる"]');
    await closeButton.click();

    // 設定パネルが閉じることを確認
    await browser.pause(500); // アニメーション待ち
    // パネルが非表示になっていることを確認（要素が存在しないか、非表示）
    const settingsPanel = await $('div*=Ollama設定');
    const isDisplayed = await settingsPanel.isDisplayed().catch(() => false);
    expect(isDisplayed).toBe(false);
  });
});
