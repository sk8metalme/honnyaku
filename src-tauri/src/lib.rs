//! Honnyaku - AI Translation Desktop App
//!
//! This library provides the core functionality for the Honnyaku translation application.

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod llm;
pub mod services;

use services::clipboard::{ClipboardContent, ClipboardError};
use services::permissions::PermissionStatus;
use services::settings::{AppSettings, SettingsError};
use services::shortcut::{self, ShortcutError, ShortcutStatus};
use services::translation::{
    self, Language, ProviderStatus, ReplyResult, SummarizeResult, TranslationError,
    TranslationResult,
};

/// Greet command for testing IPC
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Honnyaku.", name)
}

// ============================================================================
// 設定管理コマンド
// ============================================================================

/// 設定を取得する
///
/// tauri-plugin-storeから設定を読み込み、存在しない場合はデフォルト値を返す
#[tauri::command]
async fn get_settings(app: tauri::AppHandle) -> Result<AppSettings, SettingsError> {
    use tauri_plugin_store::StoreExt;

    let store = app
        .store("settings.json")
        .map_err(|e| SettingsError::LoadFailed(e.to_string()))?;

    let shortcut = store
        .get("shortcut")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().shortcut);

    let ollama_model = store
        .get("ollamaModel")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_model);

    let ollama_endpoint = store
        .get("ollamaEndpoint")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_endpoint);

    let provider = store
        .get("provider")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().provider);

    let claude_cli_path = store
        .get("claudeCliPath")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    Ok(AppSettings {
        shortcut,
        ollama_model,
        ollama_endpoint,
        provider,
        claude_cli_path,
    })
}

/// 設定を保存する
#[tauri::command]
async fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), SettingsError> {
    use tauri_plugin_store::StoreExt;

    let store = app
        .store("settings.json")
        .map_err(|e| SettingsError::SaveFailed(e.to_string()))?;

    store.set("shortcut", serde_json::json!(settings.shortcut));
    store.set("ollamaModel", serde_json::json!(settings.ollama_model));
    store.set(
        "ollamaEndpoint",
        serde_json::json!(settings.ollama_endpoint),
    );
    store.set("provider", serde_json::json!(settings.provider));
    store.set("claudeCliPath", serde_json::json!(settings.claude_cli_path));

    store
        .save()
        .map_err(|e| SettingsError::SaveFailed(e.to_string()))?;

    Ok(())
}

/// 設定をデフォルトにリセットする
#[tauri::command]
async fn reset_settings(app: tauri::AppHandle) -> Result<AppSettings, SettingsError> {
    let defaults = AppSettings::default();
    save_settings(app, defaults.clone()).await?;
    Ok(defaults)
}

// ============================================================================
// 翻訳コマンド
// ============================================================================

/// テキストをClaude CLIで翻訳する
///
/// 言語検出はフロントエンドで行い、翻訳元・翻訳先言語を指定して呼び出す
#[tauri::command]
async fn translate_with_claude_cli(
    app: tauri::AppHandle,
    text: String,
    source_lang: Language,
    target_lang: Language,
) -> Result<TranslationResult, TranslationError> {
    use tauri_plugin_store::StoreExt;

    // 設定からClaude CLIパスを取得
    let claude_cli_path = app.store("settings.json").ok().and_then(|store| {
        store
            .get("claudeCliPath")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    });

    llm::claude_cli::translate_with_claude_cli(
        &text,
        source_lang,
        target_lang,
        claude_cli_path.as_deref(),
    )
    .await
}

/// テキストを翻訳する
///
/// 言語検出はフロントエンドで行い、翻訳元・翻訳先言語を指定して呼び出す
#[tauri::command]
async fn translate(
    app: tauri::AppHandle,
    text: String,
    source_lang: Language,
    target_lang: Language,
) -> Result<TranslationResult, TranslationError> {
    use tauri_plugin_store::StoreExt;

    // 設定を取得
    let store = app
        .store("settings.json")
        .map_err(|e| TranslationError::ApiError(format!("設定の読み込みに失敗: {}", e)))?;

    let ollama_model = store
        .get("ollamaModel")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_model);

    let ollama_endpoint = store
        .get("ollamaEndpoint")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_endpoint);

    translation::translate_with_ollama(
        &text,
        source_lang,
        target_lang,
        &ollama_endpoint,
        &ollama_model,
    )
    .await
}

