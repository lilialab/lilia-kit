//! 工具开关与工具窗生命周期（监视器等）。

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime, State, WebviewUrl, WebviewWindowBuilder};

pub const MAIN_LABEL: &str = "main";
pub const MONITOR_LABEL: &str = "monitor";

const EVENT_TOOLS_CHANGED: &str = "tools-changed";

/// 各工具开关（主窗为真源；工具窗销毁时回写）。
pub struct ToolsState {
    monitor_enabled: std::sync::Mutex<bool>,
}

impl ToolsState {
    pub fn new(monitor_enabled: bool) -> Self {
        Self {
            monitor_enabled: std::sync::Mutex::new(monitor_enabled),
        }
    }

    fn snapshot(&self) -> ToolsStateDto {
        ToolsStateDto {
            monitor_enabled: *self.monitor_enabled.lock().unwrap_or_else(|e| e.into_inner()),
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
    {
        let mut flag = state
            .monitor_enabled
            .lock()
            .map_err(|_| "tools state poisoned".to_string())?;
        *flag = enabled;
    }

    let result = if enabled {
        open_monitor(&app).map_err(|err| err.to_string())
    } else {
        destroy_monitor(&app)
    };

    if let Err(err) = result {
        let mut flag = state
            .monitor_enabled
            .lock()
            .map_err(|_| "tools state poisoned".to_string())?;
        *flag = !enabled;
        emit_tools_changed(&app, &state);
        return Err(err);
    }

    emit_tools_changed(&app, &state);
    Ok(())
}

/// 启动时按默认开关打开已启用的工具窗。
pub fn open_enabled_tools<R: Runtime>(app: &AppHandle<R>, state: &ToolsState) -> tauri::Result<()> {
    let enabled = *state.monitor_enabled.lock().unwrap_or_else(|e| e.into_inner());
    if enabled {
        open_monitor(app)?;
    }
    Ok(())
}

/// 监视窗被销毁时：若开关仍为开（意外关闭），拨回并通知主窗。
pub fn on_monitor_destroyed<R: Runtime>(app: &AppHandle<R>, state: &ToolsState) {
    let enabled = *state.monitor_enabled.lock().unwrap_or_else(|e| e.into_inner());
    if !enabled {
        return;
    }

    if let Ok(mut flag) = state.monitor_enabled.lock() {
        *flag = false;
    }
    emit_tools_changed(app, state);
}

fn emit_tools_changed<R: Runtime>(app: &AppHandle<R>, state: &ToolsState) {
    let _ = app.emit(EVENT_TOOLS_CHANGED, state.snapshot());
}

fn open_monitor<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(existing) = app.get_webview_window(MONITOR_LABEL) {
        let _ = existing.unminimize();
        existing.show()?;
        return Ok(());
    }

    let window = WebviewWindowBuilder::new(app, MONITOR_LABEL, WebviewUrl::App("index.html".into()))
    .title("监视器")
    .inner_size(340.0, 420.0)
    .resizable(true)
    .decorations(false)
    .transparent(true)
    .always_on_top(false)
    .skip_taskbar(true)
    .shadow(false)
    .drag_and_drop(false)
    .no_redirection_bitmap(true)
    .visible(true)
    .build()?;

    crate::window_chrome::apply_to(&window);
    Ok(())
}

fn destroy_monitor(app: &AppHandle) -> Result<(), String> {
    let Some(window) = app.get_webview_window(MONITOR_LABEL) else {
        return Ok(());
    };
    window.destroy().map_err(|err| err.to_string())
}
