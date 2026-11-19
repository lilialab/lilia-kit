import { getCurrentWindow } from '@tauri-apps/api/window';

/** 当前 WebView 窗口 label */
export function getCurrentWindowLabel(): string {
  return getCurrentWindow().label;
}