/// テキストをストリーミングモードで翻訳する
///
/// 翻訳結果を逐次イベントで配信し、リアルタイムに表示可能にする
#[tauri::command]
async fn translate_stream(
    app: tauri::AppHandle,
    text: String,
    source_lang: Language,
    target_lang: Language,
) -> Result<(), TranslationError> {
    use tauri_plugin_store::StoreExt;

    // 設定を取得
    let store = app
        .store("settings.json")
        .map_err(|e| TranslationError::ApiError(format!("設定の読み込みに失敗: {}", e)))?;

    let ollama_model = store
        .get("ollamaModel")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_model);

    let ollama_endpoint = store
        .get("ollamaEndpoint")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_endpoint);

    translation::translate_with_ollama_stream(
        &app,
        &text,
        source_lang,
        target_lang,
        &ollama_endpoint,
        &ollama_model,
    )
    .await
}

/// Ollamaの接続状態を確認する
#[tauri::command]
async fn check_provider_status(app: tauri::AppHandle) -> ProviderStatus {
    use tauri_plugin_store::StoreExt;

    let ollama_endpoint = app
        .store("settings.json")
        .ok()
        .and_then(|store| {
            store
                .get("ollamaEndpoint")
                .and_then(|v| v.as_str().map(|s| s.to_string()))
        })
        .unwrap_or_else(|| AppSettings::default().ollama_endpoint);

    translation::check_ollama_status(&ollama_endpoint).await
}

/// Ollamaモデルをプリロードする
///
/// アプリ起動時に呼び出してモデルをウォームアップし、
/// 初回翻訳時のレスポンス時間を短縮する
#[tauri::command]
async fn preload_ollama_model(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_store::StoreExt;

    let store = app.store("settings.json").map_err(|e| e.to_string())?;

    let ollama_endpoint = store
        .get("ollamaEndpoint")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_endpoint);

    let ollama_model = store
        .get("ollamaModel")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_model);

    translation::preload_ollama_model(&ollama_endpoint, &ollama_model).await
}

/// テキストを要約する
///
/// 翻訳後のテキストを簡潔に要約し、翻訳先言語で結果を返す
#[tauri::command]
async fn summarize(
    app: tauri::AppHandle,
    text: String,
    language: Language,
) -> Result<SummarizeResult, TranslationError> {
    use tauri_plugin_store::StoreExt;

    // 設定を取得
    let store = app
        .store("settings.json")
        .map_err(|e| TranslationError::ApiError(format!("設定の読み込みに失敗: {}", e)))?;

    let ollama_model = store
        .get("ollamaModel")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_model);

    let ollama_endpoint = store
        .get("ollamaEndpoint")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_endpoint);

    translation::summarize_with_ollama(&text, language, &ollama_endpoint, &ollama_model).await
}

/// 返信を生成する
///
/// 元の文章に対するビジネス向けの丁寧な返信を翻訳先言語で生成し、
/// 翻訳元言語での説明も付与する
///
/// - language: 返信を作成する言語（翻訳先言語）
/// - source_language: 説明を作成する言語（翻訳元言語）
#[tauri::command]
async fn generate_reply(
    app: tauri::AppHandle,
    original_text: String,
    language: Language,
    source_language: Language,
) -> Result<ReplyResult, TranslationError> {
    use tauri_plugin_store::StoreExt;

    // 設定を取得
    let store = app
        .store("settings.json")
        .map_err(|e| TranslationError::ApiError(format!("設定の読み込みに失敗: {}", e)))?;

    let ollama_model = store
        .get("ollamaModel")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_model);

    let ollama_endpoint = store
        .get("ollamaEndpoint")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| AppSettings::default().ollama_endpoint);

    translation::generate_reply_with_ollama(
        &original_text,
        language,
        source_language,
        &ollama_endpoint,
        &ollama_model,
    )
    .await
}

// ============================================================================
// ショートカットコマンド
// ============================================================================

/// ショートカット文字列を検証する
#[tauri::command]
fn validate_shortcut_format(shortcut_str: String) -> Result<(), ShortcutError> {
    shortcut::validate_shortcut(&shortcut_str)
}

