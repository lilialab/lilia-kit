import { invoke } from '@tauri-apps/api/core';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { listen } from '@tauri-apps/api/event';

const EVENT_MONITOR_TOGGLE = 'monitor-toggle';

/** 监听全局快捷键：仅通知，显隐由页面自行决定 */
export function onMonitorToggle(handler: () => void): Promise<UnlistenFn> {
  return listen(EVENT_MONITOR_TOGGLE, () => {
    handler();
  });
}

/** 请求监视窗 OS 显隐（show 可补建；hide 仅隐藏） */
export function setMonitorWindowVisible(visible: boolean): Promise<void> {
  return invoke('set_monitor_window_visible', { visible });
}
