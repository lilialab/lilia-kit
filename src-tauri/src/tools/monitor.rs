//! 监视器浮窗：开/关、显隐、销毁回写。

use std::time::Duration;

use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};

use crate::commands::settings;
use crate::tools::ToolsState;
use crate::window_chrome;
use crate::window_labels;

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

/// 已启用时切换显隐（不改启用状态）。
pub fn toggle_visibility<R: Runtime>(app: &AppHandle<R>) {
    let state = app.state::<ToolsState>();
    if !state.monitor_enabled() {
        return;
    }

    let Ok(_guard) = state.monitor_lifecycle.lock() else {
        return;
    };

    let Some(window) = app.get_webview_window(window_labels::MONITOR) else {
        let _ = open(app);
        return;
    };

    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        Ok(false) => {
            let _ = window.unminimize();
            let _ = window.show();
        }
        Err(_) => {}
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
    let _ = crate::tools::apply_app_settings(app, state, &next);
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

    let window = WebviewWindowBuilder::new(
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
        .drag_and_drop(false)
        .no_redirection_bitmap(true)
        .visible(true)
        .build()?;

    window_chrome::apply_to(&window);
    Ok(())
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
