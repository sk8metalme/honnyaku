/**
 * usePermissions Hook テスト
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, act, waitFor } from '@testing-library/react';
import { usePermissions } from '../usePermissions';

// Tauri APIのモック
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

const mockInvoke = vi.mocked(invoke);

describe('usePermissions', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // デフォルトのモック設定
    mockInvoke.mockResolvedValue(false);
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('初期状態で権限確認が行われる', async () => {
    mockInvoke.mockResolvedValue(false);

    const { result } = renderHook(() => usePermissions());

    await waitFor(() => {
      expect(result.current.isChecking).toBe(false);
    });

    expect(mockInvoke).toHaveBeenCalledWith('is_accessibility_granted');
    expect(result.current.isAccessibilityGranted).toBe(false);
  });

  it('権限が付与されている場合はtrueを返す', async () => {
    mockInvoke.mockResolvedValue(true);

    const { result } = renderHook(() => usePermissions());

    await waitFor(() => {
      expect(result.current.isChecking).toBe(false);
    });

    expect(result.current.isAccessibilityGranted).toBe(true);
  });

  it('checkAccessibilityで権限状態を再確認できる', async () => {
    mockInvoke.mockResolvedValueOnce(false).mockResolvedValueOnce(true);

    const { result } = renderHook(() => usePermissions());

    await waitFor(() => {
      expect(result.current.isChecking).toBe(false);
    });

    expect(result.current.isAccessibilityGranted).toBe(false);

    await act(async () => {
      await result.current.checkAccessibility();
    });

    expect(result.current.isAccessibilityGranted).toBe(true);
    expect(mockInvoke).toHaveBeenCalledTimes(2);
  });

  it('requestAccessibilityで権限をリクエストできる', async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === 'is_accessibility_granted') {
        return false;
      }
      if (cmd === 'request_accessibility_permission_prompt') {
        return {
          accessibilityGranted: true,
          needsPermissionRequest: false,
        };
      }
      return undefined;
    });

    const { result } = renderHook(() => usePermissions());

    await waitFor(() => {
      expect(result.current.isChecking).toBe(false);
    });

    expect(result.current.isAccessibilityGranted).toBe(false);

    await act(async () => {
      await result.current.requestAccessibility();
    });

    expect(mockInvoke).toHaveBeenCalledWith(
      'request_accessibility_permission_prompt'
    );
    expect(result.current.isAccessibilityGranted).toBe(true);
  });

  it('権限確認エラー時にエラーメッセージが設定される', async () => {
    mockInvoke.mockRejectedValue(new Error('権限確認に失敗しました'));

    const { result } = renderHook(() => usePermissions());

    await waitFor(() => {
      expect(result.current.isChecking).toBe(false);
    });

    expect(result.current.error).toBe('権限確認に失敗しました');
  });

  it('権限リクエストエラー時にエラーメッセージが設定される', async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === 'is_accessibility_granted') {
        return false;
      }
      if (cmd === 'request_accessibility_permission_prompt') {
        throw new Error('権限リクエストに失敗しました');
      }
      return undefined;
    });

    const { result } = renderHook(() => usePermissions());

    await waitFor(() => {
      expect(result.current.isChecking).toBe(false);
    });

    await act(async () => {
      await result.current.requestAccessibility();
    });

    expect(result.current.error).toBe('権限リクエストに失敗しました');
  });

  it('isCheckingが処理中に正しく設定される', async () => {
    let resolvePromise: (value: boolean) => void;
    const promise = new Promise<boolean>((resolve) => {
      resolvePromise = resolve;
    });

    mockInvoke.mockReturnValue(promise);

    const { result } = renderHook(() => usePermissions());

    // isCheckingがtrueであることを確認
    expect(result.current.isChecking).toBe(true);

    // 処理を完了
    await act(async () => {
      resolvePromise!(true);
      await promise;
    });

    // isCheckingがfalseに戻ることを確認
    await waitFor(() => {
      expect(result.current.isChecking).toBe(false);
    });
  });
});
