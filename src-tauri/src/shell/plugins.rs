//! Builder 插件与 WebView 默认快捷键屏蔽。

/// 屏蔽 WebView 浏览器默认快捷键（Ctrl+P 等在 Win 上不会进到 JS）。
pub fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
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

/// 桌面：单实例、全局快捷键、自启、窗位记忆、热键状态。
#[cfg(desktop)]
pub fn register_desktop(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    use tauri_plugin_window_state::StateFlags;

    use super::autostart;
    use super::hotkeys::HotkeysState;
    use super::tray;

    builder
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            tray::show_main_window(app);
        }))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .args([autostart::STARTUP_ARG])
                .build(),
        )
        .plugin(
            tauri_plugin_window_state::Builder::new()
                // 只记位置/尺寸；可见性由工具开关管，避免和主窗托盘显隐打架
                .with_state_flags(StateFlags::POSITION | StateFlags::SIZE)
                .build(),
        )
        .manage(HotkeysState::new())
}
