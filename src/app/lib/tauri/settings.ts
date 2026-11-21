import { invoke } from '@tauri-apps/api/core';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { listen } from '@tauri-apps/api/event';
import type { AppSettings } from '../../types/appSettings';

/** 读取应用设置；Rust 侧缺字段与默认合并并写回完整文件 */
export async function loadAppSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('load_app_settings');
}

/** 保存应用设置；返回 Rust apply/写盘后的完整 settings */
export async function saveAppSettings(settings: AppSettings): Promise<AppSettings> {
  return invoke<AppSettings>('save_app_settings', { settings });
}

/** 内置默认设置（不读写磁盘） */
export async function defaultAppSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('default_app_settings');
}

/** 订阅设置变更（完整 AppSettings） */
export async function onAppSettingsChanged(
  handler: (settings: AppSettings) => void,
): Promise<UnlistenFn> {
  return listen<AppSettings>('app-settings-changed', (event) => {
    handler(event.payload);
  });
}
