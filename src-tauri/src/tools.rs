//! 工具运行态：共享状态 + 各工具子模块；根文件只组装。

pub(crate) mod monitor;
mod state;

use tauri::{AppHandle, Runtime};

use crate::settings::model::AppSettings;

pub use state::ToolsState;

/// 按已保存 settings 对齐全部工具运行态（启用窗等）。
pub fn apply_from_settings<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    settings: &AppSettings,
) -> Result<(), String> {
    monitor::apply_from_settings(app, state, &settings.monitor)?;
    Ok(())
}
