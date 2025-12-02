import { expect } from '@wdio/globals';

describe('Honnyaku - Application Launch', () => {
  it('should launch the application successfully', async () => {
    // アプリケーションウィンドウが存在することを確認
    const title = await browser.getTitle();
    expect(title).toBeTruthy();
  });

  it('should display the main application content', async () => {
    // アプリケーション名（Honnyaku）が表示されていることを確認
    const appTitle = await $('h1');
    await expect(appTitle).toBeDisplayed();
    const titleText = await appTitle.getText();
    expect(titleText).toContain('Honnyaku');
  });

  it('should display status indicators', async () => {
    // アクセシビリティ権限のステータス表示を確認
    const accessibilityStatus = await $('span*=アクセシビリティ権限');
    await expect(accessibilityStatus).toBeDisplayed();

    // ショートカットのステータス表示を確認
    const shortcutStatus = await $('span*=ショートカット');
    await expect(shortcutStatus).toBeDisplayed();

    // モデルのステータス表示を確認
    const modelStatus = await $('span*=モデル');
    await expect(modelStatus).toBeDisplayed();
  });

  it('should have settings button', async () => {
    // 設定ボタンが存在することを確認
    const settingsButton = await $('button[aria-label="設定"]');
    await expect(settingsButton).toBeDisplayed();
  });

  it('should display usage instructions', async () => {
    // 使い方セクションが表示されていることを確認
    const usageTitle = await $('h3*=使い方');
    await expect(usageTitle).toBeDisplayed();

    // 使い方の説明リストが表示されていることを確認
    const usageList = await $('ol');
    await expect(usageList).toBeDisplayed();
  });
});
