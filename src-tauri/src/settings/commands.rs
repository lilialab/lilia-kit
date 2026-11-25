//! 应用设置 / 壳层 env IPC。

use tauri::{AppHandle, State};

use super::apply;
use super::env::{self, ResolvedAppEnv};
use super::model::AppSettings;
use super::persist;
use crate::tools::ToolsState;

/// 读取应用设置（给前端）。
#[tauri::command]
pub fn load_app_settings(app: AppHandle) -> Result<AppSettings, String> {
    persist::read_app_settings(&app)
}

/// 保存应用设置：落盘并 apply 运行态，回推完整文档。
///
/// `async`：避免在同步 command 里建窗与主事件循环死锁。
#[tauri::command]
pub async fn save_app_settings(
    app: AppHandle,
    state: State<'_, ToolsState>,
    settings: AppSettings,
) -> Result<AppSettings, String> {
    apply::apply_app_settings(&app, &state, &settings)?;
    Ok(settings)
}

/// 内置默认设置，不读写磁盘。
#[tauri::command]
pub fn default_app_settings() -> AppSettings {
    AppSettings::default()
}

/// 启动对齐：当前已保存偏好下的 resolved 快照。
#[tauri::command]
pub fn get_resolved_app_env(app: AppHandle) -> Result<ResolvedAppEnv, String> {
    let settings = persist::read_app_settings(&app)?;
    Ok(env::resolve_from_settings(&app, &settings))
}
