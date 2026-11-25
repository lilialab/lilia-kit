/** 热键 accel 预设与平台可读文案（无 Angular 依赖，供组件与单测共用）。 */

import { AppPlatform } from '../types/appSettings';

/** 监视器显隐等可复用的候选（不含空；空 =「无」） */
export const HOTKEY_PRESETS = [
  'Control+Alt+M',
  'Control+Shift+M',
  'Alt+Shift+M',
  'Control+Shift+Period',
] as const;

export type HotkeyPresetId = (typeof HOTKEY_PRESETS)[number];

/** 监视器显隐默认 accel（与 `default_settings.json` 一致） */
export const DEFAULT_MONITOR_VISIBILITY_HOTKEY: HotkeyPresetId = 'Control+Alt+M';

/** 平台可读文案；空 accel 返回空串 */
export function formatAccelDisplay(accel: string, platform: AppPlatform): string {
  if (!accel) {
    return '';
  }
  return accel
    .split('+')
    .filter(Boolean)
    .map((part) => formatAccelPart(part, platform))
    .join(platform === AppPlatform.Mac ? '' : '+');
}

function formatAccelPart(part: string, platform: AppPlatform): string {
  if (platform === AppPlatform.Mac) {
    switch (part) {
      case 'Super':
        return '⌘';
      case 'Alt':
        return '⌥';
      case 'Control':
        return '⌃';
      case 'Shift':
        return '⇧';
      case 'Period':
        return '.';
      default:
        return part.length === 1 ? part.toUpperCase() : part;
    }
  }

  switch (part) {
    case 'Super':
      return 'Win';
    case 'Control':
      return 'Ctrl';
    case 'Period':
      return '.';
    default:
      return part.length === 1 ? part.toUpperCase() : part;
  }
}