/// 現在のショートカット登録状態を取得する
#[tauri::command]
fn get_shortcut_status(app: tauri::AppHandle) -> ShortcutStatus {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    // 設定からショートカットを取得
    let shortcut_str = get_current_shortcut_from_settings(&app);

    // 登録状態を確認
    let is_registered = if let Some(ref s) = shortcut_str {
        app.global_shortcut().is_registered(s.as_str())
    } else {
        false
    };

    ShortcutStatus {
        current_shortcut: shortcut_str,
        is_registered,
    }
}

/// 設定からショートカット文字列を取得するヘルパー関数
fn get_current_shortcut_from_settings(app: &tauri::AppHandle) -> Option<String> {
    use tauri_plugin_store::StoreExt;

    app.store("settings.json").ok().and_then(|store| {
        store
            .get("shortcut")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    })
}

/// グローバルショートカットを登録する
#[tauri::command]
async fn register_shortcut(
    app: tauri::AppHandle,
    shortcut_str: String,
) -> Result<(), ShortcutError> {
    use tauri::Emitter;
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

    // ショートカット文字列を検証
    shortcut::validate_shortcut(&shortcut_str)?;

    // 既に登録されているか確認
    if app.global_shortcut().is_registered(shortcut_str.as_str()) {
        return Ok(()); // 既に登録済みなら成功扱い
    }

    // ショートカットを登録
    app.global_shortcut()
        .on_shortcut(shortcut_str.as_str(), move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                // ショートカットが押されたらフロントエンドにイベントを発行
                let _ = _app.emit("shortcut-triggered", ());
            }
        })
        .map_err(|e| ShortcutError::RegistrationFailed(e.to_string()))?;

    Ok(())
}

/// グローバルショートカットを解除する
#[tauri::command]
async fn unregister_shortcut(
    app: tauri::AppHandle,
    shortcut_str: String,
) -> Result<(), ShortcutError> {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    // 登録されていなければ成功扱い
    if !app.global_shortcut().is_registered(shortcut_str.as_str()) {
        return Ok(());
    }

    // ショートカットを解除
    app.global_shortcut()
        .unregister(shortcut_str.as_str())
        .map_err(|e| ShortcutError::UnregistrationFailed(e.to_string()))?;

    Ok(())
}

/// すべてのグローバルショートカットを解除する
#[tauri::command]
async fn unregister_all_shortcuts(app: tauri::AppHandle) -> Result<(), ShortcutError> {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    app.global_shortcut()
        .unregister_all()
        .map_err(|e| ShortcutError::UnregistrationFailed(e.to_string()))?;

    Ok(())
}

// ============================================================================
// 権限管理コマンド
// ============================================================================

/// アクセシビリティ権限の状態を確認する
#[tauri::command]
async fn check_accessibility_permission_status() -> PermissionStatus {
    // ネイティブAPIを使用して権限を確認（最も信頼できる）
    #[cfg(target_os = "macos")]
    let is_granted = check_accessibility_native();

    #[cfg(not(target_os = "macos"))]
    let is_granted = false;

    PermissionStatus {
        accessibility_granted: is_granted,
        needs_permission_request: !is_granted,
    }
}

/// アクセシビリティ権限をリクエストする
///
/// システム環境設定のアクセシビリティページを開く
#[tauri::command]
async fn request_accessibility_permission_prompt() -> PermissionStatus {
    // ネイティブAPIを使用して現在の権限状態を確認
    #[cfg(target_os = "macos")]
    let is_granted = check_accessibility_native();

    #[cfg(not(target_os = "macos"))]
    let is_granted = false;

    if !is_granted {
        // 権限が付与されていない場合は、システムダイアログを表示
        tauri_plugin_macos_permissions::request_accessibility_permission().await;
    }

    // 再度ネイティブAPIで権限状態を確認（ダイアログ表示後は即座に反映されないことがある）
    #[cfg(target_os = "macos")]
    let is_granted_after = check_accessibility_native();

    #[cfg(not(target_os = "macos"))]
    let is_granted_after = false;

    PermissionStatus {
        accessibility_granted: is_granted_after,
        needs_permission_request: !is_granted_after,
    }
}

