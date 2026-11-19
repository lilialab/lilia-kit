//! LiliaKit 桌面端：命令注册、托盘与多窗生命周期。

mod metrics;
mod tools;
mod tray;
mod window_chrome;

use metrics::{MetricsState, get_system_metrics};
use tauri::Manager;
use tools::{
    MAIN_LABEL, MONITOR_LABEL, ToolsState, get_tools_state, open_enabled_tools, set_monitor_enabled,
};

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            tray::show_main_window(app);
        }));
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default_plugin())
        .manage(MetricsState::new())
        // 默认打开监视器，保留原先「一开就有浮窗」的体验
        .manage(ToolsState::new(true))
        .invoke_handler(tauri::generate_handler![
            get_system_metrics,
            get_tools_state,
            set_monitor_enabled,
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if window.label() == MAIN_LABEL {
                    // 主窗 X → 进托盘；真正退出走托盘「退出」
                    api.prevent_close();
                    let _ = window.hide();
                }
                // 工具窗允许关闭（销毁）；开关由 Destroyed 回写
            }
            tauri::WindowEvent::Destroyed => {
                if window.label() == MONITOR_LABEL {
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

            tray::setup(app.handle())?;
            let tools = app.state::<ToolsState>();
            open_enabled_tools(app.handle(), tools.inner())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
