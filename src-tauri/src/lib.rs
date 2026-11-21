//! LiliaKit 桌面端：命令注册、托盘与多窗生命周期。

mod commands;
mod hotkeys;
mod metrics;
mod tools;
mod tray;
mod window_chrome;
mod window_labels;

use commands::metrics::get_system_metrics;
use commands::settings::{default_app_settings, load_app_settings, read_app_settings};
use metrics::MetricsState;
use tauri::Manager;
use tools::{
    ToolId, ToolsState, open_enabled_tools, save_app_settings, set_monitor_window_visible,
};
use window_labels::{MAIN, MONITOR};

/// 屏蔽 WebView 浏览器默认快捷键（Ctrl+P 等在 Win 上不会进到 JS）。
fn prevent_default_plugin() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    #[cfg(debug_assertions)]
    let flags = Flags::keyboard().difference(Flags::DEV_TOOLS);
    #[cfg(not(debug_assertions))]
    let flags = Flags::keyboard();

    let builder = tauri_plugin_prevent_default::Builder::new().with_flags(flags);

    #[cfg(windows)]
    let builder = builder.platform(
        tauri_plugin_prevent_default::PlatformOptions::new().browser_accelerator_keys(false),
    );

    builder.build()
}

/// 启动 Tauri：托盘、设置、工具窗与全局快捷键。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        use tauri_plugin_window_state::StateFlags;

        builder = builder
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                tray::show_main_window(app);
            }))
            .plugin(tauri_plugin_global_shortcut::Builder::new().build())
            .plugin(
                tauri_plugin_window_state::Builder::new()
                    // 只记位置/尺寸；可见性由工具开关管，避免和主窗托盘显隐打架
                    .with_state_flags(StateFlags::POSITION | StateFlags::SIZE)
                    .build(),
            );
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default_plugin())
        .manage(MetricsState::new())
        // 默认打开监视器，保留原先「一开就有浮窗」的体验
        .manage(ToolsState::new(true))
        .invoke_handler(tauri::generate_handler![
            get_system_metrics,
            load_app_settings,
            save_app_settings,
            default_app_settings,
            set_monitor_window_visible,
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if window.label() == MAIN {
                    // 主窗 X → 进托盘；真正退出走托盘「退出」
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
            tauri::WindowEvent::Destroyed => {
                if window.label() == MONITOR {
                    let app = window.app_handle().clone();
                    if let Some(state) = app.try_state::<ToolsState>() {
                        tools::on_monitor_destroyed(&app, state.inner());
                    }
                }
            }
            _ => {}
        })
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            let app_settings = read_app_settings(app.handle()).unwrap_or_default();
            let tools = app.state::<ToolsState>();
            tools.set_enabled_flag(ToolId::Monitor, app_settings.monitor.enabled);

            tray::setup(app.handle())?;
            open_enabled_tools(app.handle(), tools.inner())
                .map_err(std::io::Error::other)?;

            #[cfg(desktop)]
            hotkeys::setup(app.handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
