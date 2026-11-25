/** 语言偏好 → 已解析 locale（纯函数，供 AppEnv 与单测）。 */

import { AppLocale, type LocalePreference } from '../types/appSettings';

/** 系统标签（如 `zh-CN`）→ 应用语言；非中/英回落英文。 */
export function localeFromSystemTag(tag: string): AppLocale {
  const lower = tag.toLowerCase();
  if (lower.startsWith('zh')) {
    return AppLocale.Zh;
  }
  return AppLocale.En;
}

/** 按偏好解析界面语言。 */
export function resolveLocale(preference: LocalePreference, systemTag: string): AppLocale {
  if (preference === 'zh') {
    return AppLocale.Zh;
  }
  if (preference === 'en') {
    return AppLocale.En;
  }
  return localeFromSystemTag(systemTag);
}
