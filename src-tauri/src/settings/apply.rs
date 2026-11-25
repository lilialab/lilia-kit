//! 已保存 settings 的统一 apply：试运行态 → 写盘 → 分发 → 通知。

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

/// 对齐一份完整 settings：热键等可失败步骤先试，成功后再落盘与分发。
pub fn apply_app_settings<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    settings: &AppSettings,
) -> Result<(), String> {
    #[cfg(desktop)]
    crate::shell::hotkeys::apply_from_settings(
        app,
        settings,
        crate::shell::hotkeys::ApplyMode::Strict,
    )?;

    #[cfg(desktop)]
    crate::shell::autostart::apply_launch_at_login(
        app,
        settings.general.launch_at_login,
        crate::shell::autostart::ApplyMode::Strict,
    )?;

    persist::write_app_settings(app, settings)?;
    apply_runtime_from_settings(app, state, settings)?;
    persist::emit_app_settings_changed(app, settings);
    Ok(())
}

/// 仅对齐运行态（不写盘、不 emit settings）：启动与保存共用。
///
/// 保存路径里热键已在写盘前 Strict 对齐，此处桌面热键用 Soft 以幂等跳过。
pub fn apply_runtime_from_settings<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    settings: &AppSettings,
) -> Result<(), String> {
    tools::apply_from_settings(app, state, settings)?;

    #[cfg(desktop)]
    crate::shell::hotkeys::apply_from_settings(
        app,
        settings,
        crate::shell::hotkeys::ApplyMode::Soft,
    )?;

    #[cfg(desktop)]
    crate::shell::autostart::apply_launch_at_login(
        app,
        settings.general.launch_at_login,
        crate::shell::autostart::ApplyMode::Soft,
    )?;

    appearance::apply_from_settings(app, settings);
    Ok(())
}
