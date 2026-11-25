//! LiliaKit 桌面端：命令注册、托盘与多窗生命周期。

mod metrics;
mod settings;
mod shell;
mod tools;

use metrics::{MetricsState, get_system_metrics};
use settings::{
    default_app_settings, get_resolved_app_env, load_app_settings, save_app_settings,
};
use tools::{ToolsState, set_monitor_window_visible};

/// 启动 Tauri：托盘、设置、工具窗与全局快捷键。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = shell::plugins::register_desktop(builder);
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(shell::plugins::prevent_default())
        .manage(MetricsState::new())
        // 冷启动未对齐；setup 里按 settings apply 后再开窗
        .manage(ToolsState::new(false))
        .invoke_handler(tauri::generate_handler![
            get_system_metrics,
            load_app_settings,
            save_app_settings,
            default_app_settings,
            set_monitor_window_visible,
            get_resolved_app_env,
        ])
        .on_window_event(shell::window_events::on_window_event)
        .setup(shell::startup::setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
