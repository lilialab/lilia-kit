/** 工具 id（与 Rust `ToolId` / settings 分组键对齐） */
export type ToolId = 'monitor' | 'screenshot' | 'clipboard';

/** 监视器工具偏好 */
export interface MonitorSettings {
  /** 是否启用（保存后装载/卸载浮窗） */
  enabled: boolean;
  /** 是否置顶 */
  pinned: boolean;
  /** 是否显示温度 */
  showTemperature: boolean;
}

/** 应用偏好设置（与 Rust `AppSettings` / settings.json 对齐，按工具分组） */
export interface AppSettings {
  monitor: MonitorSettings;
}
