import { invoke } from '@tauri-apps/api/core';
import type { AppSettings } from '../../types/appSettings';

/** 读取应用设置；Rust 侧缺字段与默认合并并写回完整文件 */
export async function loadAppSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('load_app_settings');
}

/** 将应用设置写入应用配置目录 */
export async function saveAppSettings(settings: AppSettings): Promise<void> {
  await invoke('save_app_settings', { settings });
}

/** 内置默认设置（不读写磁盘） */
export async function defaultAppSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('default_app_settings');
}
