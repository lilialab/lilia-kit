//! 监视器浮窗：开/关、显隐执行、销毁回写。
//!
//! 快捷键只发 `monitor-toggle`；显隐节奏由前端决定后再 invoke。

use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, Runtime, State, WebviewUrl, WebviewWindowBuilder};

use crate::settings;
use crate::settings::model::MonitorSettings;
use crate::shell::{window_chrome, window_labels};
use crate::tools::ToolsState;

const EVENT_MONITOR_TOGGLE: &str = "monitor-toggle";

/// 按已保存的监视器设置对齐运行态（启用 / 置顶）。
pub fn apply_from_settings<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    monitor: &MonitorSettings,
) -> Result<(), String> {
    if monitor.enabled != state.monitor_enabled() {
        set_enabled(app, state, monitor.enabled)?;
    }

    if let Some(window) = app.get_webview_window(window_labels::MONITOR) {
        window
            .set_always_on_top(monitor.pinned)
            .map_err(|err| err.to_string())?;
    }

    Ok(())
}

/// 按启用状态装载或卸载浮窗（持有 lifecycle 锁）。
pub fn set_enabled<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    enabled: bool,
) -> Result<(), String> {
    let _guard = state
        .monitor_lifecycle
        .lock()
        .map_err(|_| "tools state poisoned".to_string())?;

    state.set_monitor_enabled(enabled);

    let result = if enabled {
        open(app).map_err(|err| err.to_string())
    } else {
        destroy(app)
    };

    if result.is_err() {
        state.set_monitor_enabled(!enabled);
    }
    result
}

/// 快捷键：仅向监视窗发 toggle 事件（未启用或无窗则忽略）。
pub fn emit_toggle<R: Runtime>(app: &AppHandle<R>) {
    let state = app.state::<ToolsState>();
    if !state.monitor_enabled() {
        return;
    }
    let Some(window) = app.get_webview_window(window_labels::MONITOR) else {
        return;
    };
    let _ = window.emit(EVENT_MONITOR_TOGGLE, ());
}

/// 前端请求的 OS 显隐（show 可补建窗；hide 仅隐藏）。
#[tauri::command]
pub async fn set_monitor_window_visible(
    app: AppHandle,
    state: State<'_, ToolsState>,
    visible: bool,
) -> Result<(), String> {
    set_window_visible(&app, &state, visible)
}

fn set_window_visible<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    visible: bool,
) -> Result<(), String> {
    if !state.monitor_enabled() {
        return Ok(());
    }

    let _guard = state
        .monitor_lifecycle
        .lock()
        .map_err(|_| "tools state poisoned".to_string())?;

    if visible {
        open(app).map_err(|err| err.to_string())
    } else {
        hide(app)
    }
}

/// 窗已销毁且仍标记启用时：拨回 false 并写盘通知。
///
/// 开/关流程持有锁时忽略，避免快切时旧窗 Destroyed 误伤新窗。
pub fn on_destroyed<R: Runtime>(app: &AppHandle<R>, state: &ToolsState) {
    {
        let Ok(_guard) = state.monitor_lifecycle.try_lock() else {
            return;
        };
        if !state.monitor_enabled() || app.get_webview_window(window_labels::MONITOR).is_some() {
            return;
        }
        state.set_monitor_enabled(false);
    }

    let Ok(mut next) = settings::read_app_settings(app) else {
        return;
    };
    if !next.monitor.enabled {
        return;
    }
    next.monitor.enabled = false;
    let _ = crate::settings::apply_app_settings(app, state, &next);
}

fn open<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(existing) = app.get_webview_window(window_labels::MONITOR) {
        let _ = existing.unminimize();
        match existing.show() {
            Ok(()) => return Ok(()),
            Err(_) => {
                let _ = existing.destroy();
                wait_gone(app);
            }
        }
    }

    let pinned = settings::read_app_settings(app)
        .map(|s| s.monitor.pinned)
        .unwrap_or(true);

    let mut builder = WebviewWindowBuilder::new(
        app,
        window_labels::MONITOR,
        WebviewUrl::App("index.html".into()),
    )
    .title("监视器")
    .inner_size(220.0, 78.0)
    .resizable(false)
    .maximizable(false)
    .decorations(false)
    .transparent(true)
    .always_on_top(pinned)
    .skip_taskbar(true)
    .shadow(false)
    .visible(true);

    #[cfg(windows)]
    {
        builder = builder.drag_and_drop(false).no_redirection_bitmap(true);
    }

    let window = builder.build()?;

    window_chrome::apply_to(&window);
    Ok(())
}

fn hide<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let Some(window) = app.get_webview_window(window_labels::MONITOR) else {
        return Ok(());
    };
    window.hide().map_err(|err| err.to_string())
}

fn destroy<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let Some(window) = app.get_webview_window(window_labels::MONITOR) else {
        return Ok(());
    };

    match window.destroy() {
        Ok(()) => {}
        Err(err) => {
            let msg = err.to_string();
            // 快切时窗可能已在拆
            if !msg.to_ascii_lowercase().contains("webview reference") {
                return Err(msg);
            }
        }
    }

    wait_gone(app);
    Ok(())
}

fn wait_gone<R: Runtime>(app: &AppHandle<R>) {
    for _ in 0..50 {
        if app.get_webview_window(window_labels::MONITOR).is_none() {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
