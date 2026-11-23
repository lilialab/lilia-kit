//! 设置文档类型（与前端 `AppSettings` 对齐）。

use serde::{Deserialize, Serialize};

/// 外观偏好：跟随系统，或固定 Catppuccin 主题名。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppearancePreference {
    System,
    Latte,
    Mocha,
}

/// 通用设置分组（外观等；非工具运行态）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralSettings {
    pub appearance: AppearancePreference,
}

/// 监视器工具偏好。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorSettings {
    /// 是否启用（保存后装载/卸载浮窗）
    pub enabled: bool,
    /// 是否置顶（开窗时应用；设置页可改）
    pub pinned: bool,
    /// 是否显示温度
    pub show_temperature: bool,
}

/// 应用偏好设置（按分组：通用 + 各工具）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub monitor: MonitorSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        serde_json::from_str(include_str!("default_settings.json"))
            .expect("default_settings.json must deserialize to AppSettings")
    }
}
