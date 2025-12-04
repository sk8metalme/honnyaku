/**
 * 設定画面コンポーネント
 *
 * アプリケーションの各種設定を行うパネル
 * - ショートカットキーのカスタマイズ
 * - Ollamaモデル・エンドポイント設定
 * - 接続テスト
 */

import { useState, useCallback } from 'react';
import { useSettingsContext } from '@/contexts/SettingsContext';
import { usePermissions } from '@/hooks/usePermissions';
import { useShortcut } from '@/hooks/useShortcut';

/**
 * SettingsPanelのProps
 */
export interface SettingsPanelProps {
  /** パネルが開いているかどうか */
  isOpen: boolean;
  /** 閉じるボタンクリック時のコールバック */
  onClose: () => void;
}

/**
 * Ollamaモデルオプション
 */
const OLLAMA_MODEL_OPTIONS = [
  {
    value: 'qwen2.5:0.5b',
    label: 'qwen2.5:0.5b (最速・395MB)',
    description: '最速だが品質は低め',
  },
  {
    value: 'qwen2.5:1.5b',
    label: 'qwen2.5:1.5b (高速・986MB)',
    description: '速度と品質のバランス',
  },
  {
    value: 'qwen2.5:3b',
    label: 'qwen2.5:3b (標準・1.9GB)',
    description: '標準的な品質',
  },
  {
    value: 'qwen2.5:7b',
    label: 'qwen2.5:7b (高品質・4.7GB)',
    description: '高品質だが遅め',
  },
  {
    value: 'mitmul/plamo-2-translate:Q4_K_M',
    label: 'PLaMo-2-Translate Q4 (翻訳特化・5.6GB)',
    description: '翻訳タスクに最適化された高品質モデル（バランス重視）',
  },
  {
    value: 'mitmul/plamo-2-translate:Q2_K_S',
    label: 'PLaMo-2-Translate Q2 (翻訳特化・3.5GB)',
    description: '翻訳タスクに最適化（高速動作優先）',
  },
];

/**
 * 入力フィールドコンポーネント
 */
function InputField({
  label,
  value,
  onChange,
  placeholder,
  disabled,
}: {
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
}) {
  return (
    <div>
      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        {label}
      </label>
      <input
        type="text"
        value={value}
        onChange={(e) => {
          onChange(e.target.value);
        }}
        placeholder={placeholder}
        disabled={disabled}
        className="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 transition"
      />
    </div>
  );
}

/**
 * セレクトフィールドコンポーネント
 */
function SelectField({
  label,
  value,
  onChange,
  options,
  disabled,
}: {
  label: string;
  value: string;
  onChange: (value: string) => void;
  options: { value: string; label: string; description?: string }[];
  disabled?: boolean;
}) {
  // 現在の値が選択肢に存在しない場合、一時的な選択肢として追加
  const isValueInOptions = options.some((opt) => opt.value === value);
  const displayOptions = isValueInOptions
    ? options
    : [
        {
          value,
          label: `${value} (非推奨 - 削除されたモデル)`,
          description:
            'このモデルは利用できません。別のモデルを選択してください。',
        },
        ...options,
      ];

  return (
    <div>
      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        {label}
      </label>
      <select
        value={value}
        onChange={(e) => {
          onChange(e.target.value);
        }}
        disabled={disabled}
        className="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 transition"
      >
        {displayOptions.map((opt) => (
          <option key={opt.value} value={opt.value}>
            {opt.label}
          </option>
        ))}
      </select>
      {(() => {
        const selected = displayOptions.find((o) => o.value === value);
        return selected?.description ? (
          <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
            {selected.description}
          </p>
        ) : null;
      })()}
    </div>
  );
}

/**
 * トグルスイッチコンポーネント（将来の拡張用に保持）
 */
