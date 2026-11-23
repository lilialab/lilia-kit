import { invoke } from '@tauri-apps/api/core';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { listen } from '@tauri-apps/api/event';
import type { AppThemeName } from '../../types/appSettings';

/** 当前已保存偏好下 Rust 解析的主题名（启动对齐） */
export async function getResolvedAppTheme(): Promise<AppThemeName> {
  return invoke<AppThemeName>('get_resolved_app_theme');
}

/** 订阅 resolved 主题变更（保存 / OS 跟随系统时） */
export async function onAppThemeChanged(
  handler: (theme: AppThemeName) => void,
): Promise<UnlistenFn> {
  return listen<AppThemeName>('app-theme-changed', (event) => {
    handler(event.payload);
  });
}
