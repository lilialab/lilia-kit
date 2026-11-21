//! 工具运行态：启用位、按 settings apply、各工具窗生命周期。

mod monitor;

use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::commands::settings::{self, AppSettings};
use crate::window_labels;

pub use monitor::on_destroyed as on_monitor_destroyed;

/// 工具 id（与前端 `ToolId` / settings 分组键对齐）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ToolId {
    Monitor,
    Screenshot,
    Clipboard,
}

/// 各工具启用位（与 settings 各工具块 `enabled` 对齐）。
pub struct ToolsState {
    monitor_enabled: AtomicBool,
    /// 串行化监视器开/关，避免快切时销毁未完成又建窗。
    monitor_lifecycle: std::sync::Mutex<()>,
}

impl ToolsState {
    pub fn new(monitor_enabled: bool) -> Self {
        Self {
            monitor_enabled: AtomicBool::new(monitor_enabled),
            monitor_lifecycle: std::sync::Mutex::new(()),
        }
    }

    pub fn set_enabled_flag(&self, tool: ToolId, enabled: bool) {
        match tool {
            ToolId::Monitor => self.set_monitor_enabled(enabled),
            ToolId::Screenshot | ToolId::Clipboard => {}
        }
    }

    pub(crate) fn monitor_enabled(&self) -> bool {
        self.monitor_enabled.load(Ordering::SeqCst)
    }

    pub(crate) fn set_monitor_enabled(&self, enabled: bool) {
        self.monitor_enabled.store(enabled, Ordering::SeqCst);
    }
}

/// 前端请求监视窗 OS 显隐（热键不直接调此路径）。
#[tauri::command]
pub async fn set_monitor_window_visible(
    app: AppHandle,
    state: State<'_, ToolsState>,
    visible: bool,
) -> Result<(), String> {
    monitor::set_window_visible(&app, &state, visible)
}

/// 保存应用设置：按 settings apply 运行态后写盘，并回推完整文档。
///
/// `async`：避免在同步 command 里建窗与主事件循环死锁。
/// （放在 tools：需编排窗口副作用；纯读写仍在 `commands::settings`。）
#[tauri::command]
pub async fn save_app_settings(
    app: AppHandle,
    state: State<'_, ToolsState>,
    settings: AppSettings,
) -> Result<AppSettings, String> {
    apply_app_settings(&app, &state, &settings)?;
    Ok(settings)
}

/// 让运行态对齐一份完整 settings：启用 / 窗口偏好 → 写盘 → 通知。
pub(crate) fn apply_app_settings<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    settings: &AppSettings,
) -> Result<(), String> {
    if settings.monitor.enabled != state.monitor_enabled() {
        monitor::set_enabled(app, state, settings.monitor.enabled)?;
    }

    if let Some(window) = app.get_webview_window(window_labels::MONITOR) {
        window
            .set_always_on_top(settings.monitor.pinned)
            .map_err(|err| err.to_string())?;
    }

    settings::write_app_settings(app, settings)?;
    settings::emit_app_settings_changed(app, settings);
    Ok(())
}

/// 启动时打开已启用的工具窗。
pub fn open_enabled_tools<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
) -> Result<(), String> {
    if state.monitor_enabled() {
        monitor::set_enabled(app, state, true)?;
    }
    Ok(())
}

/// 快捷键入口：只向监视窗 emit toggle。
pub fn emit_monitor_toggle<R: Runtime>(app: &AppHandle<R>) {
    monitor::emit_toggle(app);
}
