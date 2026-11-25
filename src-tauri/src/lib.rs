//! LiliaKit 桌面端：命令注册、托盘与多窗生命周期。

mod metrics;
mod settings;
mod shell;
mod tools;

use metrics::{MetricsState, get_system_metrics};
use settings::{
    apply_runtime_from_settings, default_app_settings, get_resolved_app_env, load_app_settings,
    on_os_theme_changed, read_app_settings, save_app_settings,
};
use shell::window_labels::{MAIN, MONITOR};
use tauri::Manager;
use tools::monitor::set_monitor_window_visible;
use tools::ToolsState;

#[cfg(desktop)]
use shell::hotkeys::HotkeysState;

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
                shell::tray::show_main_window(app);
            }))
            .plugin(tauri_plugin_global_shortcut::Builder::new().build())
            .plugin(
                tauri_plugin_autostart::Builder::new()
                    .args([shell::autostart::STARTUP_ARG])
                    .build(),
            )
            .plugin(
                tauri_plugin_window_state::Builder::new()
                    // 只记位置/尺寸；可见性由工具开关管，避免和主窗托盘显隐打架
                    .with_state_flags(StateFlags::POSITION | StateFlags::SIZE)
                    .build(),
            )
            .manage(HotkeysState::new());
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default_plugin())
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
                        tools::monitor::on_destroyed(&app, state.inner());
                    }
                }
            }
            // 只跟主窗，避免多窗重复 emit；窗 theme 为 None 时才会收到
            tauri::WindowEvent::ThemeChanged(theme) => {
                if window.label() == MAIN {
                    on_os_theme_changed(window.app_handle(), *theme);
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

            shell::tray::setup(app.handle())?;

            apply_runtime_from_settings(app.handle(), tools.inner(), &app_settings)
                .map_err(std::io::Error::other)?;

            // 主窗默认 conf 里 visible:false；普通启动再显示，`--startup` 静默留托盘
            #[cfg(desktop)]
            if !shell::autostart::launched_from_startup() {
                shell::tray::show_main_window(app.handle());
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