/// macOSネイティブAPIを使用してアクセシビリティ権限を直接確認する
#[cfg(target_os = "macos")]
fn check_accessibility_native() -> bool {
    use objc::runtime::{Class, Object};
    use objc::{msg_send, sel, sel_impl};

    unsafe {
        // まず基本的なチェックを実行
        let result = AXIsProcessTrusted();
        let is_granted = result != 0;

        // デバッグログを出力
        eprintln!("[DEBUG] AXIsProcessTrusted result: {}, is_granted: {}", result, is_granted);

        // バンドル情報も出力
        let ns_bundle_class = Class::get("NSBundle").expect("NSBundle class not found");
        let main_bundle: *mut Object = msg_send![ns_bundle_class, mainBundle];
        let bundle_id: *mut Object = msg_send![main_bundle, bundleIdentifier];

        if !bundle_id.is_null() {
            let utf8: *const i8 = msg_send![bundle_id, UTF8String];
            if !utf8.is_null() {
                let bundle_id_str = std::ffi::CStr::from_ptr(utf8).to_string_lossy();
                eprintln!("[DEBUG] Bundle ID: {}", bundle_id_str);
            }
        }

        is_granted
    }
}

#[cfg(target_os = "macos")]
#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    /// アクセシビリティ権限がプロセスに付与されているかを確認
    /// 戻り値: 0 = 未許可, 非ゼロ = 許可済み
    fn AXIsProcessTrusted() -> u8;
}

/// アクセシビリティ権限が付与されているか確認する
#[tauri::command]
fn is_accessibility_granted() -> bool {
    // ネイティブAPIの結果を取得（macOSの場合）
    #[cfg(target_os = "macos")]
    {
        let result = check_accessibility_native();
        eprintln!("[DEBUG] is_accessibility_granted called, result: {}", result);
        result
    }

    // macOS以外の場合はfalseを返す
    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}

