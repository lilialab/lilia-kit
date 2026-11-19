import { invoke } from '@tauri-apps/api/core';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { listen } from '@tauri-apps/api/event';

export interface ToolsState {
  monitorEnabled: boolean;
  /** 截图是否已实现；目前恒为 false */
  screenshotAvailable: boolean;
  /** 剪切板历史是否已实现；目前恒为 false */
  clipboardAvailable: boolean;
}

/** 读取工具开关快照 */
export async function getToolsState(): Promise<ToolsState> {
  return invoke<ToolsState>('get_tools_state');
}

/** 开关监视器浮窗 */
export async function setMonitorEnabled(enabled: boolean): Promise<void> {
  await invoke('set_monitor_enabled', { enabled });
}

/** 订阅工具开关变更（工具窗关闭回写等） */
export async function onToolsChanged(
  handler: (state: ToolsState) => void,
): Promise<UnlistenFn> {
  return listen<ToolsState>('tools-changed', (event) => {
    handler(event.payload);
  });
}
