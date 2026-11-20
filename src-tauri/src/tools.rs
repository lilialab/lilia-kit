//! 工具开关与工具窗生命周期（监视器等）。

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime, State, WebviewUrl, WebviewWindowBuilder};

use crate::settings;

pub const MAIN_LABEL: &str = "main";
pub const MONITOR_LABEL: &str = "monitor";

const EVENT_TOOLS_CHANGED: &str = "tools-changed";

/// 各工具开关（主窗为真源；工具窗销毁时回写）。
pub struct ToolsState {
    monitor_enabled: AtomicBool,
    /// 串行化开/关，避免快切时销毁未完成又建窗。
    monitor_lifecycle: std::sync::Mutex<()>,
}

impl ToolsState {
    pub fn new(monitor_enabled: bool) -> Self {
        Self {
            monitor_enabled: AtomicBool::new(monitor_enabled),
            monitor_lifecycle: std::sync::Mutex::new(()),
        }
    }

    fn snapshot(&self) -> ToolsStateDto {
        ToolsStateDto {
            monitor_enabled: self.monitor_enabled.load(Ordering::SeqCst),
            screenshot_available: false,
            clipboard_available: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsStateDto {
    pub monitor_enabled: bool,
    /// 截图尚未实现；前端作占位
    pub screenshot_available: bool,
    /// 剪切板历史尚未实现；前端作占位
    pub clipboard_available: bool,
}

/// 读取工具开关快照。
#[tauri::command]
pub fn get_tools_state(state: State<'_, ToolsState>) -> ToolsStateDto {
    state.snapshot()
}

/// 开关监视器浮窗。
///
/// `async`：避免在同步 command 里 `WebviewWindowBuilder::build` 与主事件循环死锁。
#[tauri::command]
pub async fn set_monitor_enabled(
    app: AppHandle,
    state: State<'_, ToolsState>,
    enabled: bool,
) -> Result<(), String> {
    apply_monitor_enabled(&app, &state, enabled)
}

/// 快捷键等：翻转监视器开/关。
pub fn toggle_monitor_enabled<R: Runtime>(app: &AppHandle<R>) {
    let state = app.state::<ToolsState>();
    let current = state.monitor_enabled.load(Ordering::SeqCst);
    let _ = apply_monitor_enabled(app, &state, !current);
}

fn apply_monitor_enabled<R: Runtime>(
    app: &AppHandle<R>,
    state: &ToolsState,
    enabled: bool,
) -> Result<(), String> {
    let _lifecycle = state
        .monitor_lifecycle
        .lock()
        .map_err(|_| "tools state poisoned".to_string())?;

    state
        .monitor_enabled
        .store(enabled, Ordering::SeqCst);

    let result = if enabled {
        open_monitor(app).map_err(|err| err.to_string())
    } else {
        destroy_monitor(app)
    };

    if let Err(err) = result {
        state
            .monitor_enabled
            .store(!enabled, Ordering::SeqCst);
        emit_tools_changed(app, state);
        return Err(err);
    }

    emit_tools_changed(app, state);
    Ok(())
}

/// 启动时按默认开关打开已启用的工具窗。
pub fn open_enabled_tools<R: Runtime>(app: &AppHandle<R>, state: &ToolsState) -> tauri::Result<()> {
    if state.monitor_enabled.load(Ordering::SeqCst) {
        open_monitor(app)?;
    }
    Ok(())
}

/// 监视窗被销毁时：若开关仍为开且当前无窗（意外关闭），拨回并通知主窗。
///
/// 开/关流程持有 `monitor_lifecycle` 时直接忽略，避免快切时旧窗 Destroyed 误关新窗。
pub fn on_monitor_destroyed<R: Runtime>(app: &AppHandle<R>, state: &ToolsState) {
    let Ok(_lifecycle) = state.monitor_lifecycle.try_lock() else {
        return;
    };

    if !state.monitor_enabled.load(Ordering::SeqCst)
        || app.get_webview_window(MONITOR_LABEL).is_some()
    {
        return;
    }

    state.monitor_enabled.store(false, Ordering::SeqCst);
    emit_tools_changed(app, state);
}

fn emit_tools_changed<R: Runtime>(app: &AppHandle<R>, state: &ToolsState) {
    let _ = app.emit(EVENT_TOOLS_CHANGED, state.snapshot());
}

fn open_monitor<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(existing) = app.get_webview_window(MONITOR_LABEL) {
        let _ = existing.unminimize();
        match existing.show() {
            Ok(()) => return Ok(()),
            Err(_) => {
                let _ = existing.destroy();
                wait_monitor_gone(app);
            }
        }
    }

    let pinned = settings::read_app_settings(app)
        .map(|s| s.monitor_pinned)
        .unwrap_or(true);

    let window =
        WebviewWindowBuilder::new(app, MONITOR_LABEL, WebviewUrl::App("index.html".into()))
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

    crate::window_chrome::apply_to(&window);
    Ok(())
}

fn destroy_monitor<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let Some(window) = app.get_webview_window(MONITOR_LABEL) else {
        return Ok(());
    };

    match window.destroy() {
        Ok(()) => {}
        Err(err) => {
            let msg = err.to_string();
            // 快切时窗已在拆，视为成功
            if !msg.to_ascii_lowercase().contains("webview reference") {
                return Err(msg);
            }
        }
    }

    wait_monitor_gone(app);
    Ok(())
}

fn wait_monitor_gone<R: Runtime>(app: &AppHandle<R>) {
    for _ in 0..50 {
        if app.get_webview_window(MONITOR_LABEL).is_none() {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
