import { invoke } from '@tauri-apps/api/core';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { listen } from '@tauri-apps/api/event';
import type { AppLocale, AppThemeName } from '../../types/appSettings';

/** Rust 解析后的壳层快照（主题 + 语言） */
export interface ResolvedAppEnv {
  theme: AppThemeName;
  locale: AppLocale;
}

/** 当前已保存偏好下的 resolved 快照（启动对齐） */
export async function getResolvedAppEnv(): Promise<ResolvedAppEnv> {
  return invoke<ResolvedAppEnv>('get_resolved_app_env');
}

/** 订阅 resolved 壳层变更（settings apply / OS 主题） */
export async function onAppEnvChanged(
  handler: (env: ResolvedAppEnv) => void,
): Promise<UnlistenFn> {
  return listen<ResolvedAppEnv>('app-env-changed', (event) => {
    handler(event.payload);
  });
}
