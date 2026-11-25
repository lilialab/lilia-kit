//! 工具域 IPC。

use tauri::{AppHandle, State};

use super::monitor;
use super::ToolsState;

/// 前端请求的监视器 OS 显隐（show 可补建窗；hide 仅隐藏）。
#[tauri::command]
pub async fn set_monitor_window_visible(
    app: AppHandle,
    state: State<'_, ToolsState>,
    visible: bool,
) -> Result<(), String> {
    monitor::set_window_visible(&app, &state, visible)
}
