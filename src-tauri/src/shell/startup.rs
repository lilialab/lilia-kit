//! 冷启动：托盘、按 settings apply、决定是否显示主窗。

use tauri::{App, Manager};

use super::tray;
use crate::settings::{apply_runtime_from_settings, read_app_settings};
use crate::tools::ToolsState;

/// setup 钩子：读设置 → 托盘 → 对齐运行态 → 非自启则显示主窗。
pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    }

    let app_settings = read_app_settings(app.handle()).unwrap_or_default();
    let tools = app.state::<ToolsState>();

    tray::setup(app.handle())?;

    apply_runtime_from_settings(app.handle(), tools.inner(), &app_settings)
        .map_err(std::io::Error::other)?;

    // 主窗默认 conf 里 visible:false；普通启动再显示，`--startup` 静默留托盘
    #[cfg(desktop)]
    if !super::autostart::launched_from_startup() {
        tray::show_main_window(app.handle());
    }

    Ok(())
}
