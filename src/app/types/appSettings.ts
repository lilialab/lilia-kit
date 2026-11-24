/** 真正工具 id（启用/热键等；不含「通用」设置分组） */
export type ToolId = 'monitor' | 'screenshot' | 'clipboard';

/** 设置侧栏分组（含通用） */
export type SettingsSectionId = 'general' | ToolId;

/** 运行平台（dataset / 热键文案） */
export const AppPlatform = {
  Win: 'win',
  Mac: 'mac',
} as const;
export type AppPlatform = (typeof AppPlatform)[keyof typeof AppPlatform];

/** Catppuccin 主题名（与 `data-theme` 一致） */
export const AppTheme = {
  Latte: 'latte',
  Mocha: 'mocha',
} as const;
export type AppThemeName = (typeof AppTheme)[keyof typeof AppTheme];

/** 外观偏好：跟随系统，或固定某一主题 */
export const Appearance = {
  System: 'system',
  Latte: AppTheme.Latte,
  Mocha: AppTheme.Mocha,
} as const;
export type AppearancePreference = (typeof Appearance)[keyof typeof Appearance];

/** 通用设置分组 */
export interface GeneralSettings {
  /** 跟随系统 / Latte / Mocha */
  appearance: AppearancePreference;
}

/** 监视器工具偏好 */
export interface MonitorSettings {
  /** 是否启用（保存后装载/卸载浮窗） */
  enabled: boolean;
  /** 是否置顶 */
  pinned: boolean;
  /** 是否显示温度 */
  showTemperature: boolean;
  /** 显隐全局快捷键 accel；空字符串 = 未绑定 */
  visibilityHotkey: string;
}

/** 应用偏好设置（与 Rust `AppSettings` / settings.json 对齐，按分组） */
export interface AppSettings {
  general: GeneralSettings;
  monitor: MonitorSettings;
}