export function ToggleSwitch({
  label,
  description,
  checked,
  onChange,
}: {
  label: string;
  description?: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
}) {
  return (
    <div className="flex items-center justify-between">
      <div>
        <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
          {label}
        </span>
        {description && (
          <p className="text-xs text-gray-500 dark:text-gray-400">
            {description}
          </p>
        )}
      </div>
      <button
        role="switch"
        aria-checked={checked}
        onClick={() => {
          onChange(!checked);
        }}
        className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
          checked ? 'bg-blue-500' : 'bg-gray-300 dark:bg-gray-600'
        }`}
      >
        <span
          className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
            checked ? 'translate-x-6' : 'translate-x-1'
          }`}
        />
      </button>
    </div>
  );
}

/**
 * 設定画面コンポーネント
 */
export function SettingsPanel({ isOpen, onClose }: SettingsPanelProps) {
  const { settings, updateSettings, checkProviderStatus } =
    useSettingsContext();

  const { isAccessibilityGranted, requestAccessibility } = usePermissions();

  const {
    isRegistered: shortcutRegistered,
    registerShortcut,
    unregisterShortcut,
    validateShortcut,
  } = useShortcut();

  // ローカル状態（settingsから初期値を取得）
  const [shortcutInput, setShortcutInput] = useState(() => settings.shortcut);
  const [ollamaModelInput, setOllamaModelInput] = useState(
    () => settings.ollamaModel
  );
  const [ollamaEndpointInput, setOllamaEndpointInput] = useState(
    () => settings.ollamaEndpoint
  );
  const [providerStatus, setProviderStatus] = useState<
    'checking' | 'available' | 'unavailable' | null
  >(null);
  const [statusMessage, setStatusMessage] = useState('');

  // ショートカット変更
  const handleShortcutChange = useCallback(async () => {
    if (!shortcutInput) return;

    const isValid = await validateShortcut(shortcutInput);
    if (!isValid) {
      setStatusMessage('無効なショートカット形式です');
      return;
    }

    try {
      // 既存のショートカットを解除
      if (settings.shortcut && shortcutRegistered) {
        await unregisterShortcut(settings.shortcut);
      }

      // 新しいショートカットを登録
      await registerShortcut(shortcutInput);

      // 設定を保存
      await updateSettings({ shortcut: shortcutInput });
      setStatusMessage('ショートカットを更新しました');
    } catch {
      setStatusMessage('ショートカットの登録に失敗しました');
    }
  }, [
    shortcutInput,
    settings,
    shortcutRegistered,
    validateShortcut,
    unregisterShortcut,
    registerShortcut,
    updateSettings,
  ]);

  // Ollama設定保存
  const handleSaveOllamaSettings = useCallback(async () => {
    try {
      await updateSettings({
        ollamaModel: ollamaModelInput,
        ollamaEndpoint: ollamaEndpointInput,
      });
      setStatusMessage('Ollama設定を保存しました');
    } catch {
      setStatusMessage('設定の保存に失敗しました');
    }
  }, [ollamaModelInput, ollamaEndpointInput, updateSettings]);

  // 接続テスト
  const handleTestConnection = useCallback(async () => {
    setProviderStatus('checking');
    setStatusMessage('接続を確認中...');

    const status = await checkProviderStatus();
    if (status.status === 'available') {
      setProviderStatus('available');
      setStatusMessage('接続成功');
    } else {
      setProviderStatus('unavailable');
      setStatusMessage(`接続失敗: ${status.reason}`);
    }
  }, [checkProviderStatus]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 flex items-center justify-center z-50">
      {/* オーバーレイ */}
      <div
        className="absolute inset-0 bg-black/20 backdrop-blur-sm"
        onClick={onClose}
      />

      {/* パネル本体 */}
      <div className="relative bg-white dark:bg-gray-800 rounded-2xl shadow-2xl w-full max-w-lg mx-4 max-h-[90vh] overflow-hidden flex flex-col">
        {/* ヘッダー */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-lg font-semibold text-gray-800 dark:text-white">
            設定
          </h2>
          <button
            onClick={onClose}
            className="p-1 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            aria-label="閉じる"
          >
            <svg
              className="w-5 h-5 text-gray-500"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        {/* コンテンツ */}
        <div className="flex-1 overflow-y-auto p-6 space-y-6">
          {/* ステータスメッセージ */}
          {statusMessage && (
            <div
              className={`p-3 rounded-lg text-sm ${
                statusMessage.includes('失敗') ||
                statusMessage.includes('エラー')
                  ? 'bg-red-50 dark:bg-red-900/30 text-red-600 dark:text-red-400'
                  : 'bg-green-50 dark:bg-green-900/30 text-green-600 dark:text-green-400'
              }`}
            >
              {statusMessage}
            </div>
          )}

          {/* アクセシビリティ権限 */}
          <section>
            <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-3">
              アクセシビリティ権限
            </h3>
            <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
              <div className="flex items-center gap-2">
                <span
                  className={`w-2 h-2 rounded-full ${
                    isAccessibilityGranted ? 'bg-green-500' : 'bg-red-500'
                  }`}
                />
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {isAccessibilityGranted ? '許可済み' : '許可されていません'}
                </span>
              </div>
              {!isAccessibilityGranted && (
                <button
                  onClick={() => {
                    void requestAccessibility();
                  }}
                  className="px-3 py-1 text-sm bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                >
                  許可をリクエスト
                </button>
              )}
            </div>
          </section>

          {/* ショートカット設定 */}
          <section>
            <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-3">
              ショートカットキー
            </h3>
            <div className="flex gap-2">
              <input
                type="text"
                value={shortcutInput}
                onChange={(e) => {
                  setShortcutInput(e.target.value);
                }}
                placeholder="例: CommandOrControl+Shift+T"
                className="flex-1 px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
              />
              <button
                onClick={() => {
                  void handleShortcutChange();
                }}
                className="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white text-sm rounded-lg transition-colors"
              >
                変更
              </button>
            </div>
            <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
              現在: {settings.shortcut}
              {shortcutRegistered && ' (登録済み)'}
            </p>
          </section>

          {/* Ollama設定 */}
          <section>
            <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-3">
              Ollama設定
            </h3>
            <div className="space-y-3">
              <InputField
                label="エンドポイント"
                value={ollamaEndpointInput}
                onChange={setOllamaEndpointInput}
                placeholder="http://localhost:11434"
              />
              <SelectField
                label="モデル"
                value={ollamaModelInput}
                onChange={setOllamaModelInput}
                options={OLLAMA_MODEL_OPTIONS}
              />
              <button
                onClick={() => {
                  void handleSaveOllamaSettings();
                }}
                className="w-full px-4 py-2 bg-gray-800 dark:bg-gray-600 hover:bg-gray-700 dark:hover:bg-gray-500 text-white text-sm rounded-lg transition-colors"
              >
                Ollama設定を保存
              </button>
            </div>
          </section>

          {/* 接続テスト */}
          <section>
            <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-3">
              接続テスト
            </h3>
            <button
              onClick={() => {
                void handleTestConnection();
              }}
              disabled={providerStatus === 'checking'}
              className="w-full px-4 py-2 bg-green-500 hover:bg-green-600 disabled:bg-gray-400 text-white text-sm rounded-lg transition-colors flex items-center justify-center gap-2"
            >
              {providerStatus === 'checking' ? (
                <>
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
                  確認中...
                </>
              ) : (
                '接続をテスト'
              )}
            </button>
            {providerStatus && providerStatus !== 'checking' && (
              <div
                className={`mt-2 flex items-center gap-2 ${
                  providerStatus === 'available'
                    ? 'text-green-500'
                    : 'text-red-500'
                }`}
              >
                <span
                  className={`w-2 h-2 rounded-full ${
                    providerStatus === 'available'
                      ? 'bg-green-500'
                      : 'bg-red-500'
                  }`}
                />
                <span className="text-sm">
                  {providerStatus === 'available' ? '接続成功' : '接続失敗'}
                </span>
              </div>
            )}
          </section>
        </div>
      </div>
    </div>
  );
}
