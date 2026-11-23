/** 真正工具 id（启用/热键等；不含「通用」设置分组） */
export type ToolId = 'monitor' | 'screenshot' | 'clipboard';

/** 设置侧栏分组（含通用） */
export type SettingsSectionId = 'general' | ToolId;

/** Catppuccin 主题名（与 `data-theme` 一致） */
export type AppThemeName = 'latte' | 'mocha';

/** 外观偏好：跟随系统，或固定某一主题 */
export type AppearancePreference = 'system' | AppThemeName;

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
}

/** 应用偏好设置（与 Rust `AppSettings` / settings.json 对齐，按分组） */
export interface AppSettings {
  general: GeneralSettings;
  monitor: MonitorSettings;
}
