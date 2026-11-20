//! 全局快捷键注册处（监视器开/关；截图等后续在此追加）。

use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::tools;

/// 监视器开/关：Win+Alt+M / ⌘+⌥+M
const MONITOR_TOGGLE_ACCEL: &str = "Super+Alt+M";

/// 注册桌面全局快捷键；占用失败只告警，不阻断启动。
pub fn setup<R: Runtime>(app: &AppHandle<R>) {
    let monitor_toggle: Shortcut = match MONITOR_TOGGLE_ACCEL.parse() {
        Ok(shortcut) => shortcut,
        Err(err) => {
            eprintln!("skip shortcut {MONITOR_TOGGLE_ACCEL}: invalid ({err})");
            return;
        }
    };

    if let Err(err) = app.global_shortcut().on_shortcut(
        monitor_toggle,
        |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                tools::toggle_monitor_enabled(app);
            }
        },
    ) {
        eprintln!("skip shortcut {MONITOR_TOGGLE_ACCEL}: {err}");
    }
}
