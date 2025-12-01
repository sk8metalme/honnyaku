/**
 * Hooks エクスポート
 */

export { useSettings } from './useSettings';
export type { UseSettingsReturn } from './useSettings';

export { useClipboard } from './useClipboard';
export type { ClipboardContent, UseClipboardReturn } from './useClipboard';

export { useShortcut } from './useShortcut';
export type { ShortcutStatus, UseShortcutReturn } from './useShortcut';

export { usePermissions } from './usePermissions';
export type { PermissionStatus, UsePermissionsReturn } from './usePermissions';

export { useTranslation } from './useTranslation';
export type { UseTranslationReturn } from './useTranslation';

export { useTranslationFlow } from './useTranslationFlow';
export type {
  TranslationFlowState,
  TranslationFlowErrorType,
  TranslationFlowError,
  UseTranslationFlowReturn,
} from './useTranslationFlow';
