/**
 * useShortcut Hook テスト
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, act, waitFor } from '@testing-library/react';
import { useShortcut } from '../useShortcut';

// Tauri APIのモック
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const mockInvoke = vi.mocked(invoke);
const mockListen = vi.mocked(listen);

describe('useShortcut', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    // デフォルトのモック設定
    mockListen.mockResolvedValue(() => {});
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === 'get_shortcut_status') {
        return {
          currentShortcut: null,
          isRegistered: false,
        };
      }
      return undefined;
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('初期状態が正しく設定される', async () => {
    const { result } = renderHook(() => useShortcut());

    await waitFor(() => {
      expect(result.current.isRegistered).toBe(false);
    });

    expect(result.current.currentShortcut).toBe(null);
    expect(result.current.isLoading).toBe(false);
    expect(result.current.error).toBe(null);
  });

  it('既に登録されている場合は状態が反映される', async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === 'get_shortcut_status') {
        return {
          currentShortcut: 'CommandOrControl+Shift+T',
          isRegistered: true,
        };
      }
      return undefined;
    });

    const { result } = renderHook(() => useShortcut());

    await waitFor(() => {
      expect(result.current.isRegistered).toBe(true);
    });

    expect(result.current.currentShortcut).toBe('CommandOrControl+Shift+T');
  });

  it('registerShortcutでショートカットを登録できる', async () => {
    mockInvoke.mockResolvedValue(undefined);

    const { result } = renderHook(() => useShortcut());

    await act(async () => {
      await result.current.registerShortcut('CommandOrControl+Shift+T');
    });

    expect(mockInvoke).toHaveBeenCalledWith('register_shortcut', {
      shortcutStr: 'CommandOrControl+Shift+T',
    });
    expect(result.current.isRegistered).toBe(true);
    expect(result.current.currentShortcut).toBe('CommandOrControl+Shift+T');
  });

  it('unregisterShortcutでショートカットを解除できる', async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === 'get_shortcut_status') {
        return {
          currentShortcut: 'CommandOrControl+Shift+T',
          isRegistered: true,
        };
      }
      return undefined;
    });

    const { result } = renderHook(() => useShortcut());

    await waitFor(() => {
      expect(result.current.isRegistered).toBe(true);
    });

    await act(async () => {
      await result.current.unregisterShortcut('CommandOrControl+Shift+T');
    });

    expect(mockInvoke).toHaveBeenCalledWith('unregister_shortcut', {
      shortcutStr: 'CommandOrControl+Shift+T',
    });
    expect(result.current.isRegistered).toBe(false);
    expect(result.current.currentShortcut).toBe(null);
  });

  it('unregisterAllで全てのショートカットを解除できる', async () => {
    const { result } = renderHook(() => useShortcut());

    await act(async () => {
      await result.current.unregisterAll();
    });

    expect(mockInvoke).toHaveBeenCalledWith('unregister_all_shortcuts');
    expect(result.current.isRegistered).toBe(false);
    expect(result.current.currentShortcut).toBe(null);
  });

  it('validateShortcutで有効なショートカットを検証できる', async () => {
    mockInvoke.mockResolvedValue(undefined);

    const { result } = renderHook(() => useShortcut());

    let isValid = false;
    await act(async () => {
      isValid = await result.current.validateShortcut('CommandOrControl+Shift+T');
    });

    expect(isValid).toBe(true);
    expect(mockInvoke).toHaveBeenCalledWith('validate_shortcut_format', {
      shortcutStr: 'CommandOrControl+Shift+T',
    });
  });

  it('validateShortcutで無効なショートカットを検出できる', async () => {
    mockInvoke.mockRejectedValue(new Error('Invalid format'));

    const { result } = renderHook(() => useShortcut());

    let isValid = true;
    await act(async () => {
      isValid = await result.current.validateShortcut('InvalidShortcut+');
    });

    expect(isValid).toBe(false);
  });

  it('登録エラー時にエラーメッセージが設定される', async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === 'get_shortcut_status') {
        return {
          currentShortcut: null,
          isRegistered: false,
        };
      }
      if (cmd === 'register_shortcut') {
        throw new Error('ショートカットの登録に失敗しました');
      }
      return undefined;
    });

    const { result } = renderHook(() => useShortcut());

    await act(async () => {
      try {
        await result.current.registerShortcut('InvalidShortcut');
      } catch {
        // エラーは期待通り
      }
    });

    expect(result.current.error).toBe('ショートカットの登録に失敗しました');
    expect(result.current.isRegistered).toBe(false);
  });

  it('ショートカットイベントリスナーが設定される', async () => {
    const callback = vi.fn();

    renderHook(() => useShortcut(callback));

    expect(mockListen).toHaveBeenCalledWith(
      'shortcut-triggered',
      expect.any(Function)
    );
  });

  it('コールバックが呼ばれたときにonShortcutTriggeredが実行される', async () => {
    const callback = vi.fn();
    let capturedCallback: ((event: unknown) => void) | null = null;

    mockListen.mockImplementation(async (_eventName, cb) => {
      capturedCallback = cb as (event: unknown) => void;
      return () => {};
    });

    renderHook(() => useShortcut(callback));

    await waitFor(() => {
      expect(capturedCallback).not.toBeNull();
    });

    // イベントをシミュレート
    act(() => {
      capturedCallback?.({});
    });

    expect(callback).toHaveBeenCalled();
  });

  it('isLoadingが処理中に正しく設定される', async () => {
    let resolvePromise: () => void;
    const promise = new Promise<void>((resolve) => {
      resolvePromise = resolve;
    });

    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === 'get_shortcut_status') {
        return {
          currentShortcut: null,
          isRegistered: false,
        };
      }
      if (cmd === 'register_shortcut') {
        await promise;
        return undefined;
      }
      return undefined;
    });

    const { result } = renderHook(() => useShortcut());

    // 登録を開始
    let registerPromise: Promise<void>;
    act(() => {
      registerPromise = result.current.registerShortcut('CommandOrControl+Shift+T');
    });

    // isLoadingがtrueになることを確認
    expect(result.current.isLoading).toBe(true);

    // 処理を完了
    await act(async () => {
      resolvePromise!();
      await registerPromise;
    });

    // isLoadingがfalseに戻ることを確認
    expect(result.current.isLoading).toBe(false);
  });
});
