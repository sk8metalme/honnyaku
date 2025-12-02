# E2E Test Specification: ai-translation-desktop e2eテスト

**Author**: Auto-generated
**Date**: 2025-12-01
**Version**: 1.0

## 1. Overview

### 1.1 Purpose
ai-translation-desktopのエンドユーザーシナリオが完全に動作することを確認する

Example: To verify that end users can successfully complete critical user journeys in the  from start to finish in a real browser environment.

### 1.2 Scope
ai-translation-desktopの全ユーザーフロー

Example: This test specification covers the complete user registration and login flow, including UI interactions, form validations, and successful authentication.

### 1.3 Testing Tool
- **Tool**: Vitest
- **Version**: 1.0.0

Example:
- **Tool**: Playwright / Selenium WebDriver / Cypress
- **Version**: 1.40.0 / 4.15.0 / 13.6.0

## 2. Test Environment

### 2.1 Software Requirements
- Browser Automation Tool:  1.0.0
- Browsers: 
- Application Environment: 
- Backend API: 
- Test Data Management: 

Example:
- Browser Automation Tool: Playwright 1.40.0
- Browsers: Chrome 120, Firefox 121, Safari 17
- Application Environment: [https://staging.example.com](https://staging.example.com)
- Backend API: [https://api-staging.example.com](https://api-staging.example.com)
- Test Data Management: Test database with seeded data

### 2.2 Hardware Requirements
- Test Machine: 
- Display Resolution: 
- Network: 

Example:
- Test Machine: macOS/Windows/Linux with 8GB RAM
- Display Resolution: 1920x1080 (Desktop), 768x1024 (Tablet), 375x667 (Mobile)
- Network: Stable internet connection (minimum 10 Mbps)

### 2.3 Test Data
- Test user accounts: ``
- Test data setup script: ``
- Data cleanup script: ``
- Environment variables: ``

## 3. User Flows

### 3.1 User Journey Map

```text
 →  →  →  → 
```

Example:

```text
Landing Page → Sign Up Form → Email Verification → Profile Setup → Dashboard
```

### 3.2 User Flow Details

| Flow ID | Flow Name | Description | Priority | Steps |
|---------|-----------|-------------|----------|-------  | High/Medium/Low   |  | High/Medium/Low |  |

Example:

| Flow ID | Flow Name | Description | Priority | Steps |
|---------|-----------|-------------|----------|-------|
| UF-001 | User Registration | New user signs up and verifies email | High | 5 |
| UF-002 | Product Purchase | User browses, adds to cart, and completes checkout | High | 8 |
| UF-003 | Password Reset | User resets forgotten password | Medium | 4 |

### 3.3 Browser/Device Matrix

Test each user flow on the following combinations:

| Browser | Version | Desktop | Tablet | Mobile | Priority |
|---------|---------|---------|--------|--------|----------|
| Chrome |  | ✓ | ✓ | ✓ | High |
| Firefox |  | ✓ | - | - | Medium |
| Safari |  | ✓ | ✓ | ✓ | High |
| Edge |  | ✓ | - | - | Low |

Example:

| Browser | Version | Desktop | Tablet | Mobile | Priority |
|---------|---------|---------|--------|--------|----------|
| Chrome | 120+ | ✓ | ✓ | ✓ | High |
| Firefox | 121+ | ✓ | - | - | Medium |
| Safari | 17+ | ✓ | ✓ | ✓ | High |
| Edge | 120+ | ✓ | - | - | Low |

**Priority Guide**:
- High: Must test on all marked platforms
- Medium: Test on desktop only
- Low: Test if time permits

## 4. Test Cases

### Test Case E2E-001: グローバルショートカット機能 - ハッピーパス

**Description**: As a ユーザー, I want グローバルショートカットキーで翻訳を起動できること, so that どのアプリケーションを使用中でも素早く翻訳を実行できるが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. When ユーザーがデフォルトショートカット（Cmd+Shift+T）を押下した時, the 翻訳アプリ shall 選択テキストの取得処理を開始する...を実行
2. While 翻訳アプリがバックグラウンドで動作中, the 翻訳アプリ shall グローバルショートカットキーの入力を常に監視する...を実行
3. If ショートカットキーが他のアプリケーションと競合している場合, the 翻訳アプリ shall 競合を検出しユーザーに通知する...を実行

**Expected Results**:
- ユーザーがデフォルトショートカット（Cmd+Shift+T）を押下した時, the 翻訳アプリ shall 選択テキストの取得処理を開始する
- 翻訳アプリがバックグラウンドで動作中, the 翻訳アプリ shall グローバルショートカットキーの入力を常に監視する
- ショートカットキーが他のアプリケーションと競合している場合, the 翻訳アプリ shall 競合を検出しユーザーに通知する

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-002: グローバルショートカット機能 - エラーフロー

**Description**: エラー発生時に適切にハンドリングされることを確認

**Preconditions**:
- アプリケーションが起動している

**Test Steps**:
1. 無効な入力でフローを開始する
2. エラーメッセージが表示されることを確認する
3. ユーザーが回復できることを確認する

**Expected Results**:
- 適切なエラーメッセージが表示される
- ユーザーが操作を継続できる
- データの整合性が保たれる

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-003: 選択テキスト取得機能 - ハッピーパス

**Description**: As a ユーザー, I want 選択中のテキストを自動的に取得してほしい, so that 手動でコピー&ペーストする手間を省けるが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. When グローバルショートカットが押下された時, the 翻訳アプリ shall システムにCmd+Cを送信して選択テキストをクリップボードにコピーする...を実行
2. When クリップボードにテキストがコピーされた時, the 翻訳アプリ shall クリップボードからテキストを読み取る...を実行
3. If クリップボードが空または非テキストデータの場合, the 翻訳アプリ shall 「翻訳するテキストが選択されていません」とユーザーに通知する...を実行

**Expected Results**:
- グローバルショートカットが押下された時, the 翻訳アプリ shall システムにCmd+Cを送信して選択テキストをクリップボードにコピーする
- クリップボードにテキストがコピーされた時, the 翻訳アプリ shall クリップボードからテキストを読み取る
- クリップボードが空または非テキストデータの場合, the 翻訳アプリ shall 「翻訳するテキストが選択されていません」とユーザーに通知する

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-004: 選択テキスト取得機能 - エラーフロー

**Description**: エラー発生時に適切にハンドリングされることを確認

**Preconditions**:
- アプリケーションが起動している

**Test Steps**:
1. 無効な入力でフローを開始する
2. エラーメッセージが表示されることを確認する
3. ユーザーが回復できることを確認する

**Expected Results**:
- 適切なエラーメッセージが表示される
- ユーザーが操作を継続できる
- データの整合性が保たれる

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-005: 言語自動検出機能 - ハッピーパス

**Description**: As a ユーザー, I want 翻訳元言語を自動検出してほしい, so that 手動で言語を選択する手間を省けるが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. When テキストが取得された時, the 言語検出モジュール shall 入力テキストの言語を日本語または英語から判定する...を実行
2. When 日本語テキストが検出された時, the 翻訳アプリ shall 英語への翻訳を実行する...を実行
3. When 英語テキストが検出された時, the 翻訳アプリ shall 日本語への翻訳を実行する...を実行

**Expected Results**:
- テキストが取得された時, the 言語検出モジュール shall 入力テキストの言語を日本語または英語から判定する
- 日本語テキストが検出された時, the 翻訳アプリ shall 英語への翻訳を実行する
- 英語テキストが検出された時, the 翻訳アプリ shall 日本語への翻訳を実行する

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-006: 言語自動検出機能 - エラーフロー

**Description**: エラー発生時に適切にハンドリングされることを確認

**Preconditions**:
- アプリケーションが起動している

**Test Steps**:
1. 無効な入力でフローを開始する
2. エラーメッセージが表示されることを確認する
3. ユーザーが回復できることを確認する

**Expected Results**:
- 適切なエラーメッセージが表示される
- ユーザーが操作を継続できる
- データの整合性が保たれる

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-007: 翻訳処理機能 - ハッピーパス

**Description**: As a ユーザー, I want テキストを高品質に翻訳してほしい, so that 正確な翻訳結果を得られるが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. When 翻訳リクエストが発生した時, the 翻訳アプリ shall 設定されたLLMプロバイダー（OllamaまたはClaude API）に翻訳リクエストを送信する...を実行
2. When Ollamaが選択されている時, the 翻訳アプリ shall localhost:11434のOllama APIにリクエストを送信する...を実行
3. When Claude APIが選択されている時, the 翻訳アプリ shall api.anthropic.comにHTTPSでリクエストを送信する...を実行

**Expected Results**:
- 翻訳リクエストが発生した時, the 翻訳アプリ shall 設定されたLLMプロバイダー（OllamaまたはClaude API）に翻訳リクエストを送信する
- Ollamaが選択されている時, the 翻訳アプリ shall localhost:11434のOllama APIにリクエストを送信する
- Claude APIが選択されている時, the 翻訳アプリ shall api.anthropic.comにHTTPSでリクエストを送信する

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-008: 翻訳処理機能 - エラーフロー

**Description**: エラー発生時に適切にハンドリングされることを確認

**Preconditions**:
- アプリケーションが起動している

**Test Steps**:
1. 無効な入力でフローを開始する
2. エラーメッセージが表示されることを確認する
3. ユーザーが回復できることを確認する

**Expected Results**:
- 適切なエラーメッセージが表示される
- ユーザーが操作を継続できる
- データの整合性が保たれる

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-009: ポップアップ表示機能 - ハッピーパス

**Description**: As a ユーザー, I want 翻訳結果を小さなポップアップで確認したい, so that 作業中の画面を大きく遮らないが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. When 翻訳が完了した時, the ポップアップウィンドウ shall マウスカーソル位置の近くに表示される...を実行
2. While ポップアップが表示中, the ポップアップ shall 常に他のウィンドウの前面に表示される（always on top）...を実行
3. When ユーザーがポップアップ外をクリックした時, the ポップアップ shall 自動的に閉じる...を実行

**Expected Results**:
- 翻訳が完了した時, the ポップアップウィンドウ shall マウスカーソル位置の近くに表示される
- ポップアップが表示中, the ポップアップ shall 常に他のウィンドウの前面に表示される（always on top）
- ユーザーがポップアップ外をクリックした時, the ポップアップ shall 自動的に閉じる

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-010: ポップアップ表示機能 - エラーフロー

**Description**: エラー発生時に適切にハンドリングされることを確認

**Preconditions**:
- アプリケーションが起動している

**Test Steps**:
1. 無効な入力でフローを開始する
2. エラーメッセージが表示されることを確認する
3. ユーザーが回復できることを確認する

**Expected Results**:
- 適切なエラーメッセージが表示される
- ユーザーが操作を継続できる
- データの整合性が保たれる

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-011: クリップボードコピー機能 - ハッピーパス

**Description**: As a ユーザー, I want 翻訳結果をワンクリックでコピーしたい, so that 他のアプリケーションにすぐ貼り付けられるが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. When ユーザーがコピーボタンをクリックした時, the 翻訳アプリ shall 翻訳結果をクリップボードにコピーする...を実行
2. When コピーが成功した時, the 翻訳アプリ shall コピー完了のビジュアルフィードバック（チェックマーク等）を表示する...を実行
3. When ユーザーがCmd+Cを押下した時（ポップアップフォーカス中）, the 翻訳アプリ shall 翻訳結果をクリップボードにコピーする...を実行

**Expected Results**:
- ユーザーがコピーボタンをクリックした時, the 翻訳アプリ shall 翻訳結果をクリップボードにコピーする
- コピーが成功した時, the 翻訳アプリ shall コピー完了のビジュアルフィードバック（チェックマーク等）を表示する
- ユーザーがCmd+Cを押下した時（ポップアップフォーカス中）, the 翻訳アプリ shall 翻訳結果をクリップボードにコピーする

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-012: LLMプロバイダー切り替え機能 - ハッピーパス

**Description**: As a ユーザー, I want OllamaとClaude APIを切り替えたい, so that 用途に応じて最適なLLMを選択できるが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. When ユーザーが設定画面でLLMプロバイダーを変更した時, the 翻訳アプリ shall 次回の翻訳から選択されたプロバイダーを使用する...を実行
2. Where Ollamaが選択されている場合, the 翻訳アプリ shall Ollamaの接続状態を確認し、利用可能かどうかを表示する...を実行
3. Where Claude APIが選択されている場合, the 翻訳アプリ shall APIキーが設定されているか確認する...を実行

**Expected Results**:
- ユーザーが設定画面でLLMプロバイダーを変更した時, the 翻訳アプリ shall 次回の翻訳から選択されたプロバイダーを使用する
- Ollamaが選択されている場合, the 翻訳アプリ shall Ollamaの接続状態を確認し、利用可能かどうかを表示する
- Claude APIが選択されている場合, the 翻訳アプリ shall APIキーが設定されているか確認する

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-013: LLMプロバイダー切り替え機能 - エラーフロー

**Description**: エラー発生時に適切にハンドリングされることを確認

**Preconditions**:
- アプリケーションが起動している

**Test Steps**:
1. 無効な入力でフローを開始する
2. エラーメッセージが表示されることを確認する
3. ユーザーが回復できることを確認する

**Expected Results**:
- 適切なエラーメッセージが表示される
- ユーザーが操作を継続できる
- データの整合性が保たれる

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-014: 設定管理機能 - ハッピーパス

**Description**: As a ユーザー, I want アプリの設定をカスタマイズしたい, so that 自分の好みに合わせて使用できるが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. When ユーザーが設定を変更した時, the 翻訳アプリ shall 設定をローカルストレージに永続化する...を実行
2. When アプリが起動した時, the 翻訳アプリ shall 保存された設定を読み込む...を実行
3. The 設定画面 shall ショートカットキーのカスタマイズを提供する...を実行

**Expected Results**:
- ユーザーが設定を変更した時, the 翻訳アプリ shall 設定をローカルストレージに永続化する
- アプリが起動した時, the 翻訳アプリ shall 保存された設定を読み込む
- 設定画面 shall ショートカットキーのカスタマイズを提供する

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-015: 設定管理機能 - エラーフロー

**Description**: エラー発生時に適切にハンドリングされることを確認

**Preconditions**:
- アプリケーションが起動している

**Test Steps**:
1. 無効な入力でフローを開始する
2. エラーメッセージが表示されることを確認する
3. ユーザーが回復できることを確認する

**Expected Results**:
- 適切なエラーメッセージが表示される
- ユーザーが操作を継続できる
- データの整合性が保たれる

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Case E2E-016: 非機能要件 - ハッピーパス

**Description**: As a ユーザー, I want 高速でセキュア、かつ軽量なアプリを使用したい, so that ストレスなく安心して利用できるが達成できることをエンドツーエンドで確認

**Preconditions**:
- アプリケーションが起動している
- テストユーザーが作成されている
- テストデータが準備されている

**Test Steps**:
1. The 翻訳アプリ shall ショートカット押下から翻訳結果表示まで3秒以内（Ollama使用時、ローカルネットワーク）に完了する...を実行
2. The 翻訳アプリ shall アイドル時のメモリ使用量を100MB以下に維持する...を実行
3. The 翻訳アプリ shall Claude APIとの通信にHTTPSを使用する...を実行

**Expected Results**:
- 翻訳アプリ shall ショートカット押下から翻訳結果表示まで3秒以内（Ollama使用時、ローカルネットワーク）に完了する
- 翻訳アプリ shall アイドル時のメモリ使用量を100MB以下に維持する
- 翻訳アプリ shall Claude APIとの通信にHTTPSを使用する

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

## 5. Test Execution Summary

| ID | Test Name | Flow | Browser | Device | Status | Date | Notes |
|----|-----------|------|---------|--------|--------|------|-------|
| E2E-001 |  |  | Chrome | Desktop 
| E2E-002 |  |  | Chrome | Desktop 
| E2E-003 |  |  | Firefox | Desktop 
| E2E-004 |  |  | Chrome | Mobile 

## 6. Defects Found

| Defect ID | Severity | Description | Browser/Device | Screenshot/Video | Status |
|-----------|----------|-------------|----------------|------------------|--------|
| | High/Medium/Low  Open/In Progress/Fixed/Closed |

## 7. Sign-off

**Tested By**: _______________
**Date**: _______________
**Approved By**: _______________
**Date**: _______________

---

## Appendix A: Test Environment Setup

### Playwright Setup

```bash
# Install Playwright
npm install -D @playwright/test

# Install browsers
npx playwright install

# Run tests
npx playwright test

# Run tests in UI mode
npx playwright test --ui

# Generate HTML report
npx playwright show-report
```

### Selenium WebDriver Setup

```bash
# Install Selenium (Node.js)
npm install selenium-webdriver

# Download browser drivers
# ChromeDriver, GeckoDriver, etc.

# Run tests
node e2e-tests/registration.test.js
```

### Cypress Setup

```bash
# Install Cypress
npm install -D cypress

# Open Cypress
npx cypress open

# Run tests headless
npx cypress run

# Run specific test
npx cypress run --spec "cypress/e2e/registration.cy.js"
```

## Appendix B: Code Examples

### Example E2E Test Code (Playwright)

```typescript
import { test, expect } from '@playwright/test';

test('User registration flow', async ({ page }) => {
  // Navigate to landing page
  await page.goto('https://staging.example.com');

  // Click sign up button
  await page.click('text=Sign Up');

  // Fill registration form
  await page.fill('input[name="name"]', 'Test User');
  await page.fill('input[name="email"]', 'test@example.com');
  await page.fill('input[name="password"]', 'Test1234!');

  // Submit form
  await page.click('button[type="submit"]');

  // Verify confirmation message
  await expect(page.locator('text=Account created successfully')).toBeVisible();

  // Verify redirect to dashboard
  await expect(page).toHaveURL(/.*dashboard/);
  await expect(page.locator('text=Welcome, Test User')).toBeVisible();
});

test('Registration with invalid email', async ({ page }) => {
  await page.goto('https://staging.example.com/signup');

  await page.fill('input[name="email"]', 'notanemail');
  await page.fill('input[name="password"]', 'Test1234!');
  await page.click('button[type="submit"]');

  // Verify error message
  await expect(page.locator('text=Please enter a valid email')).toBeVisible();
});
```

### Example E2E Test Code (Selenium WebDriver)

```javascript
const { Builder, By, until } = require('selenium-webdriver');

async function testUserRegistration() {
  let driver = await new Builder().forBrowser('chrome').build();

  try {
    // Navigate to landing page
    await driver.get('https://staging.example.com');

    // Click sign up button
    await driver.findElement(By.linkText('Sign Up')).click();

    // Fill registration form
    await driver.findElement(By.name('name')).sendKeys('Test User');
    await driver.findElement(By.name('email')).sendKeys('test@example.com');
    await driver.findElement(By.name('password')).sendKeys('Test1234!');

    // Submit form
    await driver.findElement(By.css('button[type="submit"]')).click();

    // Wait for confirmation
    await driver.wait(until.elementLocated(By.xpath('//*[contains(text(), "Account created")]')), 5000);

    // Verify redirect
    let currentUrl = await driver.getCurrentUrl();
    assert(currentUrl.includes('dashboard'));

  } finally {
    await driver.quit();
  }
}

testUserRegistration();
```

### Example E2E Test Code (Cypress)

```javascript
describe('User Registration Flow', () => {
  it('should complete registration successfully', () => {
    // Navigate to landing page
    cy.visit('https://staging.example.com');

    // Click sign up button
    cy.contains('Sign Up').click();

    // Fill registration form
    cy.get('input[name="name"]').type('Test User');
    cy.get('input[name="email"]').type('test@example.com');
    cy.get('input[name="password"]').type('Test1234!');

    // Submit form
    cy.get('button[type="submit"]').click();

    // Verify confirmation
    cy.contains('Account created successfully').should('be.visible');

    // Verify redirect to dashboard
    cy.url().should('include', '/dashboard');
    cy.contains('Welcome, Test User').should('be.visible');
  });

  it('should show error for invalid email', () => {
    cy.visit('https://staging.example.com/signup');

    cy.get('input[name="email"]').type('notanemail');
    cy.get('input[name="password"]').type('Test1234!');
    cy.get('button[type="submit"]').click();

    // Verify error message
    cy.contains('Please enter a valid email').should('be.visible');
  });
});
```

## Appendix C: Screenshot and Video Configuration

### Playwright Configuration

```typescript
// playwright.config.ts
export default {
  use: {
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
    trace: 'retain-on-failure',
  },
};
```

### Cypress Configuration

```javascript
// cypress.config.js
module.exports = {
  video: true,
  screenshotOnRunFailure: true,
  videosFolder: 'cypress/videos',
  screenshotsFolder: 'cypress/screenshots',
};
```

## Appendix D: Execution Timing

## Phase B (Before Release) - Manual Execution

E2E tests are executed manually before creating a release tag:

1. After PR is merged to main branch
2. Before creating a release tag
3. Run all E2E tests in Phase B
4. Verify all critical user flows pass before proceeding to release

E2E tests are **NOT** executed automatically in CI/CD during PR phase (only unit tests run automatically).
