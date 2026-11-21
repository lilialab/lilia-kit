//! 全局快捷键注册处（监视器显隐；截图等后续在此追加）。

use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::tools;

/// 监视器显隐：Win+Alt+. / ⌘+⌥+.（仅已启用时生效）
const MONITOR_VISIBILITY_ACCEL: &str = "Super+Alt+Period";

/// 注册桌面全局快捷键；占用失败只告警，不阻断启动。
pub fn setup<R: Runtime>(app: &AppHandle<R>) {
    let monitor_visibility: Shortcut = match MONITOR_VISIBILITY_ACCEL.parse() {
        Ok(shortcut) => shortcut,
        Err(err) => {
            eprintln!("skip shortcut {MONITOR_VISIBILITY_ACCEL}: invalid ({err})");
            return;
        }
    };

    if let Err(err) = app.global_shortcut().on_shortcut(
        monitor_visibility,
        |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                tools::toggle_monitor_visibility(app);
            }
        },
    ) {
        eprintln!("skip shortcut {MONITOR_VISIBILITY_ACCEL}: {err}");
    }
}
