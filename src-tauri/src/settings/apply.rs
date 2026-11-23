//! 已保存 settings 的统一 apply：写盘 → 按域分发 → 通知。

use tauri::{AppHandle, Runtime, State};

use super::appearance;
use super::model::AppSettings;
use super::persist;
use crate::tools::{self, ToolsState};

/// 保存应用设置：落盘并 apply 运行态，回推完整文档。
///
/// `async`：避免在同步 command 里建窗与主事件循环死锁。
#[tauri::command]
pub async fn save_app_settings(
    app: AppHandle,
    state: State<'_, ToolsState>,
    settings: AppSettings,
) -> Result<AppSettings, String> {
    apply_app_settings(&app, &state, &settings)?;
    Ok(settings)
}

/// 对齐一份完整 settings：先持久化，再分发各域，最后通知前端。
pub fn apply_app_settings<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    settings: &AppSettings,
) -> Result<(), String> {
    persist::write_app_settings(app, settings)?;
    apply_runtime_from_settings(app, state, settings)?;
    persist::emit_app_settings_changed(app, settings);
    Ok(())
}

/// 仅对齐运行态（不写盘、不 emit settings）：启动与保存共用。
pub fn apply_runtime_from_settings<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    settings: &AppSettings,
) -> Result<(), String> {
    tools::apply_from_settings(app, state, settings)?;
    appearance::apply_from_settings(app, settings);
    Ok(())
}