/// アクセシビリティ権限のデバッグ情報を取得
#[tauri::command]
fn get_accessibility_debug_info() -> String {
    #[cfg(target_os = "macos")]
    {
        use objc::runtime::{Class, Object};
        use objc::{msg_send, sel, sel_impl};

        unsafe {
            let result = AXIsProcessTrusted();
            let is_granted = result != 0;

            let ns_bundle_class = Class::get("NSBundle").unwrap();
            let main_bundle: *mut Object = msg_send![ns_bundle_class, mainBundle];
            let bundle_id: *mut Object = msg_send![main_bundle, bundleIdentifier];
            let bundle_path: *mut Object = msg_send![main_bundle, bundlePath];

            let mut debug_info = format!(
                "AXIsProcessTrusted result: {}\nIs Granted: {}\n",
                result, is_granted
            );

            if !bundle_id.is_null() {
                let utf8: *const i8 = msg_send![bundle_id, UTF8String];
                if !utf8.is_null() {
                    let bundle_id_str = std::ffi::CStr::from_ptr(utf8).to_string_lossy();
                    debug_info.push_str(&format!("Bundle ID: {}\n", bundle_id_str));
                }
            }

            if !bundle_path.is_null() {
                let utf8: *const i8 = msg_send![bundle_path, UTF8String];
                if !utf8.is_null() {
                    let bundle_path_str = std::ffi::CStr::from_ptr(utf8).to_string_lossy();
                    debug_info.push_str(&format!("Bundle Path: {}\n", bundle_path_str));
                }
            }

            // 実行ファイルのパスも取得
            if let Ok(exe_path) = std::env::current_exe() {
                debug_info.push_str(&format!("Executable Path: {:?}\n", exe_path));
            }

            debug_info
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        "Not running on macOS".to_string()
    }
}

// ============================================================================
// ウィンドウ管理コマンド
// ============================================================================

/// マウスカーソルの現在位置を取得する (macOS)
#[tauri::command]
#[allow(deprecated)]
#[allow(unexpected_cfgs)]
async fn get_cursor_position() -> Result<(f64, f64), String> {
    #[cfg(target_os = "macos")]
    {
        use cocoa::foundation::NSPoint;

        unsafe {
            // mouseLocationはNSEventのクラスメソッド
            let ns_event_class = class!(NSEvent);
            let mouse_location: NSPoint = msg_send![ns_event_class, mouseLocation];
            // macOSの座標系は左下が原点なので、そのまま返す
            Ok((mouse_location.x, mouse_location.y))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("This function is only available on macOS".to_string())
    }
}

// ============================================================================
// クリップボードコマンド
// ============================================================================

/// クリップボードからテキストを読み取る
#[tauri::command]
async fn read_clipboard(app: tauri::AppHandle) -> Result<ClipboardContent, ClipboardError> {
    use tauri_plugin_clipboard_manager::ClipboardExt;

    match app.clipboard().read_text() {
        Ok(text) => {
            if text.is_empty() {
                Ok(ClipboardContent::empty())
            } else {
                Ok(ClipboardContent::from_text(text))
            }
        }
        Err(e) => Err(ClipboardError::ReadFailed(e.to_string())),
    }
}

/// クリップボードにテキストを書き込む
#[tauri::command]
async fn write_clipboard(app: tauri::AppHandle, text: String) -> Result<(), ClipboardError> {
    use tauri_plugin_clipboard_manager::ClipboardExt;

    app.clipboard()
        .write_text(text)
        .map_err(|e| ClipboardError::WriteFailed(e.to_string()))
}

/// 選択テキストを取得する（Cmd+Cを送信してクリップボードから読み取る）
///
/// 注: この機能にはアクセシビリティ権限が必要
#[tauri::command]
async fn get_selected_text(app: tauri::AppHandle) -> Result<ClipboardContent, ClipboardError> {
    use tauri_plugin_clipboard_manager::ClipboardExt;

    // 元のクリップボード内容を保存
    let original_content = app.clipboard().read_text().ok();

    // Cmd+Cキーストロークを送信
    // 注: Tauri v2ではキーストローク送信のためにシステムAPIを直接使用する必要がある
    // ここではAppleScriptを使用
    let script_result = std::process::Command::new("osascript")
        .arg("-e")
        .arg(
            r#"tell application "System Events"
                keystroke "c" using command down
            end tell"#,
        )
        .output();

    if script_result.is_err() {
        // 元のクリップボード内容を復元
        if let Some(original) = original_content {
            let _ = app.clipboard().write_text(original);
        }
        return Err(ClipboardError::ReadFailed(
            "キーストローク送信に失敗しました".to_string(),
        ));
    }

    // クリップボード更新を待機し、内容が変更されたか確認
    let mut selected_text = String::new();
    let mut retries = 0;
    let max_retries = 5; // 最大5回リトライ（合計500ms）

    while retries < max_retries {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        match app.clipboard().read_text() {
            Ok(text) => {
                // クリップボードの内容が元の内容と異なる場合、コピー成功
                if Some(&text) != original_content.as_ref() && !text.is_empty() {
                    selected_text = text;
                    break;
                }
            }
            Err(_) => {
                // エラーの場合はリトライ
            }
        }

        retries += 1;
    }

    // リトライ上限に達した場合、元のクリップボード内容と同じままの場合
    if selected_text.is_empty() || Some(&selected_text) == original_content.as_ref() {
        // 元のクリップボード内容を復元
        if let Some(original) = original_content {
            let _ = app.clipboard().write_text(original);
        }
        return Err(ClipboardError::ReadFailed(
            "選択テキストの取得に失敗しました。アクセシビリティ権限が許可されているか確認してください。"
                .to_string(),
        ));
    }

    // 元のクリップボード内容を復元
    if let Some(original) = original_content {
        // 少し待ってから復元
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        let _ = app.clipboard().write_text(original);
    }

    if selected_text.is_empty() {
        Ok(ClipboardContent::empty())
    } else {
        Ok(ClipboardContent::from_text(selected_text))
    }
}

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_macos_permissions::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_settings,
            save_settings,
            reset_settings,
            translate,
            translate_with_claude_cli,
            translate_stream,
            check_provider_status,
            preload_ollama_model,
            summarize,
            generate_reply,
            validate_shortcut_format,
            get_shortcut_status,
            register_shortcut,
            unregister_shortcut,
            unregister_all_shortcuts,
            check_accessibility_permission_status,
            request_accessibility_permission_prompt,
            is_accessibility_granted,
            get_accessibility_debug_info,
            read_clipboard,
            write_clipboard,
            get_selected_text,
            get_cursor_position,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
